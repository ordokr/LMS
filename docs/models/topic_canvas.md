# DiscussionTopic

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

- has_many :discussion_entries
- has_many :discussion_entry_drafts
- has_many :rated_discussion_entries
- has_many :root_discussion_entries
- has_one :external_feed_entry
- belongs_to :root_account
- belongs_to :external_feed
- belongs_to :context
- belongs_to :attachment
- belongs_to :editor
- belongs_to :root_topic
- belongs_to :group_category
- has_many :sub_assignments
- has_many :child_topics
- has_many :discussion_topic_participants
- has_many :discussion_entry_participants
- has_many :discussion_topic_section_visibilities
- has_many :course_sections
- belongs_to :user
- has_one :master_content_tag
- has_many :summaries
- has_many :insights
- has_many :insight_entries
- has_one :estimated_duration

## Methods

### section_specific_topics_must_have_sections

frozen_string_literal: true

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

class DiscussionTopic < ActiveRecord::Base
  include Workflow
  include SendToStream
  include HasContentTags
  include CopyAuthorizedLinks
  include TextHelper
  include HtmlTextHelper
  include ContextModuleItem
  include SearchTermHelper
  include Submittable
  include Plannable
  include MasterCourses::Restrictor
  include DuplicatingObjects
  include LockedFor
  include DatesOverridable

  REQUIRED_CHECKPOINT_COUNT = 2

  restrict_columns :content, [:title, :message]
  restrict_columns :settings, %i[require_initial_post
                                 discussion_type
                                 assignment_id
                                 pinned
                                 locked
                                 allow_rating
                                 only_graders_can_rate
                                 sort_by_rating
                                 group_category_id]
  restrict_columns :state, [:workflow_state]
  restrict_columns :availability_dates, %i[unlock_at delayed_post_at lock_at]
  restrict_assignment_columns

  attr_writer :can_unpublish, :preloaded_subentry_count, :sections_changed, :overrides_changed
  attr_accessor :user_has_posted, :saved_by, :total_root_discussion_entries, :notify_users

  module DiscussionTypes
    SIDE_COMMENT = "side_comment"
    NOT_THREADED = "not_threaded"
    THREADED     = "threaded"
    FLAT         = "flat"
    TYPES        = DiscussionTypes.constants.map { |c| DiscussionTypes.const_get(c) }
  end

  module SortOrder
    DESC = "desc"
    ASC = "asc"
    DEFAULT = DESC
    # Inherit is not a real sort order but a placeholder meaning participant's should follow discussion's
    INHERIT = "inherit"
    TYPES = SortOrder.constants.map { |c| SortOrder.const_get(c) }
  end

  DEFAULT_EXPANDED_STATE = false

  module Errors
    class LockBeforeDueDate < StandardError; end
  end

  attr_readonly :context_id, :context_type, :user_id, :is_anonymous_author

  has_many :discussion_entries, -> { order(:created_at) }, dependent: :destroy, inverse_of: :discussion_topic
  has_many :discussion_entry_drafts, dependent: :destroy, inverse_of: :discussion_topic
  has_many :rated_discussion_entries,
           lambda {
             order(
               Arel.sql("COALESCE(parent_id, 0)"), Arel.sql("COALESCE(rating_sum, 0) DESC"), :created_at
             )
           },
           class_name: "DiscussionEntry"
  has_many :root_discussion_entries, -> { preload(:user).where("discussion_entries.parent_id IS NULL AND discussion_entries.workflow_state<>'deleted'") }, class_name: "DiscussionEntry"
  has_one :external_feed_entry, as: :asset
  belongs_to :root_account, class_name: "Account"
  belongs_to :external_feed
  belongs_to :context, polymorphic: [:course, :group]
  belongs_to :attachment
  belongs_to :editor, class_name: "User"
  belongs_to :root_topic, class_name: "DiscussionTopic"
  belongs_to :group_category
  has_many :sub_assignments, through: :assignment
  has_many :child_topics, class_name: "DiscussionTopic", foreign_key: :root_topic_id, dependent: :destroy, inverse_of: :root_topic
  has_many :discussion_topic_participants, dependent: :destroy
  has_many :discussion_entry_participants, through: :discussion_entries
  has_many :discussion_topic_section_visibilities,
           lambda {
             where("discussion_topic_section_visibilities.workflow_state<>'deleted'")
           },
           inverse_of: :discussion_topic,
           dependent: :destroy
  has_many :course_sections, through: :discussion_topic_section_visibilities, dependent: :destroy
  belongs_to :user
  has_one :master_content_tag, class_name: "MasterCourses::MasterContentTag", inverse_of: :discussion_topic
  has_many :summaries, class_name: "DiscussionTopicSummary"
  has_many :insights, class_name: "DiscussionTopicInsight"
  has_many :insight_entries, class_name: "DiscussionTopicInsight::Entry"
  has_one :estimated_duration, dependent: :destroy, inverse_of: :discussion_topic

  validates_with HorizonValidators::DiscussionsValidator, if: -> { context.is_a?(Course) && context.horizon_course? }
  validates_associated :discussion_topic_section_visibilities
  validates :context_id, :context_type, presence: true
  validates :discussion_type, inclusion: { in: DiscussionTypes::TYPES }
  validates :message, length: { maximum: maximum_long_text_length, allow_blank: true }
  validates :title, length: { maximum: maximum_string_length, allow_nil: true }
  # For our users, when setting checkpoints, the value must be between 1 and 10.
  # But we also allow 0 when there are no checkpoints.
  validates :reply_to_entry_required_count, presence: true, numericality: { greater_than_or_equal_to: 0, less_than_or_equal_to: 10 }
  validates :reply_to_entry_required_count, numericality: { greater_than: 0 }, if: -> { reply_to_entry_checkpoint.present? }
  validate :validate_draft_state_change, if: :workflow_state_changed?
  validate :section_specific_topics_must_have_sections
  validate :only_course_topics_can_be_section_specific
  validate :assignments_cannot_be_section_specific
  validate :course_group_discussion_cannot_be_section_specific
  validate :collapsed_not_enforced

  sanitize_field :message, CanvasSanitize::SANITIZE
  copy_authorized_links(:message) { [context, nil] }
  acts_as_list scope: { context: self, pinned: true }

  before_create :initialize_last_reply_at
  before_create :set_root_account_id
  before_save :default_values
  before_save :set_schedule_delayed_transitions
  before_save :set_edited_at
  after_save :update_assignment
  after_save :update_subtopics
  after_save :touch_context
  after_save :schedule_delayed_transitions
  after_save :update_materialized_view_if_changed
  after_save :recalculate_progressions_if_sections_changed
  after_save :sync_attachment_with_publish_state
  after_update :clear_non_applicable_stream_items
  after_create :create_participant
  after_create :create_materialized_view

  include SmartSearchable
  use_smart_search title_column: :title,
                   body_column: :message,
                   index_scope: ->(course) { course.discussion_topics.active },
                   search_scope: ->(course, user) { DiscussionTopic::ScopedToUser.new(course, user, course.discussion_topics.active).scope }

### address_book_context_for

This Method is used to help the messageable user calculator narrow down the scope of users to filter.
  # After the scope is narrowed down , the calculator uses the visible_for? method to reject users without visibility permissions

### threaded

If section overrides are present
    if only_visible_to_overrides && !all_assignment_overrides.active.where.not(set_type: "CourseSection").exists?
      # Get all section overrides for the topic
      section_overrides = all_assignment_overrides.active.where(set_type: "CourseSection").pluck(:set_id)

      # get the sectiosn the user can see
      visible_sections_for_user = context.course_section_visibility(user)
      return [] if visible_sections_for_user == :none

      # If a user can see alls ections, then just return section_overrides for the topic
      section_overrides_and_visibility = (visible_sections_for_user == :all) ? section_overrides : (visible_sections_for_user & section_overrides) # return a list of sections that the user can see
      CourseSection.where(id: section_overrides_and_visibility)
    elsif is_section_specific?
      sections_for(user)
    else
      context
    end
  end

### schedule_delayed_transitions

either changed sections or undid section specificness
    return unless is_section_specific? ? @sections_changed : is_section_specific_before_last_save

    self.class.connection.after_transaction_commit do
      if context_module_tags.preload(:context_module).exists?
        context_module_tags.map(&:context_module).uniq.each do |cm|
          cm.invalidate_progressions
          cm.touch
        end
      end
    end
  end

### sync_attachment_with_publish_state

need to clear these in case we do a save whilst saving (e.g.
    # Announcement#respect_context_lock_rules), so as to avoid the dreaded
    # double delayed job ಠ_ಠ
    @should_schedule_delayed_post = nil
  end

### ensure_child_topic_for

delete any lingering child topics
      DiscussionTopic.where(root_topic_id: self).where.not(id: sub_topics).update_all(workflow_state: "deleted")
    end
  end

### recalculate_context_module_actions

prevent future syncs from recreating the deleted assignment
      if is_child_content?
        old_assignment.submission_types = "none"
        own_tag = MasterCourses::ChildContentTag.find_by(content: self)
        own_tag&.child_subscription&.create_content_tag_for!(old_assignment, downstream_changes: ["workflow_state"])
      end
    elsif assignment && @saved_by != :assignment && !root_topic_id
      deleted_assignment = assignment.deleted?
      sync_assignment
      assignment.workflow_state = "published" if is_announcement && deleted_assignment
      assignment.description = message
      if saved_change_to_group_category_id?
        assignment.validate_assignment_overrides(force_override_destroy: true)
      end
      assignment.save
    end

    # make sure that if the topic has a new assignment (either by going from
    # ungraded to graded, or from one assignment to another; we ignore the
    # transition from graded to ungraded) we acknowledge that the users that
    # have posted have contributed to the topic and that course paces are up
    # to date
    if assignment_id && saved_change_to_assignment_id?
      recalculate_context_module_actions!
      context_module_tags.find_each(&:update_course_pace_module_items)
    end
  end
  protected :update_assignment

### discussion_subentries

only the root level entries

### discussion_subentry_count

count of all active discussion_entries

### group_category_deleted_with_entries

kick off building of the view
    self.class.connection.after_transaction_commit do
      DiscussionTopic::MaterializedView.for(self).update_materialized_view(xlog_location: self.class.current_xlog_location)
    end
  end

### duplicate_base_model

This is a guess of what to copy over.

### duplicate

Presumes that self has no parents
  # Does not duplicate the child topics; the hooks take care of that for us.

### read_state

Don't clone a new record
    return self if new_record?

    default_opts = {
      duplicate_assignment: true,
      copy_title: nil,
      user: nil
    }
    opts_with_default = default_opts.merge(opts)
    copy_title =
      opts_with_default[:copy_title] || get_copy_title(self, t("Copy"), title)
    result = duplicate_base_model(copy_title, opts_with_default)

    # Start with a position guaranteed to not conflict with existing ones.
    # Clients are encouraged to set the correct position later on and do
    # an insert_at upon save.

    if pinned
      result.position = context.discussion_topics.active.where(pinned: true).maximum(:position) + 1
    end

    if assignment && opts_with_default[:duplicate_assignment]
      result.assignment = assignment.duplicate({
                                                 duplicate_discussion_topic: false,
                                                 copy_title: result.title,
                                                 discussion_topic_for_checkpoints: result
                                               })
    end

    result.discussion_topic_section_visibilities = []
    if is_section_specific
      original_visibilities = discussion_topic_section_visibilities.active
      original_visibilities.each do |visibility|
        new_visibility = DiscussionTopicSectionVisibility.new(
          discussion_topic: result,
          course_section: visibility.course_section
        )
        result.discussion_topic_section_visibilities << new_visibility
      end
    end

    # For some reason, the relation doesn't take care of this for us. Don't understand why.
    # Without this line, *two* discussion topic duplicates appear when a save is performed.
    result.assignment&.discussion_topic = result

    result
  end

  # If no join record exists, assume all discussion enrties are unread, and
  # that a join record will be created the first time one is marked as read.
  attr_accessor :current_user

### default_unread_count

if workflow_state is unread, and force_read_state is not provided then
    # mark everything as unread but use the defaults, or allow other entries to
    # be implicitly unread, but still update any existing records.
    if new_state == "unread" && !update_fields.key?(:forced_read_state)
      DiscussionEntryParticipant.where(discussion_entry_id: discussion_entries.select(:id), user: current_user)
                                .where.not(workflow_state: new_state)
                                .in_batches.update_all(update_fields)
    else
      DiscussionEntryParticipant.upsert_for_topic(self,
                                                  current_user,
                                                  new_state:,
                                                  forced: update_fields[:forced_read_state])
    end

    update_or_create_participant(current_user:,
                                 new_state:,
                                 new_count: (new_state == "unread") ? default_unread_count : 0)
  end
  protected :update_participants_read_state

### unread_count

Do not use the lock options unless you truly need
  # the lock, for instance to update the count.
  # Careless use has caused database transaction deadlocks

### subscription_hold

Cases where you CAN'T subscribe:
  #  - initial post is required and you haven't made one
  #  - it's an announcement
  #  - this is a root level graded group discussion and you aren't in any of the groups
  #  - this is group level discussion and you aren't in the group

### subscribe

if there is no explicit subscription, assume the author and posters
        # are subscribed, everyone else is not subscribed
        (current_user == user || participant.discussion_topic.posters.include?(current_user)) && !participant.discussion_topic.subscription_hold(current_user, nil)
      else
        participant.subscribed
      end
    else
      current_user == user && !subscription_hold(current_user, nil)
    end
  end

### available_from

Retrieves all the *course* (as oppposed to group) discussion topics that apply
  # to the given sections.  Group topics will not be returned.  TODO: figure out
  # a good way to deal with group topics here.
  #
  # Takes in an array of section objects, and it is required that they all belong
  # to the same course.  At least one section must be provided.
  scope :in_sections, lambda { |course_sections|
    course_ids = course_sections.pluck(:course_id).uniq
    if course_ids.length != 1
      raise QueryError, I18n.t("Searching for announcements in sections must span exactly one course")
    end

    course_id = course_ids.first
    joins("LEFT OUTER JOIN #{DiscussionTopicSectionVisibility.quoted_table_name}
           AS discussion_section_visibilities ON discussion_topics.is_section_specific = true AND
           discussion_section_visibilities.discussion_topic_id = discussion_topics.id")
      .where("discussion_topics.context_type = 'Course' AND
             discussion_topics.context_id = :course_id",
             { course_id: })
      .where("discussion_section_visibilities.id IS null OR
             (discussion_section_visibilities.workflow_state = 'active' AND
              discussion_section_visibilities.course_section_id IN (:course_sections))",
             { course_sections: course_sections.pluck(:id) }).distinct
  }

  scope :discussion_topic_section_visibility_scope, lambda { |student|
    DiscussionTopicSectionVisibility
      .select(1)
      .active
      .where("discussion_topic_section_visibilities.discussion_topic_id = discussion_topics.id")
      .where(
        Enrollment.active_or_pending.where(user_id: student)
          .select(1)
          .where("enrollments.course_section_id = discussion_topic_section_visibilities.course_section_id")
          .limit(1)
          .arel.exists
      )
  }

  scope :visible_to_student_sections, lambda { |student|
    merge(
      DiscussionTopic.where.not(discussion_topics: { context_type: "Course" })
      .or(DiscussionTopic.where(discussion_topics: { is_section_specific: false }))
      .or(DiscussionTopic.where(discussion_topic_section_visibility_scope(student).arel.exists))
    )
  }

  scope :visible_to_ungraded_discussion_student_visibilities, lambda { |users, courses = nil|
    observed_student_ids = []
    visible_topic_ids = []

    Array(courses).each do |course|
      course = Course.find(course) unless course.is_a?(Course)

      if course&.user_has_been_observer?(users)
        observed_student_ids.concat(ObserverEnrollment.observed_student_ids(course, users))
      end

      if User.observing_full_course(course).where(id: users).exists?
        visible_topic_ids.concat(DiscussionTopic.where(context_type: "Course", context_id: course.id).active.pluck(:id))
      end
    end

    user_ids = Array(users) | observed_student_ids
    visible_differentiated_topic_ids = UngradedDiscussionVisibility::UngradedDiscussionVisibilityService.discussion_topics_visible(user_ids:).map(&:discussion_topic_id)
    merge(DiscussionTopic.where.not(context_type: "Course")
    .or(DiscussionTopic.where(id: visible_topic_ids))
    .or(DiscussionTopic.where(id: visible_differentiated_topic_ids, is_section_specific: false))
    .or(DiscussionTopic.where(is_section_specific: true).where(discussion_topic_section_visibility_scope(user_ids).arel.exists)))
  }

  scope :recent, -> { where("discussion_topics.last_reply_at>?", 2.weeks.ago).order("discussion_topics.last_reply_at DESC") }
  scope :only_discussion_topics, -> { where(type: nil) }
  scope :for_subtopic_refreshing, -> { where("discussion_topics.subtopics_refreshed_at IS NOT NULL AND discussion_topics.subtopics_refreshed_at<discussion_topics.updated_at").order("discussion_topics.subtopics_refreshed_at") }
  scope :active, -> { where("discussion_topics.workflow_state<>'deleted'") }
  scope :for_context_codes, ->(codes) { where(context_code: codes) }

  scope :before, ->(date) { where("discussion_topics.created_at<?", date) }

  scope :by_position, -> { order("discussion_topics.position ASC, discussion_topics.created_at DESC, discussion_topics.id DESC") }
  scope :by_position_legacy, -> { order("discussion_topics.position DESC, discussion_topics.created_at DESC, discussion_topics.id DESC") }
  scope :by_last_reply_at, -> { order("discussion_topics.last_reply_at DESC, discussion_topics.created_at DESC, discussion_topics.id DESC") }

  scope :by_posted_at, lambda {
    order(Arel.sql(<<~SQL.squish))
      COALESCE(discussion_topics.unlock_at, discussion_topics.delayed_post_at, discussion_topics.posted_at, discussion_topics.created_at) DESC,
      discussion_topics.created_at DESC,
      discussion_topics.id DESC
    SQL
  }

  scope :read_for, lambda { |user|
    eager_load(:discussion_topic_participants)
      .where("discussion_topic_participants.id IS NOT NULL
          AND (discussion_topic_participants.user_id = :user
            AND discussion_topic_participants.workflow_state = 'read')",
             user:)
  }
  scope :unread_for, lambda { |user|
    joins(sanitize_sql(["LEFT OUTER JOIN #{DiscussionTopicParticipant.quoted_table_name} ON
            discussion_topic_participants.discussion_topic_id=discussion_topics.id AND
            discussion_topic_participants.user_id=?",
                        user.id]))
      .where("discussion_topic_participants IS NULL
          OR discussion_topic_participants.workflow_state <> 'read'
          OR discussion_topic_participants.unread_entry_count > 0")
  }
  scope :published, -> { where("discussion_topics.workflow_state = 'active'") }
  scope :published_or_post_delayed, -> { where("discussion_topics.workflow_state = 'active' OR discussion_topics.workflow_state = 'post_delayed'") }

  # TODO: this scope is appearing in a few models now with identical code.
  # Can this be extracted somewhere?
  scope :starting_with_title, lambda { |title|
    where("title ILIKE ?", "#{title}%")
  }

  alias_attribute :available_until, :lock_at

### should_not_post_yet

not assignment or vdd aware! only use this to check the topic's own field!
    # you should be checking other lock statuses in addition to this one
    lock_at && lock_at < Time.now.utc
  end
  alias_method :not_available_anymore?, :should_lock_yet

### update_based_on_date

not assignment or vdd aware! only use this to check the topic's own field!
    # you should be checking other lock statuses in addition to this one
    delayed_post_at && delayed_post_at > Time.now.utc
  end
  alias_method :not_available_yet?, :should_not_post_yet

  # There may be delayed jobs that expect to call this to update the topic, so be sure to alias
  # the old method name if you change it
  # Also: if this method is scheduled by a blueprint sync, ensure it isn't counted as a manual downstream change

### active

with draft state, this means published. without, unpublished. so we really do support both events
    end
    state :deleted
  end

### publish

using state instead of workflow_state so this works with new records
    state == :active || (!is_announcement && state == :post_delayed)
  end

### publish

follows the logic of setting post_delayed in other places of this file
    self.workflow_state = (delayed_post_at && delayed_post_at > Time.zone.now) ? "post_delayed" : "active"
    self.last_reply_at = Time.zone.now
    self.posted_at = Time.zone.now
  end

### send_items_to_stream

This is manually called for module publishing

### unlink

Not restorable if the root topic context is a course and
    # root topic is deleted.
    !(root_topic&.context_type == "Course" && root_topic&.deleted?)
  end

### self

Users may have can :read, but should not have access to all the data
    # because the topic is locked_for?(user)
    given { |user| visible_for?(user) }
    can :read

    given { |user| grants_right?(user, :read) }
    can :read_replies

    given { |user| self.user && self.user == user && visible_for?(user) && !locked_for?(user, check_policies: true) && can_participate_in_course?(user) && !comments_disabled? }
    can :reply

    given { |user| self.user && self.user == user && available_for?(user) && context.user_can_manage_own_discussion_posts?(user) && context.grants_right?(user, :participate_as_student) }
    can :update

    given { |user| self.user && self.user == user and discussion_entries.active.empty? && available_for?(user) && !root_topic_id && context.user_can_manage_own_discussion_posts?(user) && context.grants_right?(user, :participate_as_student) }
    can :delete

    given do |user, session|
      !locked_for?(user, check_policies: true) &&
        context.grants_right?(user, session, :post_to_forum) && visible_for?(user) && can_participate_in_course?(user) && !comments_disabled?
    end
    can :reply

    given { |user, session| user_can_create(user, session) }
    can :create

    given { |user, session| user_can_create(user, session) && user_can_duplicate(user, session) }
    can :duplicate

    given { |user, session| context.respond_to?(:allow_student_forum_attachments) && context.allow_student_forum_attachments && context.grants_any_right?(user, session, :create_forum, :post_to_forum) }
    can :attach

    given { course.student_reporting? }
    can :student_reporting

    given { |user, session| !root_topic_id && context.grants_all_rights?(user, session, :read_forum, :moderate_forum) && available_for?(user) }
    can :update and can :read_as_admin and can :delete and can :create and can :read and can :attach

    # Moderators can still modify content even in unavailable topics (*especially* unlocking them)
    given { |user, session| !root_topic_id && context.grants_all_rights?(user, session, :read_forum, :moderate_forum) }
    can :update and can :read_as_admin and can :delete and can :read and can :attach

    given { |user, session| root_topic&.grants_right?(user, session, :read_as_admin) }
    can :read_as_admin

    given { |user, session| root_topic&.grants_right?(user, session, :delete) }
    can :delete

    given { |user, session| root_topic&.grants_right?(user, session, :read) }
    can :read

    given { |user, session| context.grants_all_rights?(user, session, :moderate_forum, :read_forum) }
    can :moderate_forum

    given do |user, session|
      allow_rating && (!only_graders_can_rate ||
                            course.grants_right?(user, session, :manage_grades))
    end
    can :rate

    given do |user, session|
      next false unless user && context.is_a?(Course) && context.grants_right?(user, session, :moderate_forum)

      if assignment_id
        context.grants_right?(user, session, :manage_assignments_edit)
      else
        context.user_is_admin?(user) || context.account_membership_allows(user) || !context.visibility_limited_to_course_sections?(user)
      end
    end
    can :manage_assign_to

    given do |user, session|
      next false unless user && context.is_a?(Course) && context.grants_right?(user, session, :create_forum)

      if assignment_id
        context.grants_right?(user, session, :manage_assignments_add)
      else
        context.user_is_admin?(user) || context.account_membership_allows(user) || !context.visibility_limited_to_course_sections?(user)
      end
    end
    can :create_assign_to
  end

### user_can_access_insights

course can be an account in case the topic context is group
    # and the group context is account
    unless course.is_a?(Course)
      return false
    end

    course.feature_enabled?(:discussion_summary) && (
      course.user_is_instructor?(user) || course.grants_right?(user, :read_as_admin)
    )
  end

### discussion_topic_id

course can be an account in case the topic context is group
    # and the group context is account
    unless course.is_a?(Course)
      return false
    end

    course.feature_enabled?(:discussion_insights) && (
      course.user_is_instructor?(user) || course.grants_right?(user, :read_as_admin)
    )
  end

### users_with_section_visibility

From the given list of users, return those that are permitted to see the section
  # of the topic.  If the topic is not section specific this just returns the
  # original list.

### participants

Context is known to be a course here
    users_in_sections = context.enrollments.active_or_pending
                               .where(user_id: user_ids, course_section_id: section_ids).pluck(:user_id).to_set
    unlocked_teachers = context.enrollments.active_or_pending.instructor
                               .where(limit_privileges_to_course_section: false, user_id: user_ids)
                               .pluck(:user_id).to_set
    permitted_user_ids = users_in_sections.union(unlocked_teachers)
    non_nil_users.select { |u| permitted_user_ids.include?(u.id) }
  end

### participating_users

specific user
      if override.adhoc?
        adhoc_users = users_with_visibility.concat(override.assignment_override_students.pluck(:user_id))
        users_with_visibility.concat(adhoc_users)
      elsif override.course_section?
        users_in_section = User.joins(:enrollments).where(enrollments: { course_section_id: override.set_id }).pluck(:id)
        users_with_visibility.concat(users_in_section)
      end
    end

    admin_ids = course.participating_admins.pluck(:id)
    users_with_visibility.concat(admin_ids)
    users_with_visibility.uniq!

    # observers will not be returned, which is okay for the functions current use cases (but potentially not others)
    active_participants_include_tas_and_teachers.select { |p| users_with_visibility.include?(p.id) }
  end

### filter_message_users

this duplicates some logic from #subscribed? so we don't have to call
    # #posters for each legacy subscriber.
    sub_ids = discussion_topic_participants.where(subscribed: true).pluck(:user_id)
    legacy_sub_ids = discussion_topic_participants.where(subscribed: nil).pluck(:user_id)
    poster_ids = posters.map(&:id)
    legacy_sub_ids &= poster_ids
    sub_ids += legacy_sub_ids

    subscribed_users = participating_users(sub_ids).to_a

    filter_message_users(subscribed_users)
  end

### posters

an observer with no students or one with students who have visibility
          (observed_students[user.id] && (observed_students[user.id] == [] || observed_students[user.id].intersect?(students_with_visibility)))
      end
    end
    users
  end

### visible_for

Public: Determine if the given user can view this discussion topic.
  #
  # user - The user attempting to view the topic (default: nil).
  #
  # Returns a boolean.

### can_participate_in_course

user is the topic's author
      next true if user && user.id == user_id

      next false unless context
      next false unless is_announcement ? context.grants_right?(user, :read_announcements) : context.grants_right?(user, :read_forum)

      # Don't have visibilites for any of the specific sections in a section specific topic
      if context.is_a?(Course) && try(:is_section_specific)
        section_visibilities = context.course_section_visibility(user)
        next false if section_visibilities == :none

        if section_visibilities != :all
          course_section_ids = shard.activate { course_sections.ids }

          next false unless section_visibilities.intersect?(course_section_ids)
        end
      end
      # Verify that section limited teachers/ta's are properly restricted
      if context.is_a?(Course) && (!visible_to_everyone && context.user_is_instructor?(user))

        section_overrides = assignment_overrides.active.where(set_type: "CourseSection").pluck(:set_id)
        visible_sections_for_user = context.course_section_visibility(user)
        next false if visible_sections_for_user == :none

        # If there are no section_overrides, then no check for section_specific instructor roles is needed
        if visible_sections_for_user != :all && section_overrides.any?
          next false unless visible_sections_for_user.intersect?(section_overrides)
        end
      end
      # user is an admin in the context (teacher/ta/designer) OR
      # user is an account admin with appropriate permission
      next true if context.grants_any_right?(user, :manage, :read_course_content)

      # assignment exists and isn't assigned to user (differentiated assignments)
      if for_assignment?
        next false unless assignment.visible_to_user?(user)
      # Announcements can be section specific, but that is already handled above.
      # Eventually is_section_specific will be replaced with assignment overrides, and then announcements will need to be handled
      elsif !is_announcement
        next false unless visible_to_user?(user)
      end

      # topic is not published
      next false unless published?

      # unlock_at and lock_at determine visibility for announcements
      if is_announcement
        next false if lock_at && Time.now.utc > lock_at
        next false if unlock_at && unlock_at > Time.now.utc
      end

      next true
    end
  end

### low_level_locked_for

this probably isn't a perfect way to determine this but I can't think of a better one
      course.enrollments.for_user(user).active_by_date.exists? || course.grants_right?(user, :read_as_admin)
    else
      true
    end
  end

  # Determine if the discussion topic is locked for a user. The topic is locked
  # if the delayed_post_at is in the future or the assignment is locked.
  # This does not determine the visibility of the topic to the user,
  # only that they are unable to reply and unable to see the message.
  # Generally you want to call :locked_for?(user, check_policies: true), which
  # will call this method.

### show_in_search_for_user

get the topic's overridden availability dates for the user. If graded, the dates will be on the assignment.
      topic_for_user = assignment.present? ? assignment.overridden_for(user) : overridden_for(user)
      # prefer unlock_at, but fall back to delayed_post_at for DiscussionTopics until the latter is removed
      overridden_unlock_at = topic_for_user.unlock_at
      overridden_unlock_at ||= topic_for_user.delayed_post_at if topic_for_user.respond_to?(:delayed_post_at)
      overridden_lock_at = topic_for_user.lock_at
      if overridden_unlock_at && overridden_unlock_at > Time.zone.now
        locked = { object: self, unlock_at: overridden_unlock_at }
      elsif overridden_lock_at && overridden_lock_at < Time.zone.now
        locked = { object: self, lock_at: overridden_lock_at, can_view: true }
      elsif could_be_locked && (item = locked_by_module_item?(user, opts))
        locked = { object: self, module: item.context_module }
      elsif locked? # nothing more specific, it's just locked
        locked = { object: self, can_view: true }
      elsif (l = root_topic&.low_level_locked_for?(user, opts)) # rubocop:disable Lint/DuplicateBranch
        locked = l
      end
      locked
    end
  end

### materialized_view

returns the materialized view of the discussion as structure, participant_ids, and entry_ids
  # the view is already converted to a json string, the other two arrays of ids are ruby arrays
  # see the description of the format in the discussion topics api documentation.
  #
  # returns nil if the view is not currently available, and kicks off a
  # background job to build the view. this typically only takes a couple seconds.
  #
  # if a new message is posted, it won't appear in this view until the job to
  # update it completes. so this view is eventually consistent.
  #
  # if the topic itself is not yet created, it will return blank data. this is for situations
  # where we're creating topics on the first write - until that first write, we need to return
  # blank data on reads.

### create_materialized_view

synchronously create/update the materialized view

### set_edited_at

Discussions with an assignment: pluck id, assignment_id, and user_id from items joined with the SQL view
    visible_assignments = AssignmentVisibility::AssignmentVisibilityService.assignments_visible_to_students(user_ids: opts[:user_id], course_ids: opts[:course_id])
    # map the visibilities to a hash of assignment_id => [user_ids]
    assignment_user_map = visible_assignments.each_with_object(Hash.new { |hash, key| hash[key] = [] }) do |visibility, hash|
      hash[visibility.assignment_id] << visibility.user_id
    end
    # this mimicks the format of the non-flagged group_by to pair each user_id to the correct visible discussion/discussion's assignment
    plucked_visibilities = where(assignment_id: assignment_user_map.keys)
                           .pluck(:id, :assignment_id)
                           .flat_map { |discussion_id, assignment_id| assignment_user_map[assignment_id].map { |user_id| [discussion_id, assignment_id, user_id] } }
                           .group_by { |_, _, user_id| user_id }

    # Initialize dictionaries for different visibility scopes
    ids_visible_to_all = []

    # Get Section specific discussions:
    sections_per_user = {}
    Enrollment.active.where(course_id: opts[:course_id], user_id: opts[:user_id])
              .pluck(:user_id, :course_section_id)
              .each { |user_id, section_id| (sections_per_user[user_id] ||= Set.new) << section_id }

    # build hash of section_ids to array of visible topic ids
    all_section_ids = sections_per_user.values.reduce([]) { |all_ids, section_ids| all_ids.concat(section_ids.to_a) }
    topic_ids_per_section = {}
    DiscussionTopicSectionVisibility.active.where(course_section_id: all_section_ids)
                                    .pluck(:course_section_id, :discussion_topic_id)
                                    .each { |section_id, topic_id| (topic_ids_per_section[section_id] ||= Set.new) << topic_id }
    topic_ids_per_section.each { |section_id, topic_ids| topic_ids_per_section[section_id] = topic_ids.to_a }

    # finally, build hash of user_ids to array of visible topic ids
    topic_ids_per_user = {}
    opts[:user_id].each { |user_id| topic_ids_per_user[user_id] = sections_per_user[user_id]&.map { |section_id| topic_ids_per_section[section_id] }&.flatten&.uniq&.compact }
    ids_visible_to_sections = topic_ids_per_user

    visible_topic_user_id_pairs = UngradedDiscussionVisibility::UngradedDiscussionVisibilityService.discussion_topics_visible(user_ids: opts[:user_id], course_ids: opts[:course_id]).map { |visibility| [visibility.discussion_topic_id, visibility.user_id] }
    eligible_topic_ids = DiscussionTopic.where(id: visible_topic_user_id_pairs.map(&:first)).where(assignment_id: nil).where.not(is_section_specific: true).pluck(:id)
    eligible_visible_topic_user_id_pairs = visible_topic_user_id_pairs.select { |discussion_topic_id, _user_id| eligible_topic_ids.include?(discussion_topic_id) } # rubocop:disable Style/HashSlice
    ungraded_differentiated_topic_ids_per_user = eligible_visible_topic_user_id_pairs.group_by(&:last).transform_values { |pairs| pairs.map(&:first) }

    # build map of user_ids to array of item ids {1 => [2,3,4], 2 => [2,4]}
    opts[:user_id].index_with do |student_id|
      assignment_item_ids = (plucked_visibilities[student_id] || []).map { |id, _, _| id }
      section_specific_ids = ids_visible_to_sections[student_id] || []
      ungraded_differentiated_specific_ids = ungraded_differentiated_topic_ids_per_user[student_id] || []
      assignment_item_ids.concat(ids_visible_to_all).concat(section_specific_ids).concat(ungraded_differentiated_specific_ids)
    end
  end

### sort_order_for_user

this is a temporary check for any discussion_topic_section_visibilities until we eventually backfill that table
    if is_section_specific
      section_overrides = assignment_overrides.active.where(set_type: "CourseSection").select(:set_id)
      section_visibilities = discussion_topic_section_visibilities.active.where.not(course_section_id: section_overrides)
    end

    if section_visibilities
      section_overrides = section_visibilities.map do |section_visibility|
        assignment_override = AssignmentOverride.new(
          discussion_topic: section_visibility.discussion_topic,
          course_section: section_visibility.course_section
        )
        assignment_override.unlock_at = unlock_at if unlock_at
        assignment_override.lock_at = lock_at if lock_at
        assignment_override
      end
    end

    all_overrides = overrides.to_a
    all_overrides += section_overrides if section_visibilities
    all_overrides
  end

### collapsed_not_enforced

For the current business logic we don't allow collapsed discussion locking in any scenarios

