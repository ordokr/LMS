# Group

## Description

frozen_string_literal: true
require "net/imap"

## Relationships

- has_many :category_groups
- has_many :category_moderation_groups
- has_many :group_users
- has_many :group_requests
- has_many :group_mentions
- has_many :group_associated_groups
- has_many :group_archived_messages
- has_many :categories
- has_many :moderation_categories
- has_many :users
- has_many :human_users
- has_many :requesters
- has_many :group_histories
- has_many :group_category_notification_defaults
- has_many :group_tag_notification_defaults
- has_many :associated_groups
- belongs_to :flair_upload
- has_many :upload_references
- belongs_to :smtp_updated_by
- belongs_to :imap_updated_by

## Methods

### expire_cache

frozen_string_literal: true

require "net/imap"

class Group < ActiveRecord::Base
  # Maximum 255 characters including terminator.
  # https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4
  MAX_EMAIL_DOMAIN_LENGTH = 253

  # TODO: Remove flair_url when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
  # TODO: Remove smtp_ssl when db/post_migrate/20240717053710_drop_groups_smtp_ssl has been promoted to pre-deploy
  self.ignored_columns = %w[flair_url smtp_ssl]

  include HasCustomFields
  include AnonCacheInvalidator
  include HasDestroyedWebHook
  include GlobalPath

  cattr_accessor :preloaded_custom_field_names
  self.preloaded_custom_field_names = Set.new

  has_many :category_groups, dependent: :destroy
  has_many :category_moderation_groups, dependent: :destroy
  has_many :group_users, dependent: :destroy
  has_many :group_requests, dependent: :destroy
  has_many :group_mentions, dependent: :destroy
  has_many :group_associated_groups, dependent: :destroy

  has_many :group_archived_messages, dependent: :destroy

  has_many :categories, through: :category_groups
  has_many :moderation_categories, through: :category_moderation_groups, source: :category
  has_many :users, through: :group_users
  has_many :human_users, -> { human_users }, through: :group_users, source: :user
  has_many :requesters, through: :group_requests, source: :user
  has_many :group_histories, dependent: :destroy
  has_many :group_category_notification_defaults, dependent: :destroy
  has_many :group_tag_notification_defaults, dependent: :destroy
  has_many :associated_groups, through: :group_associated_groups, dependent: :destroy

  belongs_to :flair_upload, class_name: "Upload"
  has_many :upload_references, as: :target, dependent: :destroy

  belongs_to :smtp_updated_by, class_name: "User"
  belongs_to :imap_updated_by, class_name: "User"

  has_and_belongs_to_many :web_hooks

  before_save :downcase_incoming_email
  before_save :cook_bio

  after_save :destroy_deletions
  after_save :update_primary_group
  after_save :update_title

  after_save :enqueue_update_mentions_job,
             if: Proc.new { |g| g.name_before_last_save && g.saved_change_to_name? }

  after_save do
    if saved_change_to_flair_upload_id?
      UploadReference.ensure_exist!(upload_ids: [self.flair_upload_id], target: self)
    end
  end

  after_save :expire_cache
  after_destroy :expire_cache

  after_commit :automatic_group_membership, on: %i[create update]
  after_commit :trigger_group_created_event, on: :create
  after_commit :trigger_group_updated_event, on: :update
  before_destroy :cache_group_users_for_destroyed_event, prepend: true
  after_commit :trigger_group_destroyed_event, on: :destroy
  after_commit :set_default_notifications, on: %i[create update]

### self

don't allow shoddy localization to break this
    localized_name = I18n.t("groups.default_names.#{name}", locale: SiteSetting.default_locale)
    default_name = I18n.t("groups.default_names.#{name}")

    group.name =
      if can_use_name?(localized_name, group)
        localized_name
      elsif can_use_name?(default_name, group)
        default_name
      else
        name.to_s
      end

    # the everyone group is special, it can include non-users so there is no
    # way to have the membership in a table
    case name
    when :everyone
      group.visibility_level = Group.visibility_levels[:staff]
      group.save!
      return group
    when :moderators
      group.update!(messageable_level: ALIAS_LEVELS[:everyone])
    end

    if group.visibility_level == Group.visibility_levels[:public]
      group.update!(visibility_level: Group.visibility_levels[:logged_on_users])
    end

    # Remove people from groups they don't belong in.
    remove_subquery =
      case name
      when :admins
        "SELECT id FROM users WHERE NOT admin OR staged"
      when :moderators
        "SELECT id FROM users WHERE NOT moderator OR staged"
      when :staff
        "SELECT id FROM users WHERE (NOT admin AND NOT moderator) OR staged"
      when :trust_level_0, :trust_level_1, :trust_level_2, :trust_level_3, :trust_level_4
        "SELECT id FROM users WHERE trust_level < #{id - 10} OR staged"
      end

    removed_user_ids = DB.query_single <<-SQL
      DELETE FROM group_users
            USING (#{remove_subquery}) X
            WHERE group_id = #{group.id}
              AND user_id = X.id
      RETURNING group_users.user_id
    SQL

    if removed_user_ids.present?
      Jobs.enqueue(
        :publish_group_membership_updates,
        user_ids: removed_user_ids,
        group_id: group.id,
        type: AUTO_GROUPS_REMOVE,
      )
    end

    # Add people to groups
    insert_subquery =
      case name
      when :admins
        "SELECT id FROM users WHERE admin AND NOT staged"
      when :moderators
        "SELECT id FROM users WHERE moderator AND NOT staged"
      when :staff
        "SELECT id FROM users WHERE (moderator OR admin) AND NOT staged"
      when :trust_level_1, :trust_level_2, :trust_level_3, :trust_level_4
        "SELECT id FROM users WHERE trust_level >= #{id - 10} AND NOT staged"
      when :trust_level_0
        "SELECT id FROM users WHERE NOT staged"
      end

    added_user_ids = DB.query_single <<-SQL
      INSERT INTO group_users (group_id, user_id, created_at, updated_at)
           SELECT #{group.id}, X.id, now(), now()
             FROM group_users
       RIGHT JOIN (#{insert_subquery}) X ON X.id = user_id AND group_id = #{group.id}
            WHERE user_id IS NULL
       RETURNING group_users.user_id
    SQL

    group.save!

    if added_user_ids.present?
      Jobs.enqueue(
        :publish_group_membership_updates,
        user_ids: added_user_ids,
        group_id: group.id,
        type: AUTO_GROUPS_ADD,
      )
    end

    # we want to ensure consistency
    Group.reset_user_count(group)

    group
  end

### self

{where_sql}
        GROUP BY group_id
      )
      UPDATE groups
         SET user_count = tally.users
        FROM tally
       WHERE id = tally.group_id
         AND user_count <> tally.users
    SQL
  end
  private_class_method :reset_groups_user_count!

### self

given something that might be a group name, id, or record, return the group id

### self

subtle, using Group[] ensures the group exists in the DB
    Group[group_param.to_sym].id
  end

### bulk_remove

{self.id},
        u.id,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP
      FROM users AS u
      WHERE u.id IN (:user_ids)
      AND NOT EXISTS (
        SELECT 1 FROM group_users AS gu
        WHERE gu.user_id = u.id AND
        gu.group_id = :group_id
      )
      SQL

      DB.exec(sql, group_id: self.id, user_ids: user_ids)

      user_attributes = {}

      user_attributes[:primary_group_id] = self.id if self.primary_group?

      user_attributes[:title] = self.title if self.title.present?

      User.where(id: user_ids).update_all(user_attributes) if user_attributes.present?

      # update group user count
      recalculate_user_count
    end

    if self.grant_trust_level.present?
      Jobs.enqueue(:bulk_grant_trust_level, user_ids: user_ids, trust_level: self.grant_trust_level)
    end

    self
  end

### automatic_membership_email_domains_validator

avoid strip! here, it works now
    # but may not continue to work long term, especially
    # once we start returning frozen strings
    if self.name != (stripped = self.name.unicode_normalize.strip)
      self.name = stripped
    end

    UsernameValidator.perform_validation(self, "name", skip_length_validation: automatic) ||
      begin
        normalized_name = User.normalize_username(self.name)

        if self.will_save_change_to_name? &&
             User.normalize_username(self.name_was) != normalized_name &&
             User.username_exists?(self.name)
          errors.add(:name, I18n.t("activerecord.errors.messages.taken"))
        end
      end
  end

### destroy_deletions

hack around AR

