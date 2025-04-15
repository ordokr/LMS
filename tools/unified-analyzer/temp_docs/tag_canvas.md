# ContentTag

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

- belongs_to :content
- belongs_to :context
- belongs_to :associated_asset
- belongs_to :context_module
- belongs_to :learning_outcome
- belongs_to :learning_outcome_content
- has_many :learning_outcome_results
- belongs_to :root_account
- has_one :estimated_duration

## Methods

### touch_context_module

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
class ContentTag < ActiveRecord::Base
  include Lti::Migratable

  class LastLinkToOutcomeNotDestroyed < StandardError
  end

  TABLED_CONTENT_TYPES = ["Attachment",
                          "Assignment",
                          "WikiPage",
                          "Quizzes::Quiz",
                          "LearningOutcome",
                          "DiscussionTopic",
                          "Rubric",
                          "ContextExternalTool",
                          "LearningOutcomeGroup",
                          "AssessmentQuestionBank",
                          "LiveAssessments::Assessment",
                          "Lti::MessageHandler"].freeze
  TABLELESS_CONTENT_TYPES = ["ContextModuleSubHeader", "ExternalUrl"].freeze
  CONTENT_TYPES = (TABLED_CONTENT_TYPES + TABLELESS_CONTENT_TYPES).freeze
  HAS_ITS_OWN_ESTIMATED_DURATION = ["Attachment", "Assignment", "WikiPage", "Quizzes::Quiz", "DiscussionTopic"].freeze
  CONTENT_TAG_ESTIMATED_DURATION = ["ContextExternalTool", "ExternalUrl"].freeze
  HAS_ESTIMATED_DURATION = (HAS_ITS_OWN_ESTIMATED_DURATION + CONTENT_TAG_ESTIMATED_DURATION).freeze

  include Workflow
  include SearchTermHelper

  include MasterCourses::Restrictor
  restrict_columns :state, [:workflow_state]
  restrict_columns :content, %i[content_id url new_tab]

  belongs_to :content, polymorphic: [], exhaustive: false
  validates :content_type, inclusion: { allow_nil: true, in: CONTENT_TYPES }
  belongs_to :context, polymorphic:
      [:course,
       :learning_outcome_group,
       :assignment,
       :account,
       { quiz: "Quizzes::Quiz" }]
  belongs_to :associated_asset,
             polymorphic: [:learning_outcome_group, lti_resource_link: "Lti::ResourceLink"],
             polymorphic_prefix: true
  belongs_to :context_module
  belongs_to :learning_outcome
  # This allows doing a has_many_through relationship on ContentTags for linked LearningOutcomes. (see LearningOutcomeContext)
  belongs_to :learning_outcome_content, class_name: "LearningOutcome", foreign_key: :content_id, inverse_of: false
  has_many :learning_outcome_results
  belongs_to :root_account, class_name: "Account"
  has_one :estimated_duration, dependent: :destroy, inverse_of: :content_tag

  after_create :clear_stream_items_if_module_is_unpublished

  # This allows bypassing loading context for validation if we have
  # context_id and context_type set, but still allows validating when
  # context is not yet saved.
  validates :context, presence: { unless: proc { |tag| tag.context_id && tag.context_type } }
  validates :workflow_state, presence: true
  validates :comments, length: { maximum: maximum_text_length, allow_blank: true }
  before_save :associate_external_tool
  before_save :default_values
  before_save :set_root_account
  before_save :update_could_be_locked
  after_save :touch_context_module_after_transaction
  after_save :touch_context_if_learning_outcome
  after_save :run_submission_lifecycle_manager_for_quizzes_next
  after_save :clear_discussion_stream_items
  after_save :send_items_to_stream
  after_save :clear_total_outcomes_cache
  after_save :update_course_pace_module_items
  after_save :update_module_item_submissions
  after_create :update_outcome_contexts

  include CustomValidations
  validates_as_url :url

  validate :check_for_restricted_content_changes

  acts_as_list scope: :context_module

  set_policy do
    given do |user, session|
      user && context&.grants_right?(user, session, :manage_course_content_delete)
    end
    can :delete
  end

  workflow do
    state :active do
      event :unpublish, transitions_to: :unpublished
    end
    state :unpublished do
      event :publish, transitions_to: :active
    end
    state :deleted
  end

  alias_method :published?, :active?

  scope :active, -> { where(workflow_state: "active") }
  scope :not_deleted, -> { where("content_tags.workflow_state<>'deleted'") }
  scope :nondeleted, -> { not_deleted }
  scope :content_type, ->(type) { where(content_type: type) }
  scope :not_deleted_assignments, -> { content_type("Assignment").not_deleted }
  scope :assignments_for_modules, ->(modules) { not_deleted_assignments.where(context_module_id: modules) }
  scope :assignments_for_module_items, ->(module_items) { not_deleted_assignments.where(id: module_items) }

  attr_accessor :skip_touch
  attr_accessor :reassociate_external_tool

### self

do nothing
    else
      ContextModule.where(id: ids).not_recently_touched.touch_all
    end
    true
  end

### set_content_from_external_tool

set only when editing module item to allow changing the url,
      # which will force a lookup of the new correct tool
      # IF the url is potentially for a different tool.
      old_url_host = Addressable::URI.parse(url_was)&.host
      new_url_host = Addressable::URI.parse(url)&.host
      if old_url_host != new_url_host
        set_content_from_external_tool
      end

      return
    end

    # happy path
    return if content.present?

    set_content_from_external_tool
  end

### update_asset_workflow_state

Assignment proxies name= and name to title= and title, which breaks the asset_safe_title logic
    if content.respond_to?(:name=) && content.respond_to?(:name) && !content.is_a?(Assignment)
      content.name = asset_safe_title("name")
    elsif content.respond_to?(:title=)
      content.title = asset_safe_title("title")
    elsif content.respond_to?(:display_name=)
      content.display_name = asset_safe_title("display_name")
    end
    if content.changed?
      content.user = user if user && content.is_a?(WikiPage)
      content.save
    end
  end

### self

update the asset and also update _other_ content tags that point at it
    if unpublished? && content.published? && content.can_unpublish?
      content.unpublish!
      self.class.update_for(content, exclude_tag: self)
    elsif active? && !content.published?
      content.publish!
      self.class.update_for(content, exclude_tag: self)
    end
  end

### destroy

if it's a learning outcome link...
    if tag_type == "learning_outcome_association"
      # and there are no other links to the same outcome in the same context...
      outcome = content
      other_link = ContentTag.learning_outcome_links.active
                             .where(context_type:, context_id:, content_id: outcome)
                             .where.not(id: self).take
      unless other_link
        # and there are alignments to the outcome (in the link's context for
        # foreign links, in any context for native links)
        alignment_conditions = { learning_outcome_id: outcome.id }
        native = outcome.context_type == context_type && outcome.context_id == context_id
        if native
          @should_destroy_outcome = true
        else
          alignment_conditions[:context_id] = context_id
          alignment_conditions[:context_type] = context_type
        end

        @active_alignment_tags = ContentTag.learning_outcome_alignments.active.where(alignment_conditions)
        if @active_alignment_tags.exists?
          # then don't let them delete the link
          return false
        end
      end
    end
    true
  end

  alias_method :destroy_permanently!, :destroy

### locked_for

for outcome links delete the associated friendly description
    delete_outcome_friendly_description if content_type == "LearningOutcome"

    run_submission_lifecycle_manager_for_quizzes_next(force: true)
    update_module_item_submissions(change_of_module: false)

    # after deleting the last native link to an unaligned outcome, delete the
    # outcome. we do this here instead of in LearningOutcome#destroy because
    # (a) LearningOutcome#destroy *should* only ever be called from here, and
    # (b) we've already determined other_link and native
    if @should_destroy_outcome
      content.destroy
    end

    true
  end

### sync_title_to_asset_title

update title
    tag_ids = tags.select(&:sync_title_to_asset_title?).map(&:id)
    attr_hash = { updated_at: Time.now.utc }
    { display_name: :title, name: :title, title: :title }.each do |attr, val|
      attr_hash[val] = asset.send(attr) if asset.respond_to?(attr)
    end
    ContentTag.where(id: tag_ids).update_all(attr_hash) unless tag_ids.empty?

    # update workflow_state
    tag_ids = tags.select(&:sync_workflow_state_to_asset?).map(&:id)
    attr_hash = { updated_at: Time.now.utc }

    workflow_state = asset_workflow_state(asset)
    attr_hash[:workflow_state] = workflow_state if workflow_state
    ContentTag.where(id: tag_ids).update_all(attr_hash) if attr_hash[:workflow_state] && !tag_ids.empty?

    # update the module timestamp
    ContentTag.touch_context_modules(module_ids)
  end

### self

{CourseAccountAssociation.quoted_table_name} AS caa
      ON caa.course_id = content_tags.context_id AND content_tags.context_type = 'Course'
      AND caa.account_id = #{account.id}")
  }
  scope :learning_outcome_alignments, -> { where(tag_type: "learning_outcome") }
  scope :learning_outcome_links, -> { where(tag_type: "learning_outcome_association", associated_asset_type: "LearningOutcomeGroup", content_type: "LearningOutcome") }

  # Scopes For Differentiated Assignment Filtering:

  scope :visible_to_students_in_course_with_da, lambda { |user_ids, course_ids|
    differentiable_classes = ["Assignment", "DiscussionTopic", "Quiz", "Quizzes::Quiz", "WikiPage"]
    scope = for_non_differentiable_classes(course_ids, differentiable_classes)

    visible_page_ids = WikiPage.visible_to_students_in_course_with_da(user_ids, course_ids).select(:id)
    scope = scope.union(where(content_id: visible_page_ids, context_id: course_ids, context_type: "Course", content_type: "WikiPage"))

    scope.union(
      for_non_differentiable_discussions(course_ids)
        .merge(DiscussionTopic.visible_to_ungraded_discussion_student_visibilities(user_ids)),
      for_differentiable_assignments(user_ids, course_ids),
      for_differentiable_discussions(user_ids, course_ids),
      for_differentiable_quizzes(user_ids, course_ids)
    )
  }

  scope :for_non_differentiable_classes, lambda { |course_ids, differentiable_classes|
    where(context_id: course_ids, context_type: "Course").where.not(content_type: differentiable_classes)
  }

  scope :for_non_differentiable_discussions, lambda { |course_ids|
    joins("JOIN #{DiscussionTopic.quoted_table_name} as discussion_topics ON discussion_topics.id = content_tags.content_id")
      .where("content_tags.context_id IN (?)
             AND content_tags.context_type = 'Course'
             AND content_tags.content_type = 'DiscussionTopic'
             AND discussion_topics.assignment_id IS NULL",
             course_ids)
  }

  scope :for_differentiable_quizzes, lambda { |user_ids, course_ids|
    visible_quiz_ids = QuizVisibility::QuizVisibilityService.quizzes_visible_to_students(user_ids:, course_ids:).map(&:quiz_id)
    where(content_id: visible_quiz_ids, context_id: course_ids, context_type: "Course", content_type: ["Quiz", "Quizzes::Quiz"])
  }

  scope :for_differentiable_assignments, lambda { |user_ids, course_ids|
    visible_assignment_ids = AssignmentVisibility::AssignmentVisibilityService.assignments_visible_to_students(user_ids:, course_ids:).map(&:assignment_id)
    where(content_id: visible_assignment_ids, context_id: course_ids, context_type: "Course", content_type: "Assignment")
  }

  scope :for_differentiable_discussions, lambda { |user_ids, course_ids|
    unfiltered_discussion_ids = where(content_type: "DiscussionTopic").pluck(:content_id)
    assignment_ids = DiscussionTopic.where(id: unfiltered_discussion_ids).where.not(assignment_id: nil).pluck(:assignment_id)
    visible_assignment_ids = AssignmentVisibility::AssignmentVisibilityService.assignments_visible_to_students(user_ids:, course_ids:, assignment_ids:).map(&:assignment_id)
    discussion_topic_ids = DiscussionTopic.where(assignment_id: visible_assignment_ids).pluck(:id)
    joins("JOIN #{DiscussionTopic.quoted_table_name} ON discussion_topics.id = content_tags.content_id
          AND content_tags.content_type = 'DiscussionTopic'")
      .where(content_id: discussion_topic_ids, context_id: course_ids, context_type: "Course", content_type: "DiscussionTopic")
  }

  scope :can_have_assignment, -> { where(content_type: ["Assignment", "DiscussionTopic", "Quizzes::Quiz", "WikiPage"]) }

  # only intended for learning outcome links

### migrate_to_1_3_if_needed

Used to either Just-In-Time migrate a ContentTag to fully support 1.3 or
  # as part of a backfill job to migrate existing 1.3 ContentTags to fully
  # support 1.3. Fully support in this case means the associated resource link
  # has the LTI 1.1 resource_link_id stored on it. Will only migrate tags that
  # are module items that are associated with ContextExternalTools.
  # @see Lti::Migratable

### self

Updating a 1.3 module item
    if associated_asset_lti_resource_link.present? && content&.use_1_3?
      associated_asset_lti_resource_link.update!(lti_1_1_id: tool.opaque_identifier_for(self))
    # Migrating a 1.1 module item
    elsif !content&.use_1_3?
      rl = Lti::ResourceLink.create_with(context, tool, nil, url, lti_1_1_id: tool.opaque_identifier_for(self))
      update!(associated_asset: rl, content: tool)
    end
  end

  # filtered by context during migrate_content_to_1_3
  # @see Lti::Migratable

### self

filtered by context during migrate_content_to_1_3
  # @see Lti::Migratable

### self

TODO: this does not account for content tags that _are_ linked to a
    # tool and the tag has a content_id, but the content_id doesn't match
    # the current tool
    ContentTag.nondeleted.where(tag_type: :context_module, content_id: nil)
  end

  # @param [Array<Integer>] ids The IDs of the resources to fetch for this batch
  # @see Lti::Migratable

### self

@param [Integer] tool_id The ID of the LTI 1.1 tool that the resource is indirectly
  # associated with
  # @param [Array<Integer>] ids The IDs of the resources to fetch for this batch
  # @see Lti::Migratable

### set_root_account

Quizzes next should ideally only ever be attached to an
    # assignment.  Let's ignore any other contexts.
    return unless context_type == "Assignment"

    SubmissionLifecycleManager.recompute(context) if content.try(:quiz_lti?) && (force || workflow_state != "deleted")
  end

### update_module_item_submissions

Course paces takes over how and when assignment overrides are managed so if we are deleting an assignment from
      # a module we need to reset it back to an untouched state with regards to overrides.
      if deleted?
        cpmi&.destroy
        cpmi&.module_item&.assignment&.assignment_overrides&.destroy_all
      elsif !cpmi.valid?
        cpmi&.destroy
      end

      # Republish the course pace if changes were made
      course_pace.create_publish_progress if deleted? || cpmi.destroyed? || cpmi.saved_change_to_id? || saved_change_to_position?
    end
  end

