# User

## Description

frozen_string_literal: true

## Relationships

- has_many :posts
- has_many :topics
- has_many :uploads
- has_many :category_users
- has_many :tag_users
- has_many :user_api_keys
- has_many :topic_allowed_users
- has_many :user_archived_messages
- has_many :email_change_requests
- has_many :email_tokens
- has_many :topic_links
- has_many :user_uploads
- has_many :upload_references
- has_many :user_emails
- has_many :user_associated_accounts
- has_many :oauth2_user_infos
- has_many :user_second_factors
- has_many :user_badges
- has_many :user_auth_tokens
- has_many :group_users
- has_many :user_warnings
- has_many :api_keys
- has_many :push_subscriptions
- has_many :acting_group_histories
- has_many :targeted_group_histories
- has_many :reviewable_scores
- has_many :invites
- has_many :user_custom_fields
- has_many :user_associated_groups
- has_many :pending_posts
- has_one :user_option
- has_one :user_avatar
- has_one :primary_email
- has_one :user_stat
- has_one :user_profile
- has_one :single_sign_on_record
- has_one :anonymous_user_master
- has_one :anonymous_user_shadow
- has_one :invited_user
- has_one :user_notification_schedule
- has_one :user_password
- has_many :bookmarks
- has_many :notifications
- has_many :topic_users
- has_many :incoming_emails
- has_many :user_visits
- has_many :user_auth_token_logs
- has_many :group_requests
- has_many :muted_user_records
- has_many :ignored_user_records
- has_many :do_not_disturb_timings
- has_many :sidebar_sections
- has_one :user_status
- has_many :user_actions
- has_many :post_actions
- has_many :post_timings
- has_many :directory_items
- has_many :email_logs
- has_many :security_keys
- has_many :all_security_keys
- has_many :badges
- has_many :default_featured_user_badges
- has_many :topics_allowed
- has_many :groups
- has_many :secure_categories
- has_many :associated_groups
- has_many :totps
- has_one :master_user
- has_one :shadow_user
- has_one :profile_background_upload
- has_one :card_background_upload
- belongs_to :approved_by
- belongs_to :primary_group
- belongs_to :flair_group
- has_many :muted_users
- has_many :ignored_users
- belongs_to :uploaded_avatar
- has_many :sidebar_section_links
- has_many :embeddable_hosts

## Methods

### self

frozen_string_literal: true

class User < ActiveRecord::Base
  self.ignored_columns = [
    :salt, # TODO: Remove when DropPasswordColumnsFromUsers has been promoted to pre-deploy.
    :password_hash, # TODO: Remove when DropPasswordColumnsFromUsers has been promoted to pre-deploy.
    :password_algorithm, # TODO: Remove when DropPasswordColumnsFromUsers has been promoted to pre-deploy.
    :old_seen_notification_id, # TODO: Remove once 20240829140226_drop_old_notification_id_columns has been promoted to pre-deploy
  ]

  include Searchable
  include Roleable
  include HasCustomFields
  include SecondFactorManager
  include HasDestroyedWebHook
  include HasDeprecatedColumns

  DEFAULT_FEATURED_BADGE_COUNT = 3
  MAX_SIMILAR_USERS = 10

  deprecate_column :flag_level, drop_from: "3.2"

  # not deleted on user delete
  has_many :posts
  has_many :topics
  has_many :uploads

  has_many :category_users, dependent: :destroy
  has_many :tag_users, dependent: :destroy
  has_many :user_api_keys, dependent: :destroy
  has_many :topic_allowed_users, dependent: :destroy
  has_many :user_archived_messages, dependent: :destroy
  has_many :email_change_requests, dependent: :destroy
  has_many :email_tokens, dependent: :destroy
  has_many :topic_links, dependent: :destroy
  has_many :user_uploads, dependent: :destroy
  has_many :upload_references, as: :target, dependent: :destroy
  has_many :user_emails, dependent: :destroy, autosave: true
  has_many :user_associated_accounts, dependent: :destroy
  has_many :oauth2_user_infos, dependent: :destroy
  has_many :user_second_factors, dependent: :destroy
  has_many :user_badges, -> { for_enabled_badges }, dependent: :destroy
  has_many :user_auth_tokens, dependent: :destroy
  has_many :group_users, dependent: :destroy
  has_many :user_warnings, dependent: :destroy
  has_many :api_keys, dependent: :destroy
  has_many :push_subscriptions, dependent: :destroy
  has_many :acting_group_histories,
           dependent: :destroy,
           foreign_key: :acting_user_id,
           class_name: "GroupHistory"
  has_many :targeted_group_histories,
           dependent: :destroy,
           foreign_key: :target_user_id,
           class_name: "GroupHistory"
  has_many :reviewable_scores, dependent: :destroy
  has_many :invites, foreign_key: :invited_by_id, dependent: :destroy
  has_many :user_custom_fields, dependent: :destroy
  has_many :user_associated_groups, dependent: :destroy
  has_many :pending_posts,
           -> { merge(Reviewable.pending) },
           class_name: "ReviewableQueuedPost",
           foreign_key: :target_created_by_id

  has_one :user_option, dependent: :destroy
  has_one :user_avatar, dependent: :destroy
  has_one :primary_email,
          -> { where(primary: true) },
          class_name: "UserEmail",
          dependent: :destroy,
          autosave: true,
          validate: false
  has_one :user_stat, dependent: :destroy
  has_one :user_profile, dependent: :destroy, inverse_of: :user
  has_one :single_sign_on_record, dependent: :destroy
  has_one :anonymous_user_master, class_name: "AnonymousUser", dependent: :destroy
  has_one :anonymous_user_shadow,
          ->(record) { where(active: true) },
          foreign_key: :master_user_id,
          class_name: "AnonymousUser",
          dependent: :destroy
  has_one :invited_user, dependent: :destroy
  has_one :user_notification_schedule, dependent: :destroy
  has_one :user_password, class_name: "UserPassword", dependent: :destroy, autosave: true

  # delete all is faster but bypasses callbacks
  has_many :bookmarks, dependent: :delete_all
  has_many :notifications, dependent: :delete_all
  has_many :topic_users, dependent: :delete_all
  has_many :incoming_emails, dependent: :delete_all
  has_many :user_visits, dependent: :delete_all
  has_many :user_auth_token_logs, dependent: :delete_all
  has_many :group_requests, dependent: :delete_all
  has_many :muted_user_records, class_name: "MutedUser", dependent: :delete_all
  has_many :ignored_user_records, class_name: "IgnoredUser", dependent: :delete_all
  has_many :do_not_disturb_timings, dependent: :delete_all
  has_many :sidebar_sections, dependent: :destroy
  has_one :user_status, dependent: :destroy

  # dependent deleting handled via before_destroy (special cases)
  has_many :user_actions
  has_many :post_actions
  has_many :post_timings
  has_many :directory_items
  has_many :email_logs
  has_many :security_keys, -> { where(enabled: true) }, class_name: "UserSecurityKey"
  has_many :all_security_keys, class_name: "UserSecurityKey"

  has_many :badges, through: :user_badges
  has_many :default_featured_user_badges,
           -> do
             max_featured_rank =
               (
                 if SiteSetting.max_favorite_badges > 0
                   SiteSetting.max_favorite_badges + 1
                 else
                   DEFAULT_FEATURED_BADGE_COUNT
                 end
               )
             for_enabled_badges.grouped_with_count.where("featured_rank <= ?", max_featured_rank)
           end,
           class_name: "UserBadge"

  has_many :topics_allowed, through: :topic_allowed_users, source: :topic
  has_many :groups, through: :group_users
  has_many :secure_categories, -> { distinct }, through: :groups, source: :categories
  has_many :associated_groups, through: :user_associated_groups, dependent: :destroy

  # deleted in user_second_factors relationship
  has_many :totps,
           -> { where(method: UserSecondFactor.methods[:totp], enabled: true) },
           class_name: "UserSecondFactor"

  has_one :master_user, through: :anonymous_user_master
  has_one :shadow_user, through: :anonymous_user_shadow, source: :user

  has_one :profile_background_upload, through: :user_profile
  has_one :card_background_upload, through: :user_profile
  belongs_to :approved_by, class_name: "User"
  belongs_to :primary_group, class_name: "Group"
  belongs_to :flair_group, class_name: "Group"

  has_many :muted_users, through: :muted_user_records
  has_many :ignored_users, through: :ignored_user_records

  belongs_to :uploaded_avatar, class_name: "Upload"

  has_many :sidebar_section_links, dependent: :delete_all
  has_many :embeddable_hosts

  delegate :last_sent_email_address, to: :email_logs

  validates_presence_of :username
  validate :username_validator, if: :will_save_change_to_username?
  validate :password_validator
  validate :name_validator, if: :will_save_change_to_name?
  validates :name, user_full_name: true, if: :will_save_change_to_name?, length: { maximum: 255 }
  validates :ip_address, allowed_ip_address: { on: :create }
  validates :primary_email, presence: true, unless: :skip_email_validation
  validates :validatable_user_fields_values,
            watched_words: true,
            unless: :should_skip_user_fields_validation?

  validates_associated :primary_email,
                       message: ->(_, user_email) do
                         user_email[:value]&.errors&.[](:email)&.first.to_s
                       end

  after_initialize :add_trust_level

  before_validation :set_skip_validate_email

  after_create :create_email_token
  after_create :create_user_stat
  after_create :create_user_option
  after_create :create_user_profile
  after_create :set_random_avatar
  after_create :ensure_in_trust_level_group
  after_create :set_default_categories_preferences
  after_create :set_default_tags_preferences
  after_create :set_default_sidebar_section_links
  after_update :set_default_sidebar_section_links, if: Proc.new { self.saved_change_to_staged? }

  after_update :trigger_user_updated_event,
               if: Proc.new { self.human? && self.saved_change_to_uploaded_avatar_id? }

  after_update :trigger_user_automatic_group_refresh, if: :saved_change_to_staged?
  after_update :change_display_name, if: :saved_change_to_name?

  before_save :update_usernames
  before_save :match_primary_group_changes
  before_save :check_if_title_is_badged_granted
  before_save :apply_watched_words, unless: :should_skip_user_fields_validation?
  before_save :check_qualification_for_users_directory,
              if: Proc.new { SiteSetting.bootstrap_mode_enabled }

  after_save :expire_tokens_if_password_changed
  after_save :clear_global_notice_if_needed
  after_save :refresh_avatar
  after_save :badge_grant
  after_save :index_search
  after_save :check_site_contact_username
  after_save :add_to_user_directory,
             if: Proc.new { SiteSetting.bootstrap_mode_enabled && @qualified_for_users_directory }

  after_save do
    if saved_change_to_uploaded_avatar_id?
      UploadReference.ensure_exist!(upload_ids: [self.uploaded_avatar_id], target: self)
    end
  end

  after_commit :trigger_user_created_event, on: :create
  after_commit :trigger_user_destroyed_event, on: :destroy

  before_destroy do
    # These tables don't have primary keys, so destroying them with activerecord is tricky:
    PostTiming.where(user_id: self.id).delete_all
    TopicViewItem.where(user_id: self.id).delete_all
    UserAction.where(
      "user_id = :user_id OR target_user_id = :user_id OR acting_user_id = :user_id",
      user_id: self.id,
    ).delete_all

    # we need to bypass the default scope here, which appears not bypassed for :delete_all
    # however :destroy it is bypassed
    PostAction.with_deleted.where(user_id: self.id).delete_all

    # This is a perf optimisation to ensure we hit the index
    # without this we need to scan a much larger number of rows
    DirectoryItem
      .where(user_id: self.id)
      .where("period_type in (?)", DirectoryItem.period_types.values)
      .delete_all

    # our relationship filters on enabled, this makes sure everything is deleted
    UserSecurityKey.where(user_id: self.id).delete_all

    Developer.where(user_id: self.id).delete_all
    DraftSequence.where(user_id: self.id).delete_all
    GivenDailyLike.where(user_id: self.id).delete_all
    MutedUser.where(user_id: self.id).or(MutedUser.where(muted_user_id: self.id)).delete_all
    IgnoredUser.where(user_id: self.id).or(IgnoredUser.where(ignored_user_id: self.id)).delete_all
    UserAvatar.where(user_id: self.id).delete_all
  end

  # Skip validating email, for example from a particular auth provider plugin
  attr_accessor :skip_email_validation

  # Whether we need to be sending a system message after creation
  attr_accessor :send_welcome_message

  # This is just used to pass some information into the serializer
  attr_accessor :notification_channel_position

  # set to true to optimize creation and save for imports
  attr_accessor :import_mode

  # Cache for user custom fields. Currently it is used to display quick search results
  attr_accessor :custom_data

  # Information if user was authenticated with OAuth
  attr_accessor :authenticated_with_oauth

  scope :with_email,
        ->(email) { joins(:user_emails).where("lower(user_emails.email) IN (?)", email) }

  scope :with_primary_email,
        ->(email) do
          joins(:user_emails).where(
            "lower(user_emails.email) IN (?) AND user_emails.primary",
            email,
          )
        end

  scope :human_users,
        ->(allowed_bot_user_ids: nil) do
          if allowed_bot_user_ids.present?
            where("users.id > 0 OR users.id IN (?)", allowed_bot_user_ids)
          else
            where("users.id > 0")
          end
        end

  # excluding fake users like the system user or anonymous users
  scope :real,
        ->(allowed_bot_user_ids: nil) do
          human_users(allowed_bot_user_ids: allowed_bot_user_ids).where(
            "NOT EXISTS(
                     SELECT 1
                     FROM anonymous_users a
                     WHERE a.user_id = users.id
                  )",
          )
        end

  # TODO-PERF: There is no indexes on any of these
  # and NotifyMailingListSubscribers does a select-all-and-loop
  # may want to create an index on (active, silence, suspended_till)?
  scope :silenced, -> { where("silenced_till IS NOT NULL AND silenced_till > ?", Time.zone.now) }
  scope :not_silenced, -> { where("silenced_till IS NULL OR silenced_till <= ?", Time.zone.now) }
  scope :suspended, -> { where("suspended_till IS NOT NULL AND suspended_till > ?", Time.zone.now) }
  scope :not_suspended, -> { where("suspended_till IS NULL OR suspended_till <= ?", Time.zone.now) }
  scope :activated, -> { where(active: true) }
  scope :not_staged, -> { where(staged: false) }
  scope :approved, -> { where(approved: true) }

  scope :filter_by_username,
        ->(filter) do
          if filter.is_a?(Array)
            where("username_lower ~* ?", "(#{filter.join("|")})")
          else
            where("username_lower ILIKE ?", "%#{filter}%")
          end
        end

  scope :filter_by_username_or_email,
        ->(filter) do
          if filter.is_a?(String) && filter =~ /.+@.+/
            # probably an email so try the bypass
            if user_id = UserEmail.where("lower(email) = ?", filter.downcase).pick(:user_id)
              return where("users.id = ?", user_id)
            end
          end

          users = joins(:primary_email)

          if filter.is_a?(Array)
            users.where(
              "username_lower ~* :filter OR lower(user_emails.email) SIMILAR TO :filter",
              filter: "(#{filter.join("|")})",
            )
          else
            users.where(
              "username_lower ILIKE :filter OR lower(user_emails.email) ILIKE :filter",
              filter: "%#{filter}%",
            )
          end
        end

  scope :watching_topic,
        ->(topic) do
          joins(
            DB.sql_fragment(
              "LEFT JOIN category_users ON category_users.user_id = users.id AND category_users.category_id = :category_id",
              category_id: topic.category_id,
            ),
          )
            .joins(
              DB.sql_fragment(
                "LEFT JOIN topic_users ON topic_users.user_id = users.id AND topic_users.topic_id = :topic_id",
                topic_id: topic.id,
              ),
            )
            .joins(
              "LEFT JOIN tag_users ON tag_users.user_id = users.id AND tag_users.tag_id IN (#{topic.tag_ids.join(",").presence || "NULL"})",
            )
            .where(
              "category_users.notification_level > 0 OR topic_users.notification_level > 0 OR tag_users.notification_level > 0",
            )
        end

  module NewTopicDuration
    ALWAYS = -1
    LAST_VISIT = -2
  end

  MAX_STAFF_DELETE_POST_COUNT = 5

### self

staged users can use the same username since they will take over the account
    email.present? &&
      User.joins(:user_emails).exists?(
        staged: true,
        username_lower: lower,
        user_emails: {
          primary: true,
          email: email,
        },
      )
  end

### sync_notification_channel_position

tricky, we need our bus to be subscribed from the right spot

### should_validate_email_address

this is unfortunate, but when an invite is redeemed,
    # any user created by the invite is created *after*
    # the invite's redeemed_at
    invite_redemption_delay = 5.seconds
    used_invite =
      Invite
        .with_deleted
        .joins(:invited_users)
        .where(
          "invited_users.user_id = ? AND invited_users.redeemed_at <= ?",
          self.id,
          self.created_at + invite_redemption_delay,
        )
        .first
    used_invite.try(:invited_by)
  end

### unread_notifications_of_priority

perf critical, much more efficient than AR
    sql = <<~SQL
        SELECT COUNT(*)
          FROM notifications n
     LEFT JOIN topics t ON t.id = n.topic_id
         WHERE t.deleted_at IS NULL
           AND n.notification_type = :notification_type
           AND n.user_id = :user_id
           AND NOT read
           #{since ? "AND n.created_at > :since" : ""}
    SQL

    # to avoid coalesce we do to_i
    DB.query_single(sql, user_id: id, notification_type: notification_type, since: since)[0].to_i
  end

### grouped_unread_notifications

perf critical, much more efficient than AR
    sql = <<~SQL
        SELECT COUNT(*)
          FROM notifications n
     LEFT JOIN topics t ON t.id = n.topic_id
         WHERE t.deleted_at IS NULL
           AND n.high_priority = :high_priority
           AND n.user_id = :user_id
           AND NOT read
    SQL

    # to avoid coalesce we do to_i
    DB.query_single(sql, user_id: id, high_priority: high_priority)[0].to_i
  end

  MAX_UNREAD_BACKLOG = 400

### self

PERF: This safeguard is in place to avoid situations where
  # a user with enormous amounts of unread data can issue extremely
  # expensive queries
  MAX_UNREAD_NOTIFICATIONS = 99

### all_unread_notifications_count

perf critical, much more efficient than AR
        sql = <<~SQL
        SELECT COUNT(*) FROM (
          SELECT 1 FROM
          notifications n
          LEFT JOIN topics t ON t.id = n.topic_id
           WHERE t.deleted_at IS NULL AND
            n.high_priority = FALSE AND
            n.user_id = :user_id AND
            n.id > :seen_notification_id AND
            NOT read
          LIMIT :limit
        ) AS X
      SQL

        DB.query_single(
          sql,
          user_id: id,
          seen_notification_id: seen_notification_id,
          limit: User.max_unread_notifications,
        )[
          0
        ].to_i
      end
  end

### publish_do_not_disturb

publish last notification json with the message so we can apply an update
    notification = notifications.visible.order("notifications.created_at desc").first
    json = NotificationSerializer.new(notification).as_json if notification

    sql = (<<~SQL)
       SELECT * FROM (
         SELECT n.id, n.read FROM notifications n
         LEFT JOIN topics t ON n.topic_id = t.id
         WHERE
          t.deleted_at IS NULL AND
          n.high_priority AND
          n.user_id = :user_id AND
          NOT read
        ORDER BY n.id DESC
        LIMIT 20
      ) AS x
      UNION ALL
      SELECT * FROM (
       SELECT n.id, n.read FROM notifications n
       LEFT JOIN topics t ON n.topic_id = t.id
       WHERE
        t.deleted_at IS NULL AND
        (n.high_priority = FALSE OR read) AND
        n.user_id = :user_id
       ORDER BY n.id DESC
       LIMIT 20
      ) AS y
    SQL

    recent = DB.query(sql, user_id: id).map! { |r| [r.id, r.read] }

    payload = {
      unread_notifications: unread_notifications,
      unread_high_priority_notifications: unread_high_priority_notifications,
      read_first_notification: read_first_notification?,
      last_notification: json,
      recent: recent,
      seen_notification_id: seen_notification_id,
    }

    payload[:all_unread_notifications_count] = all_unread_notifications_count
    payload[:grouped_unread_notifications] = grouped_unread_notifications
    payload[:new_personal_messages_notifications_count] = new_personal_messages_notifications_count

    MessageBus.publish("/notification/#{id}", payload, user_ids: [id])
  end

### password

special case for passwordless accounts
    return if pw.blank?

    if user_password
      user_password.password = pw
    else
      build_user_password(password: pw)
    end
    @raw_password = pw # still required to maintain compatibility with usage of password-related User interface
  end

### password_required

Indicate that this is NOT a passwordless account for the purposes of validation

### update_posts_read

we only want to update the user's timezone if they have not set it themselves
    UserOption.where(user_id: self.id, timezone: nil).update_all(timezone: timezone)
  end

### self

using update_column to avoid the AR transaction
    update_column(:last_seen_at, now)
    update_column(:first_seen_at, now) unless self.first_seen_at

    DiscourseEvent.trigger(:user_seen, self)
  end

### small_avatar_url

Don't pass this up to the client - it's meant for server side use
  # This is used in
  #   - self oneboxes in open graph data
  #   - emails

### self

TODO it may be worth caching this in a distributed cache, should be benched
    if SiteSetting.external_system_avatars_enabled
      url = SiteSetting.external_system_avatars_url.dup
      url = +"#{Discourse.base_path}#{url}" unless url =~ %r{\Ahttps?://}
      url.gsub! "{color}", letter_avatar_color(normalized_username)
      url.gsub! "{username}", UrlHelper.encode_component(username)
      url.gsub! "{first_letter}",
                UrlHelper.encode_component(normalized_username.grapheme_clusters.first)
      url.gsub! "{hostname}", Discourse.current_hostname
      url
    else
      "#{Discourse.base_path}/letter_avatar/#{normalized_username}/{size}/#{LetterAvatar.version}.png"
    end
  end

### like_count

The following count methods are somewhat slow - definitely don't use them in a loop.
  # They might need to be denormalized

### delete_posts_in_batches

Does not apply to staff and non-new members...
    return false if staff? || (trust_level != TrustLevel[0])
    # ... your own topics or in private messages
    topic = Topic.where(id: topic_id).first
    return false if topic.try(:private_message?) || (topic.try(:user_id) == self.id)

    last_action_in_topic = UserAction.last_action_in_topic(id, topic_id)
    since_reply = Post.where(user_id: id, topic_id: topic_id)
    since_reply = since_reply.where("id > ?", last_action_in_topic) if last_action_in_topic

    (since_reply.count >= SiteSetting.newuser_max_replies_per_topic)
  end

### has_trust_level

Use this helper to determine if the user has a particular trust level.
  # Takes into account admin, etc.

### admin

a touch faster than automatic

### flag_linked_posts_as_spam

Flag all posts from a user as spam

### first_post_created_at

mark all the user's quoted posts as "needing a rebake"
    Post.rebake_all_quoted_posts(self.id) if saved_change_to_uploaded_avatar_id?
  end

### number_of_deleted_posts

ignore multiselect fields since they are admin-set and thus not user generated content
    @public_user_field_ids ||=
      UserField.public_fields.where.not(field_type: "multiselect").pluck(:id)

    user_fields(@public_user_field_ids)
  end

### email

Shortcut to set the primary email of the user.
  # Automatically removes any identical secondary emails.

### clear_global_notice_if_needed

force is needed as user custom fields are updated using SQL and after_save callback is not triggered
    SearchIndexer.index(self, force: true)
  end

### hash_password

NOTE: setting raw password is the only valid way of changing a password
    # the password field in the DB is actually hashed, nobody should be amending direct
    if @raw_password
      # Association in model may be out-of-sync
      UserAuthToken.where(user_id: id).destroy_all

      email_tokens.where("not expired").update_all(expired: true) if !saved_change_to_id?

      # We should not carry this around after save
      @raw_password = nil
      @password_required = false
    end
  end

### update_usernames

there is a possibility we did not load trust level column, skip it
    return unless has_attribute? :trust_level
    self.trust_level ||= SiteSetting.default_trust_level
  end

### set_default_tags_preferences

The following site settings are used to pre-populate default category
    # tracking settings for a user:
    #
    # * default_categories_watching
    # * default_categories_tracking
    # * default_categories_watching_first_post
    # * default_categories_normal
    # * default_categories_muted
    %w[watching watching_first_post tracking normal muted].each do |setting|
      category_ids = SiteSetting.get("default_categories_#{setting}").split("|").map(&:to_i)
      category_ids.each do |category_id|
        next if category_id == 0
        values << {
          user_id: self.id,
          category_id: category_id,
          notification_level: CategoryUser.notification_levels[setting.to_sym],
        }
      end
    end

    CategoryUser.insert_all(values) if values.present?
  end

### self

The following site settings are used to pre-populate default tag
    # tracking settings for a user:
    #
    # * default_tags_watching
    # * default_tags_tracking
    # * default_tags_watching_first_post
    # * default_tags_muted
    %w[watching watching_first_post tracking muted].each do |setting|
      tag_names = SiteSetting.get("default_tags_#{setting}").split("|")
      now = Time.zone.now

      Tag
        .where(name: tag_names)
        .pluck(:id)
        .each do |tag_id|
          values << {
            user_id: self.id,
            tag_id: tag_id,
            notification_level: TagUser.notification_levels[setting.to_sym],
            created_at: now,
            updated_at: now,
          }
        end
    end

    TagUser.insert_all(values) if values.present?
  end

### match_primary_group_changes

keep going
        end
      end
  end

