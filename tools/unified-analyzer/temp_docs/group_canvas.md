# Group

## Description

frozen_string_literal: true

Copyright (C) 2011 - present Instructure, Inc.

This file is part of Canvas.

Canvas is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, version 3 of the License.

Canvas is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <http://www.gnu.org/licenses/>.


## Relationships

- has_many :group_memberships
- has_many :users
- has_many :user_past_lti_ids
- has_many :participating_group_memberships
- has_many :participating_users
- belongs_to :context
- belongs_to :group_category
- belongs_to :account
- belongs_to :root_account
- has_many :calendar_events
- has_many :discussion_topics
- has_many :active_discussion_topics
- has_many :all_discussion_topics
- has_many :discussion_entries
- has_many :announcements
- has_many :active_announcements
- has_many :attachments
- has_many :active_images
- has_many :active_assignments
- has_many :all_attachments
- has_many :folders
- has_many :active_folders
- has_many :submissions_folders
- has_many :collaborators
- has_many :external_feeds
- has_many :messages
- belongs_to :wiki
- has_many :wiki_pages
- has_many :wiki_page_lookups
- has_many :web_conferences
- has_many :collaborations
- has_many :media_objects
- has_many :content_migrations
- has_many :content_exports
- has_many :usage_rights
- belongs_to :avatar_attachment
- belongs_to :leader
- has_many :lti_resource_links
- has_many :favorites

## Methods

### refresh_group_discussion_topics

frozen_string_literal: true

#
# Copyright (C) 2011 - present Instructure, Inc.
#
# This file is part of Canvas.
#
# Canvas is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, version 3 of the License.
#
# Canvas is distributed in the hope that it will be useful, but WITHOUT ANY
# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
# A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
# details.
#
# You should have received a copy of the GNU Affero General Public License along
# with this program. If not, see <http://www.gnu.org/licenses/>.
#

class Group < ActiveRecord::Base
  self.ignored_columns += ["category"]

  include Context
  include Workflow
  include CustomValidations

  validates :context_id, :context_type, :account_id, :root_account_id, :workflow_state, :uuid, presence: true
  validates_allowed_transitions :is_public, false => true

  validates :sis_source_id, uniqueness: { scope: :root_account }, allow_nil: true

  attr_readonly :non_collaborative
  validate :validate_non_collaborative_constraints

  # use to skip queries in can_participate?, called by policy block
  attr_accessor :can_participate

  has_many :group_memberships, -> { where("group_memberships.workflow_state<>'deleted'") }, dependent: :destroy
  has_many :users, -> { where("users.workflow_state<>'deleted'") }, through: :group_memberships
  has_many :user_past_lti_ids, as: :context, inverse_of: :context
  has_many :participating_group_memberships, -> { where(workflow_state: "accepted") }, class_name: "GroupMembership"
  has_many :participating_users, source: :user, through: :participating_group_memberships
  belongs_to :context, polymorphic: [:course, { context_account: "Account" }]
  belongs_to :group_category
  belongs_to :account
  belongs_to :root_account, class_name: "Account", inverse_of: :all_groups
  has_many :calendar_events, as: :context, inverse_of: :context, dependent: :destroy
  has_many :discussion_topics, -> { where("discussion_topics.workflow_state<>'deleted'").preload(:user).order("discussion_topics.position DESC, discussion_topics.created_at DESC") }, dependent: :destroy, as: :context, inverse_of: :context
  has_many :active_discussion_topics, -> { where("discussion_topics.workflow_state<>'deleted'").preload(:user) }, as: :context, inverse_of: :context, class_name: "DiscussionTopic"
  has_many :all_discussion_topics, -> { preload(:user) }, as: :context, inverse_of: :context, class_name: "DiscussionTopic", dependent: :destroy
  has_many :discussion_entries, -> { preload(:discussion_topic, :user) }, through: :discussion_topics, dependent: :destroy
  has_many :announcements, as: :context, inverse_of: :context, class_name: "Announcement", dependent: :destroy
  has_many :active_announcements, -> { where("discussion_topics.workflow_state<>'deleted'") }, as: :context, inverse_of: :context, class_name: "Announcement"
  has_many :attachments, as: :context, inverse_of: :context, dependent: :destroy, extend: Attachment::FindInContextAssociation
  has_many :active_images, -> { where("attachments.file_state<>'deleted' AND attachments.content_type LIKE 'image%'").order("attachments.display_name").preload(:thumbnail) }, as: :context, inverse_of: :context, class_name: "Attachment"
  has_many :active_assignments, -> { where("assignments.workflow_state<>'deleted'") }, as: :context, inverse_of: :context, class_name: "Assignment"
  has_many :all_attachments, as: "context", class_name: "Attachment"
  has_many :folders, -> { order("folders.name") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :active_folders, -> { where("folders.workflow_state<>'deleted'").order("folders.name") }, class_name: "Folder", as: :context, inverse_of: :context
  has_many :submissions_folders, -> { where.not(folders: { submission_context_code: nil }) }, as: :context, inverse_of: :context, class_name: "Folder"
  has_many :collaborators
  has_many :external_feeds, as: :context, inverse_of: :context, dependent: :destroy
  has_many :messages, as: :context, inverse_of: :context, dependent: :destroy
  belongs_to :wiki
  has_many :wiki_pages, as: :context, inverse_of: :context
  has_many :wiki_page_lookups, as: :context, inverse_of: :context
  has_many :web_conferences, as: :context, inverse_of: :context, dependent: :destroy
  has_many :collaborations, -> { order(Arel.sql("collaborations.title, collaborations.created_at")) }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :media_objects, as: :context, inverse_of: :context
  has_many :content_migrations, as: :context, inverse_of: :context
  has_many :content_exports, as: :context, inverse_of: :context
  has_many :usage_rights, as: :context, inverse_of: :context, class_name: "UsageRights", dependent: :destroy
  belongs_to :avatar_attachment, class_name: "Attachment"
  belongs_to :leader, class_name: "User"
  has_many :lti_resource_links,
           as: :context,
           inverse_of: :context,
           class_name: "Lti::ResourceLink",
           dependent: :destroy
  has_many :favorites, as: :context, inverse_of: :context, dependent: :destroy

  before_validation :ensure_defaults
  before_save :update_max_membership_from_group_category

  after_create :refresh_group_discussion_topics
  after_save :touch_context, if: :saved_change_to_workflow_state?

  after_update :clear_cached_short_name, if: :saved_change_to_name?

  delegate :time_zone, to: :context
  delegate :usage_rights_required?, to: :context
  delegate :allow_student_anonymous_discussion_topics, to: :context
  delegate :discussion_checkpoints_enabled?, to: :account

  include StickySisFields
  are_sis_sticky :name

  validates_each :name do |record, attr, value|
    if value.blank?
      record.errors.add attr, t(:name_required, "Name is required")
    elsif value.length > maximum_string_length
      record.errors.add attr, t(:name_too_long, "Enter a shorter group name")
    end
  end

  validates_each :max_membership do |record, attr, value|
    next if value.nil?

    record.errors.add attr, t(:greater_than_1, "Must be greater than 1") unless value.to_i > 1
  end

  validates_with HorizonValidators::GroupValidator, if: -> { context.is_a?(Course) && context.horizon_course? }

### full_name

>99.9% of groups have fewer than 100 members
      User.where(id: participating_users_in_context.pluck(:id) + context.participating_admins.pluck(:id))
    else
      participating_users
    end
  end

### add_user

this method is idempotent

### set_users

only update moderator if true/false is explicitly passed in
        member.moderator = moderator unless moderator.nil?
        member.save if member.changed?
      else
        member = group_memberships.create(attrs)
      end
    end
    # permissions for this user in the group are probably different now
    clear_permissions_cache(user)
    member
  end

### account

update root account when account changes

### users_visible_to

if you modify this set_policy block, note that we've denormalized this
  # permission check for efficiency -- see User#cached_contexts
  set_policy do
    # Base permissions for users who can participate in the group
    # Conditions:
    # - The group is collaborative (`!non_collaborative?`)
    # - A valid user is present (`user`)
    # - The user can participate (`can_participate?(user)`)
    # - The user is a member of the group (`has_member?(user)`)
    given { |user| !non_collaborative? && user && can_participate?(user) && has_member?(user) }
    can :participate,
        :manage_calendar,
        :manage_course_content_add,
        :manage_course_content_edit,
        :manage_course_content_delete,
        :manage_files_add,
        :manage_files_edit,
        :manage_files_delete,
        :manage_wiki_create,
        :manage_wiki_delete,
        :manage_wiki_update,
        :post_to_forum,
        :create_collaborations,
        :create_forum

    # Course-level groups don't grant any permissions besides :participate (because for a teacher to add a student to a
    # group, the student must be able to :participate, and the teacher should be able to add students while the course
    # is unpublished and therefore unreadable to said students) unless their containing context can be read by the user
    # in question
    # Conditions:
    # - The group is collaborative (`!non_collaborative?`)
    # - The context is either an Account or grants read permission to the user
    given { |user, session| !non_collaborative? && (context.is_a?(Account) || context&.grants_right?(user, session, :read) || false) }

    use_additional_policy do
      given { |user| user && has_member?(user) }
      can %i[
        read_forum
        read
        read_announcements
        read_roster
        view_unpublished_items
        read_files
      ]

      given do |user, session|
        next false unless user

        if context.nil? || context.is_a?(Account)
          has_member?(user)
        else
          context.grants_any_right?(user, session, :send_messages, :send_messages_all)
        end
      end
      can :send_messages
      can :send_messages_all
      # if I am a member of this group and I can moderate_forum in the group's context
      # (makes it so group members cant edit each other's discussion entries)
      given { |user, session| user && has_member?(user) && (!context || context.grants_right?(user, session, :moderate_forum)) }
      can :moderate_forum

      given { |user| user && has_moderator?(user) }
      can :delete and
        can :manage and
        can :allow_course_admin_actions and
        can :manage_students and
        can :moderate_forum and
        can :update

      given { |user| user && leader == user }
      can :update

      given { group_category.try(:communities?) }
      can :create

      given { |user, session| context&.grants_right?(user, session, :participate_as_student) }
      can :participate_as_student

      given { |user, session| grants_right?(user, session, :participate_as_student) && context.allow_student_organized_groups }
      can :create

      given do |user, session|
        context.grants_right?(user, session, :manage_groups_add)
      end
      can %i[read read_files create]

      # permissions to update a group and manage actions within the context of a group
      given do |user, session|
        context.grants_right?(user, session, :manage_groups_manage)
      end
      can %i[
        read
        update
        create_collaborations
        manage
        allow_course_admin_actions
        manage_calendar
        manage_course_content_add
        manage_course_content_edit
        manage_course_content_delete
        manage_files_add
        manage_files_edit
        manage_files_delete
        manage_students
        manage_wiki_create
        manage_wiki_delete
        manage_wiki_update
        moderate_forum
        post_to_forum
        create_forum
        read_forum
        read_announcements
        read_roster
        send_messages
        send_messages_all
        view_unpublished_items
        read_files
      ]

      given do |user, session|
        context.grants_right?(user, session, :manage_groups_delete)
      end
      can %i[read read_files delete]

      given { |user, session| context&.grants_all_rights?(user, session, :read_as_admin, :post_to_forum) }
      can :post_to_forum

      given { |user, session| context&.grants_all_rights?(user, session, :read_as_admin, :create_forum) }
      can :create_forum

      given { |user, session| context&.grants_right?(user, session, :view_group_pages) }
      can %i[read read_forum read_announcements read_roster read_files]

      # Join is participate + the group being in a state that allows joining directly (free_association)
      given { |user| user && can_participate?(user) && free_association?(user) }
      can :join and can :read_roster

      given { |user| user && (self.group_category.try(:allows_multiple_memberships?) || allow_self_signup?(user)) }
      can :leave

      given do |user, session|
        grants_right?(user, session, :manage_course_content_add) &&
          context&.grants_right?(user, session, :create_conferences)
      end
      can :create_conferences

      given { |user, session| context&.grants_right?(user, session, :read_as_admin) }
      can :read_as_admin

      given { |user, session| context&.grants_right?(user, session, :read_sis) }
      can :read_sis

      given { |user, session| context&.grants_right?(user, session, :view_user_logins) }
      can :view_user_logins

      given { |user, session| context&.grants_right?(user, session, :read_email_addresses) }
      can :read_email_addresses
    end

    ##################### Non-Collaborative Group Permission Block ##########################
    # Permissions for non-collaborative groups
    # Conditions:
    # - The group is non-collaborative (`non_collaborative?`)
    # - The context grants read permission
    # - The context grants any manage_tag rights
    given { |user, session| non_collaborative? && context&.grants_right?(user, session, :read) && context.grants_any_right?(user, session, *RoleOverride::GRANULAR_MANAGE_TAGS_PERMISSIONS) }
    use_additional_policy do
      # Base permissions for non-collaborative groups
      given { |user| user }
      can :read,
          :read_roster

      # Permission to send messages
      # Conditions:
      # - A valid user is present
      # - The context grants send_messages right
      given do |user, session|
        user && context.grants_right?(user, session, :send_messages)
      end
      can :send_messages

      # Permission to send all messages
      # Conditions:
      # - A valid user is present
      # - The context grants send_messages_all right
      given do |user, session|
        user && context.grants_right?(user, session, :send_messages_all)
      end
      can :send_messages_all

      # Permission to manage/update the group
      # Conditions:
      # - A valid user is present
      # - The context grants manage_tags_manage right
      given { |user, session| user && context&.grants_right?(user, session, :manage_tags_manage) }
      can :update,
          :manage,
          :allow_course_admin_actions,
          :manage_students

      # Permission to delete the group
      # Conditions:
      # - A valid user is present
      # - The context grants manage_tags_manage right
      given { |user, session| user && context.grants_right?(user, session, :manage_tags_delete) }
      can :delete

      # Permission to create the group
      # Conditions:
      # - A valid user is present
      # - The context grants manage_tags_add right
      given { |user, session| user && context.grants_right?(user, session, :manage_tags_add) }
      can :create

      given { |user, session| context&.grants_right?(user, session, :view_group_pages) }
      can %i[read read_roster read_files]

      given { |user, session| context&.grants_right?(user, session, :read_as_admin) }
      can :read_as_admin

      given { |user, session| context&.grants_right?(user, session, :read_sis) }
      can :read_sis

      given { |user, session| context&.grants_right?(user, session, :view_user_logins) }
      can :view_user_logins

      given { |user, session| context&.grants_right?(user, session, :read_email_addresses) }
      can :read_email_addresses

      # Permissions purposely excluded from non_collaborative groups because Non_collaborative groups will NEVER
      # be used as a context that owns content. So no user should ever be able to manage content in a non_collaborative group.
      # %i[
      #   manage_calendar
      #   manage_course_content_add
      #   manage_course_content_edit
      #   manage_course_content_delete
      #   manage_files_add
      #   manage_files_edit
      #   manage_files_delete
      #   manage_wiki_create
      #   manage_wiki_delete
      #   manage_wiki_update
      #   moderate_forum
      #   post_to_forum
      #   create_forum
      #   read_forum
      #   read_announcements
      #   view_unpublished_items
      #   read_files
      # ]
    end
  end

### can_participate

Helper needed by several permissions, use grants_right?(user, :participate)

### has_common_section

remove anything coming automatically from deprecated db column
      json["group"].delete("category")
      if self.group_category
        # put back version from association
        json["group"]["group_category"] = self.group_category.name
      end
    end
    json
  end

### feature_enabled

Public: Determine whether a feature is enabled, deferring to the group's context.
  #
  # Returns a boolean.

### grading_periods

shouldn't matter, but most specs create anonymous (contextless) groups :(
    return false if context.nil?

    context.feature_enabled?(feature)
  end

### sortable_name

This implicitly includes add_federated_parent_to_chain
    if include_site_admin
      return @account_chain_with_site_admin ||= Account.add_site_admin_to_chain!(@account_chain.dup).freeze
    end

    if include_federated_parent
      return @account_chain_with_federated_parent ||= Account.add_federated_parent_to_chain!(@account_chain.dup).freeze
    end

    @account_chain
  end

### favorite_for_user

#
  # Returns a boolean describing if the user passed in has marked this group
  # as a favorite.

