# Course

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

- belongs_to :root_account
- belongs_to :abstract_course
- belongs_to :enrollment_term
- belongs_to :template_course
- has_many :templated_courses
- has_many :templated_accounts
- has_many :block_editor_templates
- belongs_to :linked_homeroom_course
- has_many :course_sections
- has_many :active_course_sections
- has_many :moved_sections
- has_many :enrollments
- has_many :all_enrollments
- has_many :current_enrollments
- has_many :all_current_enrollments
- has_many :prior_enrollments
- has_many :prior_users
- has_many :prior_students
- has_many :participating_enrollments
- has_many :participating_students
- has_many :participating_students_by_date
- has_many :student_enrollments
- has_many :student_enrollments_including_completed
- has_many :students
- has_many :self_enrolled_students
- has_many :admin_visible_student_enrollments
- has_many :admin_visible_students
- has_many :gradable_student_enrollments
- has_many :gradable_students
- has_many :all_student_enrollments
- has_many :all_student_enrollments_including_deleted
- has_many :all_students
- has_many :all_students_including_deleted
- has_many :all_accepted_student_enrollments
- has_many :all_accepted_students
- has_many :all_real_enrollments
- has_many :all_real_users
- has_many :all_real_student_enrollments
- has_many :all_real_students
- has_many :teacher_enrollments
- has_many :teachers
- has_many :ta_enrollments
- has_many :tas
- has_many :observer_enrollments
- has_many :observers
- has_many :non_observer_enrollments
- has_many :enrollments_excluding_linked_observers
- has_many :participating_observers
- has_many :participating_observers_by_date
- has_many :instructors
- has_many :instructor_enrollments
- has_many :participating_instructors
- has_many :participating_instructors_by_date
- has_many :admins
- has_many :admin_enrollments
- has_many :participating_admins
- has_many :participating_admins_by_date
- has_many :student_view_enrollments
- has_many :student_view_students
- has_many :custom_gradebook_columns
- has_many :course_account_associations
- has_many :users
- has_many :all_users
- has_many :current_users
- has_many :all_current_users
- has_many :active_users
- has_many :user_past_lti_ids
- has_many :group_categories
- has_many :all_group_categories
- has_many :combined_group_and_differentiation_tag_categories
- has_many :active_combined_group_and_differentiation_tag_categories
- has_many :groups
- has_many :active_groups
- has_many :differentiation_tag_categories
- has_many :all_differentiation_tag_categories
- has_many :differentiation_tags
- has_many :active_differentiation_tags
- has_many :combined_groups_and_differentiation_tags
- has_many :assignment_groups
- has_many :assignments
- has_many :calendar_events
- has_many :submissions
- has_many :submission_comments
- has_many :discussion_topics
- has_many :active_discussion_topics
- has_many :all_discussion_topics
- has_many :discussion_entries
- has_many :announcements
- has_many :active_announcements
- has_many :attachments
- has_many :active_images
- has_many :active_assignments
- has_many :folders
- has_many :active_folders
- has_many :messages
- has_many :context_external_tools
- has_many :tool_proxies
- belongs_to :wiki
- has_many :wiki_pages
- has_many :wiki_page_lookups
- has_many :quizzes
- has_many :quiz_questions
- has_many :active_quizzes
- has_many :assessment_question_banks
- has_many :assessment_questions
- has_many :external_feeds
- belongs_to :grading_standard
- has_many :grading_standards
- has_many :web_conferences
- has_many :collaborations
- has_many :context_modules
- has_many :context_module_progressions
- has_many :active_context_modules
- has_many :context_module_tags
- has_many :media_objects
- has_many :page_views
- has_many :asset_user_accesses
- has_many :role_overrides
- has_many :content_migrations
- has_many :content_exports
- has_many :epub_exports
- has_many :course_reports
- has_many :gradebook_filters
- has_many :web_zip_exports
- has_many :alerts
- has_many :appointment_group_contexts
- has_many :appointment_groups
- has_many :appointment_participants
- has_many :content_participation_counts
- has_many :poll_sessions
- has_many :grading_period_groups
- has_many :grading_periods
- has_many :usage_rights
- has_many :custom_grade_statuses
- has_many :sis_post_grades_statuses
- has_many :progresses
- has_many :gradebook_csvs
- has_many :master_course_templates
- has_many :master_course_subscriptions
- has_one :late_policy
- has_many :quiz_migration_alerts
- has_many :notification_policy_overrides
- has_many :post_policies
- has_many :assignment_post_policies
- has_one :default_post_policy
- has_one :course_score_statistic
- has_many :auditor_course_records
- has_many :auditor_grade_change_records
- has_many :lti_resource_links
- has_many :conditional_release_rules
- has_one :outcome_proficiency
- has_one :outcome_calculation_method
- has_one :microsoft_sync_group
- has_many :microsoft_sync_partial_sync_changes
- has_many :comment_bank_items
- has_many :course_paces
- has_many :blackout_dates
- has_many :favorites
- belongs_to :account

## Methods

### time_zone

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

class Course < ActiveRecord::Base
  include Context
  include Workflow
  include TextHelper
  include HtmlTextHelper
  include TimeZoneHelper
  include ContentLicenses
  include TurnitinID
  include Courses::ItemVisibilityHelper
  include Courses::ExportWarnings
  include OutcomeImportContext
  include MaterialChanges

  attr_accessor :teacher_names, :master_course, :primary_enrollment_role, :saved_by
  attr_writer :student_count, :teacher_count, :primary_enrollment_type, :primary_enrollment_role_id, :primary_enrollment_rank, :primary_enrollment_state, :primary_enrollment_date, :invitation, :master_migration

  alias_attribute :short_name, :course_code

  time_zone_attribute :time_zone

### inherited_assessment_question_banks

regardless of non_collaborative state, all active groups are included
  has_many :combined_groups_and_differentiation_tags, class_name: "Group", as: :context, inverse_of: :context
  has_many :assignment_groups, -> { order("assignment_groups.position", AssignmentGroup.best_unicode_collation_key("assignment_groups.name")) }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :assignments, -> { order("assignments.created_at") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :calendar_events, -> { where("calendar_events.workflow_state<>'cancelled'") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :submissions, -> { active.order("submissions.updated_at DESC") }, inverse_of: :course, dependent: :destroy
  has_many :submission_comments, -> { published }, as: :context, inverse_of: :context
  has_many :discussion_topics, -> { where("discussion_topics.workflow_state<>'deleted'").preload(:user).order("discussion_topics.position DESC, discussion_topics.created_at DESC") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :active_discussion_topics, -> { where("discussion_topics.workflow_state<>'deleted'").preload(:user) }, as: :context, inverse_of: :context, class_name: "DiscussionTopic"
  has_many :all_discussion_topics, -> { preload(:user) }, as: :context, inverse_of: :context, class_name: "DiscussionTopic", dependent: :destroy
  has_many :discussion_entries, -> { preload(:discussion_topic, :user) }, through: :discussion_topics, dependent: :destroy
  has_many :announcements, as: :context, inverse_of: :context, class_name: "Announcement", dependent: :destroy
  has_many :active_announcements, -> { where("discussion_topics.workflow_state<>'deleted'") }, as: :context, inverse_of: :context, class_name: "Announcement"
  has_many :attachments, as: :context, inverse_of: :context, dependent: :destroy, extend: Attachment::FindInContextAssociation
  has_many :active_images, -> { where("attachments.file_state<>? AND attachments.content_type LIKE 'image%'", "deleted").order("attachments.display_name").preload(:thumbnail) }, as: :context, inverse_of: :context, class_name: "Attachment"
  has_many :active_assignments, -> { where("assignments.workflow_state<>'deleted'").order("assignments.title, assignments.position") }, as: :context, inverse_of: :context, class_name: "Assignment"
  has_many :folders, -> { order("folders.name") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :active_folders, -> { where("folders.workflow_state<>'deleted'").order("folders.name") }, class_name: "Folder", as: :context, inverse_of: :context
  has_many :messages, as: :context, inverse_of: :context, dependent: :destroy
  has_many :context_external_tools, -> { order("name") }, as: :context, inverse_of: :context, dependent: :destroy
  has_many :tool_proxies, class_name: "Lti::ToolProxy", as: :context, inverse_of: :context, dependent: :destroy
  belongs_to :wiki
  has_many :wiki_pages, as: :context, inverse_of: :context
  has_many :wiki_page_lookups, as: :context, inverse_of: :context
  has_many :quizzes, -> { order(:lock_at, :title, :id) }, class_name: "Quizzes::Quiz", as: :context, inverse_of: :context, dependent: :destroy
  has_many :quiz_questions, class_name: "Quizzes::QuizQuestion", through: :quizzes
  has_many :active_quizzes, -> { preload(:assignment).where("quizzes.workflow_state<>'deleted'").order(:created_at) }, class_name: "Quizzes::Quiz", as: :context, inverse_of: :context
  has_many :assessment_question_banks, -> { preload(:assessment_questions, :assessment_question_bank_users) }, as: :context, inverse_of: :context
  has_many :assessment_questions, through: :assessment_question_banks

### grade_statuses

only valid if non-nil
  attr_accessor :is_master_course

  has_many :master_course_subscriptions, class_name: "MasterCourses::ChildSubscription", foreign_key: "child_course_id", inverse_of: :child_course
  has_one :late_policy, dependent: :destroy, inverse_of: :course
  has_many :quiz_migration_alerts, dependent: :destroy
  has_many :notification_policy_overrides, as: :context, inverse_of: :context

  has_many :post_policies, dependent: :destroy, inverse_of: :course
  has_many :assignment_post_policies, -> { where.not(assignment_id: nil) }, class_name: "PostPolicy", inverse_of: :course
  has_one :default_post_policy, -> { where(assignment_id: nil) }, class_name: "PostPolicy", inverse_of: :course

  has_one :course_score_statistic, dependent: :destroy
  has_many :auditor_course_records,
           class_name: "Auditors::ActiveRecord::CourseRecord",
           dependent: :destroy,
           inverse_of: :course
  has_many :auditor_grade_change_records,
           as: :context,
           inverse_of: :course,
           class_name: "Auditors::ActiveRecord::GradeChangeRecord",
           dependent: :destroy
  has_many :lti_resource_links,
           as: :context,
           inverse_of: :context,
           class_name: "Lti::ResourceLink",
           dependent: :destroy

  has_many :conditional_release_rules, inverse_of: :course, class_name: "ConditionalRelease::Rule", dependent: :destroy
  has_one :outcome_proficiency, -> { preload(:outcome_proficiency_ratings) }, as: :context, inverse_of: :context, dependent: :destroy
  has_one :outcome_calculation_method, as: :context, inverse_of: :context, dependent: :destroy

  has_one :microsoft_sync_group, class_name: "MicrosoftSync::Group", dependent: :destroy, inverse_of: :course
  has_many :microsoft_sync_partial_sync_changes, class_name: "MicrosoftSync::PartialSyncChange", dependent: :destroy, inverse_of: :course

  has_many :comment_bank_items, inverse_of: :course

  has_many :course_paces
  has_many :blackout_dates, as: :context, inverse_of: :context
  has_many :favorites, as: :context, inverse_of: :context, dependent: :destroy

  prepend Profile::Association

  before_create :set_restrict_quantitative_data_when_needed

  before_save :assign_uuid
  before_validation :assert_defaults
  before_save :update_enrollments_later
  before_save :update_show_total_grade_as_on_weighting_scheme_change
  before_save :set_self_enrollment_code
  before_save :validate_license
  after_save :update_final_scores_on_weighting_scheme_change
  after_save :update_account_associations_if_changed
  after_save :update_enrollment_states_if_necessary
  after_save :clear_caches_if_necessary
  after_save :log_published_assignment_count
  after_commit :update_cached_due_dates

  after_create :set_default_post_policy
  after_create :copy_from_course_template

  after_update :clear_cached_short_name, if: :saved_change_to_course_code?
  after_update :log_create_to_publish_time, if: :saved_change_to_workflow_state?
  after_update :track_end_date_stats
  after_update :log_course_pacing_publish_update, if: :saved_change_to_workflow_state?
  after_update :log_course_format_publish_update, if: :saved_change_to_workflow_state?
  after_update :log_course_pacing_settings_update, if: :change_to_logged_settings?
  after_update :log_rqd_setting_enable_or_disable

  before_update :handle_syllabus_changes_for_master_migration

  before_save :touch_root_folder_if_necessary
  before_validation :verify_unique_ids
  validate :validate_course_dates
  validate :validate_course_image
  validate :validate_banner_image
  validate :validate_default_view
  validate :validate_template
  validate :validate_not_on_siteadmin
  validates :sis_source_id, uniqueness: { scope: :root_account }, allow_nil: true
  validates :account_id, :root_account_id, :enrollment_term_id, :workflow_state, presence: true
  validates :syllabus_body, length: { maximum: maximum_long_text_length, allow_blank: true }
  validates :name, length: { maximum: maximum_string_length, allow_blank: true }
  validates :sis_source_id, length: { maximum: maximum_string_length, allow_nil: true, allow_blank: false }
  validates :course_code, length: { maximum: maximum_string_length, allow_blank: true }
  validates_locale allow_nil: true

  sanitize_field :syllabus_body, CanvasSanitize::SANITIZE

  include StickySisFields
  are_sis_sticky :name,
                 :course_code,
                 :start_at,
                 :conclude_at,
                 :restrict_enrollments_to_course_dates,
                 :enrollment_term_id,
                 :workflow_state,
                 :account_id,
                 :grade_passback_setting

  include FeatureFlags

  include ContentNotices
  define_content_notice :import_in_progress,
                        text: -> { t("One or more items are currently being imported. They will be shown in the course below once they are available.") },
                        link_text: -> { t("Import Status") },
                        link_target: ->(course) { "/courses/#{course.to_param}/content_migrations" },
                        should_show: lambda { |course, user|
                          course.grants_any_right?(user, *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS)
                        }

  has_a_broadcast_policy

  # A hard limit on the number of graders (excluding the moderator) a moderated
  # assignment can have.
  MODERATED_GRADING_GRADER_LIMIT = 10

  # using a lambda for setting name to avoid caching the translated string when the model is loaded
  # (in case selected language changes)
  CUSTOMIZABLE_PERMISSIONS = ActiveSupport::OrderedHash[
    "syllabus",
    {
      get_setting_name: -> { t("syllabus", "Syllabus") },
      flex: :looser,
      as_bools: true,
    },
    "files",
    {
      get_setting_name: -> { t("files", "Files") },
      flex: :any
    },
  ].freeze

  def [](attr)
    (attr.to_s == "asset_string") ? asset_string : super
  end

### self

pre-loading dummy account here to avoid error when finding
    # Account 0 on a new shard before the shard is finished creation,
    # since finding via cache switches away from the creating shard
    a = Account.find(0)
    create_with(account: a, root_account: a, enrollment_term_id: 0, workflow_state: "deleted").find_or_create_by!(id: 0)
  end

### grading_standard_read_permission

We need to update the scope to use AbstractAssignment instead of its subclass Assignment so that we can merge the
      # scope query with the checkpoints_scope query
      scope_assignment_ids = scope.pluck(:id)
      scope = AbstractAssignment.where(id: scope_assignment_ids)
      checkpoints_scope = SubAssignment.active.where(parent_assignment_id: scope_assignment_ids)
      # merge the queries
      scope = scope.or(checkpoints_scope)
    end
    scope
  end

### track_end_date_stats

a lot of things can change the date logic here :/
    if (saved_changes.keys.intersect?(%w[restrict_enrollments_to_course_dates account_id enrollment_term_id]) ||
       (restrict_enrollments_to_course_dates? && saved_material_changes_to?(:start_at, :conclude_at)) ||
       (saved_change_to_workflow_state? && (completed? || workflow_state_before_last_save == "completed"))) &&
       enrollments.exists?
      EnrollmentState.delay_if_production(n_strand: ["invalidate_enrollment_states", global_root_account_id])
                     .invalidate_states_for_course_or_section(self)
    end
    # if the course date settings have been changed, we'll end up reprocessing all the access values anyway, so no need to queue below for other setting changes
    if saved_change_to_account_id? || @changed_settings
      state_settings = [:restrict_student_future_view, :restrict_student_past_view]
      changed_keys = saved_change_to_account_id? ? state_settings : (@changed_settings & state_settings)
      if changed_keys.any?
        EnrollmentState.delay_if_production(n_strand: ["invalidate_access_for_course", global_root_account_id])
                       .invalidate_access_for_course(self, changed_keys)
      end
    end

    @changed_settings = nil
  end

### image

Don't validate if we are creating the dummy account so we don't go try to create siteadmin while migrating
    return if id == 0

    if root_account_id_changed? && root_account_id == Account.site_admin&.id
      errors.add(:root_account_id, t("Courses cannot be created on the site_admin account."))
    end
  end

### syllabus_visibility_option

DEPRECATED - Used only by View

### files_visibility_option

DEPRECATED - Used only by View

### update_account_associations

Split it up into manageable chunks
    user_ids_to_update_account_associations = []
    if courses_or_course_ids.length > 500
      opts = opts.dup
      opts.reverse_merge! skip_user_account_associations: true
      courses_or_course_ids.uniq.compact.each_slice(500) do |courses_or_course_ids_slice|
        user_ids_to_update_account_associations += update_account_associations(courses_or_course_ids_slice, opts)
      end
    else
      if courses_or_course_ids.first.is_a? Course
        courses = courses_or_course_ids
        ActiveRecord::Associations.preload(courses, course_sections: :nonxlist_course)
        course_ids = courses.map(&:id)
      else
        course_ids = courses_or_course_ids
        courses = Course.where(id: course_ids)
                        .preload(course_sections: [:course, :nonxlist_course])
                        .select([:id, :account_id]).to_a
      end
      course_ids_to_update_user_account_associations = []
      CourseAccountAssociation.transaction do
        current_associations = {}
        to_delete = []
        CourseAccountAssociation.where(course_id: course_ids).each do |aa|
          key = [aa.course_section_id, aa.account_id]
          current_course_associations = current_associations[aa.course_id] ||= {}
          # duplicates. the unique index prevents these now, but this code
          # needs to hang around for the migration itself
          if current_course_associations.key?(key)
            to_delete << aa.id
            next
          end
          current_course_associations[key] = [aa.id, aa.depth]
        end

        courses.each do |course|
          did_an_update = false
          current_course_associations = current_associations[course.id] || {}

          # Courses are tied to accounts directly and through sections and crosslisted courses
          (course.course_sections + [nil]).each do |section|
            next if section && !section.active?

            section.course = course if section
            starting_account_ids = [course.account_id, section.try(:course).try(:account_id), section.try(:nonxlist_course).try(:account_id)].compact.uniq

            account_ids_with_depth = User.calculate_account_associations_from_accounts(starting_account_ids, account_chain_cache).map

            account_ids_with_depth.each do |account_id_with_depth|
              account_id = account_id_with_depth[0]
              depth = account_id_with_depth[1]
              key = [section.try(:id), account_id]
              association = current_course_associations[key]
              if association.nil?
                # new association, create it
                begin
                  course.transaction(requires_new: true) do
                    course.course_account_associations.create! do |aa|
                      aa.course_section_id = section.try(:id)
                      aa.account_id = account_id
                      aa.depth = depth
                    end
                  end
                rescue ActiveRecord::RecordNotUnique
                  course.course_account_associations.where(course_section_id: section,
                                                           account_id:).update_all(depth:)
                end
                did_an_update = true
              else
                if association[1] != depth
                  CourseAccountAssociation.where(id: association[0]).update_all(depth:)
                  did_an_update = true
                end
                # remove from list of existing
                current_course_associations.delete(key)
              end
            end
          end
          did_an_update ||= !current_course_associations.empty?
          if did_an_update
            course.course_account_associations.reset
            course_ids_to_update_user_account_associations << course.id
          end
        end

        to_delete += current_associations.map { |_k, v| v.map { |_k2, v2| v2[0] } }.flatten
        unless to_delete.empty?
          CourseAccountAssociation.where(id: to_delete).in_batches(of: 10_000).delete_all
        end
      end
      Course.clear_cache_keys(course_ids_to_update_user_account_associations, :account_associations)

      unless course_ids_to_update_user_account_associations.empty?
        user_ids_to_update_account_associations = Enrollment
                                                  .where("course_id IN (?) AND workflow_state<>'deleted'", course_ids_to_update_user_account_associations)
                                                  .group(:user_id).pluck(:user_id)
      end
    end
    User.update_account_associations(user_ids_to_update_account_associations, account_chain_cache:) unless user_ids_to_update_account_associations.empty? || opts[:skip_user_account_associations]
    user_ids_to_update_account_associations
  end

### potential_collaborators

{"AND course_section_id IS NULL" unless include_crosslisted_courses}
                           GROUP BY account_id
                         )
                         SELECT accounts.*
                         FROM #{Account.quoted_table_name} INNER JOIN depths ON accounts.id=depths.account_id
                         ORDER BY min
                       SQL
                     end
                   end
        accounts << account if account_id && !accounts.find { |a| a.id == account_id }
        accounts << root_account if root_account_id && !accounts.find { |a| a.id == root_account_id }
        accounts
      end
    end
  end

  scope :recently_started, -> { where(start_at: 1.month.ago..Time.zone.now).order("start_at DESC").limit(10) }
  scope :recently_ended, -> { where(conclude_at: 1.month.ago..Time.zone.now).order("start_at DESC").limit(10) }
  scope :recently_created, -> { where(created_at: 1.month.ago..Time.zone.now).order("created_at DESC").limit(50).preload(:teachers) }
  scope :for_term, ->(term) { term ? where(enrollment_term_id: term) : all }
  scope :active_first, -> { order(Arel.sql("CASE WHEN courses.workflow_state='available' THEN 0 ELSE 1 END, #{best_unicode_collation_key("name")}")) }
  scope :name_like, lambda { |query|
    where(coalesced_wildcard("courses.name", "courses.sis_source_id", "courses.course_code", query))
      .or(where(id: query))
  }
  scope :needs_account, ->(account, limit) { where(account_id: nil, root_account_id: account).limit(limit) }
  scope :active, -> { where.not(workflow_state: "deleted") }
  scope :least_recently_updated, ->(limit) { order(:updated_at).limit(limit) }

  scope :manageable_by_user, lambda { |*args|
    # args[0] should be user_id, args[1], if true, will include completed
    # enrollments as well as active enrollments
    user_id = args[0]
    workflow_states = (args[1].present? ? ["'active'", "'completed'"] : ["'active'"]).join(", ")
    admin_completed_sql = ""
    enrollment_completed_sql = ""

    if args[1].blank?
      admin_completed_sql = sanitize_sql(["INNER JOIN #{Course.quoted_table_name} AS courses ON courses.id = caa.course_id
        INNER JOIN #{EnrollmentTerm.quoted_table_name} AS et ON et.id = courses.enrollment_term_id
        WHERE courses.workflow_state<>'completed' AND
          ((et.end_at IS NULL OR et.end_at >= :end) OR
          (courses.restrict_enrollments_to_course_dates = true AND courses.conclude_at >= :end))",
                                          end: Time.now.utc])
      enrollment_completed_sql = sanitize_sql(["INNER JOIN #{EnrollmentTerm.quoted_table_name} AS et ON et.id = courses.enrollment_term_id
        WHERE courses.workflow_state<>'completed' AND
          ((et.end_at IS NULL OR et.end_at >= :end) OR
          (courses.restrict_enrollments_to_course_dates = true AND courses.conclude_at >= :end))",
                                               end: Time.now.utc])
    end

    distinct.joins("INNER JOIN (
         SELECT caa.course_id, au.user_id FROM #{CourseAccountAssociation.quoted_table_name} AS caa
         INNER JOIN #{Account.quoted_table_name} AS a ON a.id = caa.account_id AND a.workflow_state = 'active'
         INNER JOIN #{AccountUser.quoted_table_name} AS au ON au.account_id = a.id AND au.user_id = #{user_id.to_i} AND au.workflow_state = 'active'
         #{admin_completed_sql}
       UNION SELECT courses.id AS course_id, e.user_id FROM #{Course.quoted_table_name}
         INNER JOIN #{Enrollment.quoted_table_name} AS e ON e.course_id = courses.id AND e.user_id = #{user_id.to_i}
           AND e.workflow_state IN(#{workflow_states}) AND e.type IN ('TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment')
         INNER JOIN #{EnrollmentState.quoted_table_name} AS es ON es.enrollment_id = e.id AND es.state IN (#{workflow_states})
         #{enrollment_completed_sql}) AS course_users
       ON course_users.course_id = courses.id")
  }

  scope :not_deleted, -> { where("workflow_state<>'deleted'") }

  scope :with_enrollments, lambda {
    where(Enrollment.active.where("enrollments.course_id=courses.id").arel.exists)
  }
  scope :with_enrollment_types, lambda { |types|
    types = types.map { |type| "#{type.capitalize}Enrollment" }
    where(Enrollment.active.where("enrollments.course_id=courses.id").where(type: types).arel.exists)
  }
  scope :without_enrollments, lambda {
    where.not(Enrollment.active.where("enrollments.course_id=courses.id").arel.exists)
  }

  # completed and not_completed -- logic should match up as much as possible with #soft_concluded?
  scope :completed, lambda {
    joins(:enrollment_term)
      .where("courses.workflow_state='completed' OR courses.conclude_at<? OR (courses.conclude_at IS NULL AND enrollment_terms.end_at<?)", Time.now.utc, Time.now.utc)
  }
  scope :not_completed, lambda {
    joins(:enrollment_term)
      .where("courses.workflow_state<>'completed' AND
          (courses.conclude_at IS NULL OR courses.conclude_at>=?) AND
          (courses.conclude_at IS NOT NULL OR enrollment_terms.end_at IS NULL OR enrollment_terms.end_at>=?)",
             Time.now.utc,
             Time.now.utc)
  }
  scope :by_teachers, lambda { |teacher_ids|
    if teacher_ids.empty?
      none
    else
      where(Enrollment.active.where("enrollments.course_id=courses.id AND enrollments.type='TeacherEnrollment' AND enrollments.user_id IN (?)", teacher_ids).arel.exists)
    end
  }
  scope :by_associated_accounts, lambda { |account_ids|
    if account_ids.empty?
      none
    else
      where(CourseAccountAssociation.where("course_account_associations.course_id=courses.id AND course_account_associations.account_id IN (?)", account_ids).arel.exists)
    end
  }
  scope :published, -> { where(workflow_state: %w[available completed]) }
  scope :unpublished, -> { where(workflow_state: %w[created claimed]) }

  scope :deleted, -> { where(workflow_state: "deleted") }
  scope :archived, -> { deleted.where.not(archived_at: nil) }

  scope :master_courses, -> { joins(:master_course_templates).where.not(MasterCourses::MasterTemplate.table_name => { workflow_state: "deleted" }) }
  scope :not_master_courses, -> { joins("LEFT OUTER JOIN #{MasterCourses::MasterTemplate.quoted_table_name} AS mct ON mct.course_id=courses.id AND mct.workflow_state<>'deleted'").where("mct IS NULL") } # rubocop:disable Rails/WhereEquals -- mct is a table, not a column

  scope :associated_courses, -> { joins(:master_course_subscriptions).where.not(MasterCourses::ChildSubscription.table_name => { workflow_state: "deleted" }) }
  scope :not_associated_courses, -> { joins("LEFT OUTER JOIN #{MasterCourses::ChildSubscription.quoted_table_name} AS mcs ON mcs.child_course_id=courses.id AND mcs.workflow_state<>'deleted'").where("mcs IS NULL") } # rubocop:disable Rails/WhereEquals -- mcs is a table, not a column

  scope :public_courses, -> { where(is_public: true) }
  scope :not_public_courses, -> { where(is_public: false) }

  scope :templates, -> { where(template: true) }

  scope :homeroom, -> { where(homeroom_course: true) }
  scope :syncing_subjects, -> { joins("INNER JOIN #{Course.quoted_table_name} AS homeroom ON homeroom.id = courses.homeroom_course_id").where("homeroom.homeroom_course = true AND homeroom.workflow_state <> 'deleted'").where(sis_batch_id: nil).where(sync_enrollments_from_homeroom: true) }

  scope :horizon, -> { where(horizon_course: true) }
  scope :not_horizon, -> { where(horizon_course: false) }

### users_not_in_groups

TODO: i18n
    t("default_name", "My Course")
  end

### user_is_admin

filter to users with view_all_grades or manage_grades permission
        role_user_ids = instructor_enrollment_scope.pluck(:role_id, :user_id)
        return [] unless role_user_ids.any?

        role_ids = role_user_ids.map(&:first).uniq

        roles = Role.where(id: role_ids).to_a
        allowed_role_ids = roles.select do |role|
          [:view_all_grades, :manage_grades].any? { |permission| RoleOverride.enabled_for?(self, permission, role, self).include?(:self) }
        end.map(&:id)
        return [] unless allowed_role_ids.any?

        allowed_user_ids = Set.new
        role_user_ids.each { |role_id, u_id| allowed_user_ids << u_id if allowed_role_ids.include?(role_id) }
        User.where(id: allowed_user_ids).to_a
      else
        User.where(id: instructor_enrollment_scope.select(:id)).to_a
      end
    end
  end

### preloaded_user_has_been

plz to use before you make a billion calls to user_has_been_X? with different users
    @user_ids_by_enroll_type ||= shard.activate do
      map = {}
      enrollments.active.pluck(:user_id, :type).each do |user_id, type|
        map[type] ||= []
        map[type] << user_id
      end
      map
    end
  end

### user_has_been_admin

enrollments should be on the course's shard
    fetch_on_enrollments("user_has_been_instructor", user) do
      instructor_enrollments.active.where(user_id: user).exists? # active here is !deleted; it still includes concluded, etc.
    end
  end

### apply_group_weights

Public: Determine if a group weighting scheme should be applied.
  #
  # Returns boolean.

### self_enrollment_allowed

We might get lots of database locks when lots of courses with the same users are being updated,
        # so we can skip touching those users' updated_at stamp since another process will do it
        User.touch_and_clear_cache_keys(user_ids, :enrollments, skip_locked: true)
      end

      data
    end
  end

### self_enrollment_limit_met

subset of letters and numbers that are unambiguous
    alphanums = "ABCDEFGHJKLMNPRTWXY346789".chars
    code_length = 6

    # we're returning a 6-digit base-25(ish) code. that means there are ~250
    # million possible codes. we should expect to see our first collision
    # within the first 16k or so (thus the retry loop), but we won't risk ever
    # exhausting a retry loop until we've used up about 15% or so of the
    # keyspace. if needed, we can grow it at that point (but it's scoped to a
    # shard, and not all courses will have enrollment codes, so that may not be
    # necessary)
    code = nil
    10.times do
      code = Array.new(code_length) { alphanums.sample }.join
      next if Course.where(self_enrollment_code: code).exists?

      self.self_enrollment_code = code
      break
    end
    code
  end

### self_enrollment_codes

still include the old longer format, since links may be out there

### validate_license

set license to "private" if it's present but not recognized

### touch_root_folder_if_necessary

to ensure permissions on the root folder are updated after hiding or showing the files tab

### recompute_student_scores_without_send_later

if we have all default args, let's queue this job in a singleton to avoid duplicates
        inst_job_opts[:singleton] = "recompute_student_scores:#{global_id}"
      elsif student_ids.blank? && grading_period_id.present?
        # A migration that changes a lot of due dates in a grading period
        # situation can kick off a job storm and redo work. Let's avoid
        # that by putting it into a singleton.
        inst_job_opts[:singleton] = "recompute_student_scores:#{global_id}:#{grading_period_id}"
      end

      delay_if_production(**inst_job_opts).recompute_student_scores_without_send_later(
        student_ids,
        grading_period_id:,
        update_all_grading_period_scores:
      )
    end
  end

### handle_syllabus_changes_for_master_migration

We were given student_ids.  Let's see how many of those students can even see this assignment
                            admin_visible_student_enrollments.where(user_id: student_ids).pluck(:user_id)
                          else
                            # We were not given any student_ids
                            # Let's get them all!
                            admin_visible_student_enrollments.pluck(:user_id)
                          end

    Enrollment.recompute_final_score(
      visible_student_ids,
      id,
      grading_period_id: opts[:grading_period_id],
      update_all_grading_period_scores: opts.fetch(:update_all_grading_period_scores, true)
    )
  end

### home_page

master migration sync
        self.syllabus_master_template_id ||= updating_master_template_id if syllabus_body_was.blank? # sync if there was no syllabus before
        if self.syllabus_master_template_id.to_i != updating_master_template_id
          restore_syllabus_body! # revert the change
          @master_migration.add_skipped_item(:syllabus)
        end
      elsif self.syllabus_master_template_id
        # local change - remove the template id to prevent future syncs
        self.syllabus_master_template_id = nil
      end
    end
  end

### discussion_checkpoints_enabled

Allows the account to be set directly
  belongs_to :account

### messages

A universal lookup for all messages.

### allows_gradebook_uploads

Active students
    given do |user|
      available? && user && fetch_on_enrollments("has_active_student_enrollment", user) { enrollments.for_user(user).active_by_date.of_student_type.exists? }
    end
    can :read, :participate_as_student, :read_grades, :read_outcomes, :read_as_member, :reset_what_if_grades

    given do |user|
      (available? || completed?) && user &&
        fetch_on_enrollments("has_active_observer_enrollment", user) { enrollments.for_user(user).active_by_date.where(type: "ObserverEnrollment").where.not(associated_user_id: nil).exists? }
    end
    can :read_grades

    # Active admins (Teacher/TA/Designer)
    given do |user|
      user && (available? || created? || claimed?) &&
        fetch_on_enrollments("has_active_admin_enrollment", user) do
          enrollments.for_user(user).of_admin_type.active_by_date.exists?
        end
    end
    can %i[
      read_as_admin
      read
      read_as_member
      manage
      update
      read_outcomes
      view_unpublished_items
      manage_feature_flags
      view_feature_flags
      read_rubrics
      use_student_view
    ]

    # Teachers and Designers can reset content, but not TAs
    given do |user|
      user && !deleted? && !template? &&
        fetch_on_enrollments("active_content_admin_enrollments", user) do
          enrollments.for_user(user).of_content_admins.active_by_date.to_a
        end.any? { |e| e.has_permission_to?(:manage_courses_reset) }
    end
    can :reset_content

    # Teachers and Designers can delete, but not TAs
    given do |user|
      user && !template? && !deleted? && !sis_source_id &&
        fetch_on_enrollments("active_content_admin_enrollments", user) do
          enrollments.for_user(user).of_content_admins.active_by_date.to_a
        end.any? { |e| e.has_permission_to?(:manage_courses_delete) }
    end
    can :delete

    # Student view student
    given { |user| user&.fake_student? && current_enrollments.for_user(user).exists? }
    can %i[read participate_as_student read_grades read_outcomes read_as_member]

    # Prior users
    given do |user|
      (available? || completed?) && user &&
        fetch_on_enrollments("has_completed_enrollment", user) { enrollments.for_user(user).completed_by_date.exists? }
    end
    can :read, :read_outcomes, :read_as_member

    # Admin (Teacher/TA/Designer) of a concluded course
    given do |user|
      !deleted? && user &&
        fetch_on_enrollments("has_completed_admin_enrollment", user) { enrollments.for_user(user).of_admin_type.completed_by_date.exists? }
    end
    can %i[read read_as_admin use_student_view read_outcomes view_unpublished_items read_rubrics read_as_member]

    # overrideable permissions for concluded users
    RoleOverride.concluded_permission_types.each do |permission, details|
      applicable_roles = details[:applies_to_concluded].is_a?(Array) && details[:applies_to_concluded]

      given do |user|
        !deleted? && user &&
          fetch_on_enrollments("completed_enrollments", user) { enrollments.for_user(user).completed_by_date.to_a }.any? { |e| e.has_permission_to?(permission) && (!applicable_roles || applicable_roles.include?(e.type)) }
      end
      can permission
    end

    # Teacher or Designer of a concluded course
    given do |user|
      user && !sis_source_id && !deleted? && !template? &&
        enrollments.for_user(user).of_content_admins.completed_by_date.to_a.any? do |e|
          e.has_permission_to?(:manage_courses_delete)
        end
    end
    can :delete

    # Student of a concluded course
    given do |user|
      (available? || completed?) && user &&
        fetch_on_enrollments("has_completed_student_enrollment", user) do
          enrollments.for_user(user).completed_by_date
                     .where("enrollments.type = ? OR (enrollments.type = ? AND enrollments.associated_user_id IS NOT NULL)", "StudentEnrollment", "ObserverEnrollment").exists?
        end
    end
    can :read, :read_grades, :read_outcomes, :read_as_member

    # Admin
    given { |user| account_membership_allows(user) }
    can :read_as_admin and can :view_unpublished_items

    given do |user|
      account_membership_allows(user, :manage_courses_admin)
    end
    can :manage and can :update and can :use_student_view and can :manage_feature_flags and
      can :view_feature_flags

    # reset course content
    given do |user|
      !template? && account_membership_allows(user, :manage_courses_reset)
    end
    can :reset_content

    # delete or undelete a given course
    given do |user|
      !template? && account_membership_allows(user, :manage_courses_delete)
    end
    can :delete

    given { |user| account_membership_allows(user, :read_course_content) }
    can %i[read read_outcomes read_as_member]

    # Admins with read_roster can see prior enrollments (can't just check read_roster directly,
    # because students can't see prior enrollments)
    given { |user| grants_all_rights?(user, :read_roster, :read_as_admin) }
    can :read_prior_roster

    given do |user|
      grants_right?(user, :manage_course_content_add) ||
        (concluded? && grants_right?(user, :read_as_admin))
    end
    can :direct_share

    given do |user|
      account.grants_right?(user, :manage_courses_admin) ||
        (grants_right?(user, :manage) && !root_account.settings[:prevent_course_availability_editing_by_teachers])
    end
    can :edit_course_availability
  end

### allows_speed_grader

Public: Determine if SpeedGrader is enabled for the Course.
  #
  # Returns a boolean.

### inactive

People may conclude courses and then unclude them. This is a good alias_method
  # to check for in situations where we are dependent on those cases

### soft_concluded

Public: Return true if the end date for a course (or its term, if the course doesn't have one) has passed.
  # Logic should match up as much as possible with scopes `completed` and `not_completed`
  #
  # Returns boolean

### account_chain_ids

This implicitly includes add_federated_parent_to_chain
    if include_site_admin
      return @account_chain_with_site_admin ||= Account.add_site_admin_to_chain!(@account_chain.dup).freeze
    end

    if include_federated_parent
      return @account_chain_with_federated_parent ||= Account.add_federated_parent_to_chain!(@account_chain.dup).freeze
    end

    @account_chain
  end

### account_membership_allows

Since this method can return AdheresToPolicy::JustifiedFailure, it must be last in a `given` block
  # or must be explicitly checked for truth

### grade_publishing_status_translation

return the first result with a justification or false, either of which will deny access
        results.find { |r| r.is_a?(AdheresToPolicy::JustifiedFailure) } || false
      end
    end
  end

### send_final_grades_to_endpoint

we want to set all the publishing statuses to 'pending' immediately,
    # and then as a delayed job, actually go publish them.

    raise "final grade publishing disabled" unless Canvas::Plugin.find!("grade_export").enabled?

    settings = Canvas::Plugin.find!("grade_export").settings

    last_publish_attempt_at = Time.now.utc
    scope = student_enrollments.not_fake
    scope = scope.where(user_id: user_ids_to_publish) if user_ids_to_publish
    scope.update_all(grade_publishing_status: "pending",
                     grade_publishing_message: nil,
                     last_publish_attempt_at:)

    delay_if_production(n_strand: ["send_final_grades_to_endpoint", global_root_account_id])
      .send_final_grades_to_endpoint(publishing_user, user_ids_to_publish)
    delay(run_at: last_publish_attempt_at + settings[:success_timeout].to_i.seconds).expire_pending_grade_publishing_statuses(last_publish_attempt_at) if should_kick_off_grade_publishing_timeout?
  end

### generate_grade_publishing_csv_output

actual grade publishing logic is here, but you probably want
    # 'publish_final_grades'

    recompute_student_scores_without_send_later(user_ids_to_publish)
    enrollments = student_enrollments.not_fake.eager_load(:user).preload(:course_section).order_by_sortable_name
    enrollments = enrollments.where(user_id: user_ids_to_publish) if user_ids_to_publish

    errors = []
    posts_to_make = []
    posted_enrollment_ids = []
    all_enrollment_ids = enrollments.map(&:id)

    begin
      raise "final grade publishing disabled" unless Canvas::Plugin.find!("grade_export").enabled?

      settings = Canvas::Plugin.find!("grade_export").settings
      raise "endpoint undefined" if settings[:publish_endpoint].blank?

      format_settings = Course.valid_grade_export_types[settings[:format_type]]
      raise "unknown format type: #{settings[:format_type]}" unless format_settings
      raise "grade publishing requires a grading standard" if !grading_standard_enabled? && format_settings[:requires_grading_standard]

      publishing_pseudonym = SisPseudonym.for(publishing_user, self)
      raise "publishing disallowed for this publishing user" if publishing_pseudonym.nil? && format_settings[:requires_publishing_pseudonym]

      callback = Course.valid_grade_export_types[settings[:format_type]][:callback]
      posts_to_make = callback.call(self, enrollments, publishing_user, publishing_pseudonym)
    rescue => e
      Enrollment.where(id: all_enrollment_ids).update_all(grade_publishing_status: "error", grade_publishing_message: e.to_s)
      raise e
    end

    default_timeout = Setting.get("send_final_grades_to_endpoint_timelimit", 15.seconds.to_s).to_f

    timeout_options = { raise_on_timeout: true, fallback_timeout_length: default_timeout }

    posts_to_make.each do |enrollment_ids, res, mime_type, headers = {}|
      posted_enrollment_ids += enrollment_ids
      if res
        Canvas.timeout_protection("send_final_grades_to_endpoint:#{global_root_account_id}", timeout_options) do
          SSLCommon.post_data(settings[:publish_endpoint], res, mime_type, headers)
        end
      end
      Enrollment.where(id: enrollment_ids).update_all(grade_publishing_status: (should_kick_off_grade_publishing_timeout? ? "publishing" : "published"), grade_publishing_message: nil)
    rescue => e
      errors << e
      Enrollment.where(id: enrollment_ids).update_all(grade_publishing_status: "error", grade_publishing_message: e.to_s)
    end

    Enrollment.where(id: (all_enrollment_ids.to_set - posted_enrollment_ids.to_set).to_a).update_all(grade_publishing_status: "unpublishable", grade_publishing_message: nil)

    raise errors[0] unless errors.empty?
  end

### grading_standard_title

included to make it easier to work with api, which returns
  # sis_source_id as sis_course_id.
  alias_attribute :sis_course_id, :sis_source_id

### enroll_student

order by course_section_id<>section.id so that if there *is* an existing
            # enrollment for this section, we get it (false orders before true)
            scope.order(Arel.sql("course_section_id<>#{section.id}")).first
          end
      if e && (!e.active? || opts[:force_update])
        e.already_enrolled = true
        if e.workflow_state == "deleted"
          e.sis_batch_id = nil
        end
        if e.completed? || e.rejected? || e.deleted? || e.workflow_state != enrollment_state
          e.attributes = {
            course_section: section,
            workflow_state: e.is_a?(StudentViewEnrollment) ? "active" : enrollment_state
          }
        end
      end
      # if we're reusing an enrollment and +limit_privileges_to_course_section+ was supplied, apply it
      e.limit_privileges_to_course_section = limit_privileges_to_course_section if e
      # if we're creating a new enrollment, we want to return it as the correct
      # subclass, but without using associations, we need to manually activate
      # sharding. We should probably find a way to go back to using the
      # association here -- just ran out of time.
      shard.activate do
        e ||= Enrollment.typed_enrollment(type).new(
          user:,
          course: self,
          course_section: section,
          workflow_state: enrollment_state,
          limit_privileges_to_course_section:
        )
      end
      e.associated_user_id = associated_user_id
      e.temporary_enrollment_source_user_id = source_user_id
      e.temporary_enrollment_pairing_id = pairing_id
      e.role = role
      e.self_enrolled = self_enrolled
      e.start_at = start_at
      e.end_at = end_at
      e.sis_pseudonym_id = opts[:sis_pseudonym_id]
      if e.changed?
        e.need_touch_user = true if opts[:skip_touch_user]
        if opts[:no_notify]
          e.save_without_broadcasting
        else
          e.save
        end
      end
      e.user = user
      claim if created? && e && e.admin?
      unless opts[:skip_touch_user]
        e.associated_user.try(:touch)
        user.touch
      end
      user.reload
      e
    end
  end

### self

make sure the file has somewhere to go
          unless new_folder_id
            # gather mapping of needed folders from old course to new course
            old_folders = []
            old_folders << file.folder
            new_folders = []
            new_folders << old_folders.last.clone_for(self, nil, options.merge({ include_subcontent: false }))
            while old_folders.last.parent_folder&.parent_folder_id
              old_folders << old_folders.last.parent_folder
              new_folders << old_folders.last.clone_for(self, nil, options.merge({ include_subcontent: false }))
            end
            old_folders.reverse!
            new_folders.reverse!
            # try to use folders that already match if possible
            final_new_folders = []
            parent_folder = Folder.root_folders(self).first
            old_folders.each_with_index do |folder, idx|
              final_new_folders << if (f = parent_folder.active_sub_folders.where(name: folder.name).first)
                                     f
                                   else
                                     new_folders[idx]
                                   end
              parent_folder = final_new_folders.last
            end
            # add or update the folder structure needed for the file
            final_new_folders.first.parent_folder_id ||=
              merge_mapped_id(old_folders.first.parent_folder) ||
              Folder.root_folders(self).first.id
            old_folders.each_with_index do |folder, idx|
              final_new_folders[idx].save!
              map_merge(folder, final_new_folders[idx])
              final_new_folders[idx + 1].parent_folder_id ||= final_new_folders[idx].id if final_new_folders[idx + 1]
            end
            new_folder_id = merge_mapped_id(file.folder)
          end
          new_file.folder_id = new_folder_id
          new_file.need_notify = false
          new_file.save_without_broadcasting!
          new_file.handle_duplicates(:rename)
          cm.add_imported_item(new_file)
          cm.add_imported_item(new_file.folder, key: new_file.folder.id)
          map_merge(file, new_file)
        rescue => e
          Canvas::Errors.capture(e) unless e.message.include?("Cannot create attachments in deleted folders")
          Rails.logger.error "Couldn't copy file: #{e}"
          cm.add_warning(t(:file_copy_error, "Couldn't copy file \"%{name}\"", name: file.display_name || file.path_name), $!)
        end
      end
    end
  end

### fetch_on_enrollments

helper method to DRY-up some similar methods that all can be cached based on a user's enrollments

### visibility_limited_to_course_sections

only keep temporary enrollments if they are active, and keep all permanent enrollments
        e.temporary_enrollment? ? e.enrollment_state.active? : true
      end

      enrollment_rows = enrollment_rows.pluck(
        :course_section_id,
        :limit_privileges_to_course_section,
        :type,
        :associated_user_id,
        :workflow_state
      )

      enrollment_rows.map do |section_id, limit_privileges, type, associated_user_id, workflow_state|
        {
          course_section_id: section_id,
          limit_privileges_to_course_section: limit_privileges,
          type:,
          associated_user_id:,
          admin: ADMIN_TYPES.include?(type),
          workflow_state:
        }
      end
    end
  end

### students_visible_to

returns a scope, not an array of users/enrollments

### apply_enrollment_visibility

can apply to user scopes as well if through enrollments (e.g. students, teachers)
  # returns a scope for enrollments

### users_visible_to

teachers, account admins, and student view students can see student view students
    unless visibility_level == :full ||
           visibilities.any? { |v| v[:admin] || v[:type] == "StudentViewEnrollment" }
      scope = scope.where("enrollments.type<>'StudentViewEnrollment'")
    end

    if include.include?(:inactive) && ![:full, :sections].include?(visibility_level)
      # don't really include inactive unless user is able to view them
      scope = scope.where("enrollments.workflow_state <> 'inactive'")
    end
    if include.include?(:completed) && ![:full, :sections].include?(visibility_level)
      # don't really include concluded unless user is able to view them
      scope = scope.where("enrollments.workflow_state <> 'completed'")
    end
    # See also MessageableUser::Calculator (same logic used to get
    # users across multiple courses) (should refactor)
    case visibility_level
    when :full, :limited
      scope
    when :sections, :sections_limited
      scope.where("enrollments.course_section_id IN (?) OR (enrollments.limit_privileges_to_course_section=? AND enrollments.type IN ('TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment'))",
                  visibilities.pluck(:course_section_id),
                  false)
    when :restricted
      user_ids = visibilities.filter_map { |s| s[:associated_user_id] }
      scope.where(enrollments: { user_id: (user_ids + [user&.id]).compact })
    else
      scope.none
    end
  end

### course_section_visibility

See also MessageableUsers (same logic used to get users across multiple courses) (should refactor)
    case visibility
    when :full then scope
    when :sections then scope.where(enrollments: { course_section_id: visibilities.pluck(:course_section_id) })
    when :restricted then scope.where(enrollments: { user_id: (visibilities.filter_map { |s| s[:associated_user_id] } + [user]) })
    when :limited then scope.where(enrollments: { type: %w[StudentEnrollment TeacherEnrollment TaEnrollment StudentViewEnrollment] })
    when :sections_limited then scope.where(enrollments: { course_section_id: visibilities.pluck(:course_section_id) })
                                     .where(enrollments: { type: %w[StudentEnrollment TeacherEnrollment TaEnrollment StudentViewEnrollment] })
    else scope.none
    end
  end

  # returns :all or an array of section ids

### enrollment_visibility_level_for

return an empty set, but keep it as a scope for downstream consistency
      is_scope ? sections.none : []
    when Array
      is_scope ? sections.where(id: section_ids) : sections.select { |section| section_ids.include?(section.id) }
    end
  end

  # check_full is a hint that we don't care about the difference between :full and :limited,
  # so don't bother with an extra permission check to see if they have :full. Just return :limited.

### invited_count_visible_to

e.g. observer, can only see admins in the course
    return :restricted unless has_read_roster || has_admin

    if visibility_limited_to_section
      has_admin ? :sections : :sections_limited
    elsif has_admin
      :full
    else
      :limited
    end
  end

### self

`account_id.present?` is there to prevent a failure in `feature_enabled?`
    # if an account hasn't been set on the course yet
    if account_id.present? && feature_enabled?(:canvas_k6_theme) && super.nil?
      return canvas_k6_tab_configuration.map(&:with_indifferent_access)
    end

    super&.compact&.map(&:with_indifferent_access) || []
  end

  TAB_HOME = 0
  TAB_SYLLABUS = 1
  TAB_PAGES = 2
  TAB_ASSIGNMENTS = 3
  TAB_QUIZZES = 4
  TAB_GRADES = 5
  TAB_PEOPLE = 6
  TAB_GROUPS = 7
  TAB_DISCUSSIONS = 8
  TAB_MODULES = 10
  TAB_FILES = 11
  TAB_CONFERENCES = 12
  TAB_SETTINGS = 13
  TAB_ANNOUNCEMENTS = 14
  TAB_OUTCOMES = 15
  TAB_COLLABORATIONS = 16
  TAB_COLLABORATIONS_NEW = 17
  TAB_RUBRICS = 18
  TAB_SCHEDULE = 19
  TAB_COURSE_PACES = 20
  TAB_SEARCH = 21

  CANVAS_K6_TAB_IDS = [TAB_HOME, TAB_ANNOUNCEMENTS, TAB_GRADES, TAB_MODULES].freeze
  COURSE_SUBJECT_TAB_IDS = [TAB_HOME, TAB_SCHEDULE, TAB_MODULES, TAB_GRADES, TAB_GROUPS].freeze
  HORIZON_HIDDEN_TABS = [TAB_HOME, TAB_RUBRICS, TAB_OUTCOMES, TAB_COLLABORATIONS, TAB_COLLABORATIONS_NEW, TAB_DISCUSSIONS].freeze

### self

Add the unique TAB_SCHEDULE and TAB_GROUPS
    course_tabs.insert(1,
                       {
                         id: TAB_SCHEDULE,
                         label: t("#tabs.schedule", "Schedule"),
                         css_class: "schedule",
                         href: :course_path
                       },
                       {
                         id: TAB_GROUPS,
                         label: t("#tabs.groups", "Groups"),
                         css_class: "groups",
                         href: :course_groups_path,
                       })
    course_tabs.sort_by { |tab| COURSE_SUBJECT_TAB_IDS.index tab[:id] }
  end

### allow_wiki_comments

make sure t() is called before we switch to the secondary, in case we update the user's selected locale in the process
    course_subject_tabs = elementary_subject_course? && opts[:course_subject_tabs]
    default_tabs = if elementary_homeroom_course?
                     Course.default_homeroom_tabs
                   elsif course_subject_tabs
                     Course.course_subject_tabs
                   elsif elementary_subject_course?
                     Course.elementary_course_nav_tabs
                   elsif horizon_course?
                     Course.horizon_course_nav_tabs
                   else
                     Course.default_tabs
                   end

    if SmartSearch.smart_search_available?(self)
      default_tabs.insert(1,
                          {
                            id: TAB_SEARCH,
                            label: t("#tabs.search", "Smart Search"),
                            css_class: "search",
                            href: :course_search_path
                          })
    end

    if enable_course_paces && grants_any_right?(user, *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS)
      default_tabs.insert(default_tabs.index { |t| t[:id] == TAB_MODULES } + 1, {
                            id: TAB_COURSE_PACES,
                            label: t("#tabs.course_paces", "Course Pacing"),
                            css_class: "course_paces",
                            href: :course_course_pacing_path
                          })
    end

    # Remove already cached tabs for Horizon courses
    if horizon_course?
      default_tabs.delete_if do |tab|
        HORIZON_HIDDEN_TABS.include?(tab[:id])
      end
    end

    opts[:include_external] = false if elementary_homeroom_course?

    GuardRail.activate(:secondary) do
      # We will by default show everything in default_tabs, unless the teacher has configured otherwise.
      tabs = (elementary_subject_course? && !course_subject_tabs) ? [] : tab_configuration.compact
      home_tab = default_tabs.find { |t| t[:id] == TAB_HOME }
      settings_tab = default_tabs.find { |t| t[:id] == TAB_SETTINGS }
      external_tabs = if opts[:include_external]
                        external_tool_tabs(opts, user) + Lti::MessageHandler.lti_apps_tabs(self, [Lti::ResourcePlacement::COURSE_NAVIGATION], opts)
                      else
                        []
                      end
      item_banks_tab = Lti::ResourcePlacement.update_tabs_and_return_item_banks_tab(external_tabs)

      tabs = tabs.map do |tab|
        default_tab = default_tabs.find { |t| t[:id] == tab[:id] } || external_tabs.find { |t| t[:id] == tab[:id] }
        next unless default_tab

        tab[:label] = default_tab[:label]
        tab[:href] = default_tab[:href]
        tab[:css_class] = default_tab[:css_class]
        tab[:args] = default_tab[:args]
        tab[:visibility] = default_tab[:visibility]
        tab[:external] = default_tab[:external]
        tab[:icon] = default_tab[:icon]
        tab[:target] = default_tab[:target] if default_tab[:target]
        default_tabs.delete_if { |t| t[:id] == tab[:id] }
        external_tabs.delete_if { |t| t[:id] == tab[:id] }
        tab
      end
      tabs.compact!

      if course_subject_tabs
        # If we didn't have a saved position for Schedule, insert it in the 2nd position
        schedule_tab = default_tabs.detect { |t| t[:id] == TAB_SCHEDULE }
        tabs.insert(1, default_tabs.delete(schedule_tab)) if schedule_tab && !tabs.empty?
      end
      tabs += default_tabs
      tabs += external_tabs

      tabs.delete_if { |t| t[:id] == TAB_SETTINGS }
      if course_subject_tabs
        # Don't show Settings, ensure that all external tools are at the bottom (with the exception of Groups, which
        # should stick to the end unless it has been re-ordered)
        lti_tabs = tabs.filter { |t| t[:external] }
        tabs -= lti_tabs
        groups_tab = tabs.pop if tabs.last&.dig(:id) == TAB_GROUPS && !opts[:for_reordering]
        tabs += lti_tabs
        tabs << groups_tab if groups_tab
      else
        # Ensure that Settings is always at the bottom
        tabs << settings_tab if settings_tab
        # Ensure that Home is always at the top
        tabs.delete_if { |t| t[:id] == TAB_HOME }
        tabs.unshift home_tab if home_tab
      end

      if opts[:only_check]
        tabs = tabs.select { |t| opts[:only_check].include?(t[:id]) }
      end

      check_for_permission = lambda do |*permissions|
        permissions.any? do |permission|
          if opts[:precalculated_permissions]&.key?(permission)
            opts[:precalculated_permissions][permission]
          else
            grants_right?(user, opts[:session], permission)
          end
        end
      end

      delete_unless = lambda do |tabs_to_check, *permissions|
        matched_tabs = tabs.select { |t| tabs_to_check.include?(t[:id]) }
        tabs -= matched_tabs if matched_tabs.present? && !check_for_permission.call(*permissions)
      end

      tabs_that_can_be_marked_hidden_unused = [
        { id: TAB_MODULES, relation: :modules },
        { id: TAB_FILES, relation: :files },
        { id: TAB_QUIZZES, relation: :quizzes },
        { id: TAB_ASSIGNMENTS, relation: :assignments },
        { id: TAB_ANNOUNCEMENTS, relation: :announcements },
        { id: TAB_OUTCOMES, relation: :outcomes },
        { id: TAB_PAGES, relation: :pages, additional_check: -> { allow_student_wiki_edits } },
        { id: TAB_CONFERENCES, relation: :conferences, additional_check: -> { check_for_permission.call(:create_conferences) } },
        { id: TAB_DISCUSSIONS, relation: :discussions, additional_check: -> { allow_student_discussion_topics } }
      ].select { |hidable_tab| tabs.any? { |t| t[:id] == hidable_tab[:id] } }

      if course_subject_tabs
        # Show modules tab in k5 even if there's no modules (but not if its hidden)
        tabs_that_can_be_marked_hidden_unused.reject! { |t| t[:id] == TAB_MODULES }

        # Hide Groups tab for students if there are no groups
        unless grants_right?(user, :read_as_admin) || active_groups.exists?
          tabs.delete_if { |t| t[:id] == TAB_GROUPS }
        end
      end

      if tabs_that_can_be_marked_hidden_unused.present?
        ar_types = active_record_types(only_check: tabs_that_can_be_marked_hidden_unused.pluck(:relation))
        tabs_that_can_be_marked_hidden_unused.each do |t|
          if !ar_types[t[:relation]] && (!t[:additional_check] || !t[:additional_check].call)
            # that means there are none of this type of thing in the DB
            if opts[:include_hidden_unused] || opts[:for_reordering] || opts[:api]
              tabs.detect { |tab| tab[:id] == t[:id] }[:hidden_unused] = true
            else
              tabs.delete_if { |tab| tab[:id] == t[:id] }
            end
          end
        end
      end

      # remove tabs that the user doesn't have access to
      unless opts[:for_reordering]
        delete_unless.call([TAB_HOME, TAB_ANNOUNCEMENTS, TAB_PAGES, TAB_OUTCOMES, TAB_CONFERENCES, TAB_COLLABORATIONS, TAB_MODULES], :read, *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS)

        member_only_tabs = tabs.select { |t| t[:visibility] == "members" }
        tabs -= member_only_tabs if member_only_tabs.present? && !check_for_permission.call(:participate_as_student, :read_as_admin)

        delete_unless.call([TAB_ASSIGNMENTS, TAB_QUIZZES], :read, *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS)
        delete_unless.call([TAB_SYLLABUS], :read, :read_syllabus, *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS)

        admin_only_tabs = tabs.select { |t| t[:visibility] == "admins" }
        tabs -= admin_only_tabs if admin_only_tabs.present? && !check_for_permission.call(:read_as_admin)

        hidden_external_tabs = tabs.select do |t|
          next false unless t[:external]

          elementary_enabled = elementary_subject_course? && !course_subject_tabs
          (t[:hidden] && !elementary_enabled) || (elementary_enabled && tab_hidden?(t[:id]))
        end
        tabs -= hidden_external_tabs if hidden_external_tabs.present? && !(opts[:api] && check_for_permission.call(:read_as_admin))

        delete_unless.call([TAB_GRADES], :read_grades, :view_all_grades, :manage_grades)
        delete_unless.call([TAB_GROUPS], :read_roster)

        delete_unless.call([TAB_PEOPLE], :read_roster)
        delete_unless.call([TAB_DISCUSSIONS], :read_forum, :post_to_forum, :create_forum, :moderate_forum)
        delete_unless.call([TAB_SETTINGS], :read_as_admin)
        delete_unless.call([TAB_ANNOUNCEMENTS], :read_announcements)
        delete_unless.call([TAB_RUBRICS], :read_rubrics, :manage_rubrics)
        delete_unless.call([TAB_FILES], :read_files, *RoleOverride::GRANULAR_FILE_PERMISSIONS)

        if item_banks_tab &&
           !check_for_permission.call(*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS)
          tabs.reject! { |tab| tab[:id] == item_banks_tab[:id] }
        end
        # remove outcomes tab for logged-out users or non-students
        outcome_tab = tabs.detect { |t| t[:id] == TAB_OUTCOMES }
        tabs.delete(outcome_tab) if outcome_tab && (!user || !check_for_permission.call(*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, :participate_as_student, :read_as_admin))

        # remove hidden tabs from students
        additional_checks = {
          TAB_ASSIGNMENTS => [*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS],
          TAB_SYLLABUS => [*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS],
          TAB_QUIZZES => [*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS],
          TAB_GRADES => [:view_all_grades, :manage_grades],
          TAB_FILES => RoleOverride::GRANULAR_FILE_PERMISSIONS,
          TAB_DISCUSSIONS => [:moderate_forum],
          TAB_PEOPLE => RoleOverride::GRANULAR_MANAGE_USER_PERMISSIONS
        }

        tabs.reject! do |t|
          # tab shouldn't be shown to non-admins
          (t[:hidden] || t[:hidden_unused]) &&
            # not an admin user
            (!user || !check_for_permission.call(*RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS, :read_as_admin)) &&
            # can't do any of the additional things required
            (!additional_checks[t[:id]] || !check_for_permission.call(*additional_checks[t[:id]]))
        end
      end

      tabs
    end
  end

### elementary_enabled

unfortunately we decided to pluralize this in the API after the fact...
  # so now we pluralize it everywhere except the actual settings hash and
  # course import/export :(
  add_setting :hide_final_grade, alias: :hide_final_grades, boolean: true
  add_setting :hide_sections_on_course_users_page, boolean: true, default: false
  add_setting :hide_distribution_graphs, boolean: true
  add_setting :allow_final_grade_override, boolean: false, default: false
  add_setting :allow_student_discussion_topics, boolean: true, default: true
  add_setting :allow_student_discussion_editing, boolean: true, default: true
  add_setting :allow_student_forum_attachments, boolean: true, default: true
  add_setting :allow_student_discussion_reporting, boolean: true, default: true
  add_setting :allow_student_anonymous_discussion_topics, boolean: true, default: false
  add_setting :show_total_grade_as_points, boolean: true, default: false
  add_setting :filter_speed_grader_by_student_group, boolean: true, default: false
  add_setting :lock_all_announcements, boolean: true, default: false, inherited: true
  add_setting :large_roster, boolean: true, default: ->(c) { c.root_account.large_course_rosters? }
  add_setting :course_format
  add_setting :newquizzes_engine_selected
  add_setting :image_id
  add_setting :image_url
  add_setting :banner_image_id
  add_setting :banner_image_url
  add_setting :organize_epub_by_content_type, boolean: true, default: false
  add_setting :enable_offline_web_export, boolean: true, default: ->(c) { c.account.enable_offline_web_export? }
  add_setting :is_public_to_auth_users, boolean: true, default: false
  add_setting :overridden_course_visibility

  add_setting :restrict_quantitative_data, boolean: true, default: false
  add_setting :restrict_student_future_view, boolean: true, inherited: true
  add_setting :restrict_student_past_view, boolean: true, inherited: true

  add_setting :timetable_data, arbitrary: true
  add_setting :syllabus_master_template_id
  add_setting :syllabus_course_summary, boolean: true, default: true
  add_setting :syllabus_updated_at

  add_setting :enable_course_paces, boolean: true, default: false

  add_setting :usage_rights_required, boolean: true, default: false, inherited: true

  add_setting :course_color
  add_setting :alt_name

  add_setting :default_due_time, inherited: true
  add_setting :conditional_release, default: false, boolean: true, inherited: true
  add_setting :search_embedding_version, arbitrary: true

  add_setting :show_student_only_module_id
  add_setting :show_teacher_only_module_id

### restrict_quantitative_data

If the feature flag is off, then the setting is not visible nor has any effect
    return false unless feature_enabled
    # If the RQD setting is on and not locked, courses can turn it on and off at will
    return true if account_setting && !account_lock_state
    # If the course setting is off but the account setting is on and locked, then the course setting can be turned on
    return true if !course_setting && account_setting && account_lock_state
    # If the course setting is on, but the account setting is off, then the course can turn it off, but not back on
    return true if course_setting && !account_setting

    # Otherwise the RQD setting can not be changed
    false
  end

### friendly_name

When check_extra_permissions is true, return false for a teacher,ta, admin, or designer
    can_read_as_admin = if check_extra_permissions
                          grants_any_right?(
                            user,
                            :read_as_admin,
                            :manage_grades,
                            *RoleOverride::GRANULAR_MANAGE_ASSIGNMENT_PERMISSIONS,
                            *RoleOverride::GRANULAR_MANAGE_COURSE_CONTENT_PERMISSIONS
                          )
                        else
                          false
                        end
    is_account_admin = account.grants_right?(user, :manage)

    # never restrict quantitative data for admins
    root_account.feature_enabled?(:restrict_quantitative_data) && restrict_quantitative_data && !is_account_admin && !can_read_as_admin
  end

### user_can_manage_own_discussion_posts

roles don't apply across shards, so fall back to the base type
                              all_enrollments.find_or_initialize_by(type: enrollment.type, user_id: enrollment.user_id, associated_user_id: enrollment.associated_user_id)
                            end
        course_enrollment.workflow_state = enrollment.workflow_state
        course_enrollment.start_at = enrollment.start_at
        course_enrollment.end_at = enrollment.end_at
        course_enrollment.completed_at = enrollment.completed_at
        course_enrollment.save!
        progress.increment_completion!(1) if progress&.total
      end
    end
  end

### settings

frozen, because you should use setters

### user_list_search_mode_for

there's a unique constraint on this, so we need to clear it out
        self.self_enrollment_code = nil
        self.self_enrollment = false
        # The order here is important; we have to set our sis id to nil and save first
        # so that the new course can be saved, then we need the new course saved to
        # get its id to move over sections and enrollments.  Setting this course to
        # deleted has to be last otherwise it would set all the enrollments to
        # deleted before they got moved
        self.sis_source_id = self.sis_batch_id = self.integration_id = nil
        self.uuid = nil unless reset_uuid
        save!
        Course.process_as_sis { new_course.save! }
        course_sections.update_all(course_id: new_course.id)
        # we also want to bring along prior enrollments, so don't use the enrollments
        # association
        Enrollment.where(course_id: self).in_batches(of: 10_000).update_all(course_id: new_course.id, updated_at: Time.now.utc)
        user_ids = new_course.all_enrollments.pluck(:user_id)
        self.class.connection.after_transaction_commit do
          User.touch_and_clear_cache_keys(user_ids, :enrollments)
        end
        Shard.partition_by_shard(user_ids) do |sharded_user_ids|
          Favorite.where(user_id: sharded_user_ids, context_type: "Course", context_id: id)
                  .in_batches(of: 10_000).update_all(context_id: new_course.id, updated_at: Time.now.utc)
        end

        self.replacement_course_id = new_course.id
        self.workflow_state = "deleted"
        Course.suspend_callbacks(:copy_from_course_template) do
          save!
        end

        unless profile.new_record?
          profile.update_attribute(:context, new_course)
        end

        Course.find(new_course.id)
      end
    end
  end

### find_or_create_student_view_student

part of the way we isolate this fake student from places we don't want it
  # to appear is to ensure that it does not have a pseudonym or any
  # account_associations. if either of these conditions is false, something is
  # wrong.

### sync_enrollments

hash the unique_id so that it's hard to accidently enroll the user in
        # a course by entering something in a user list. :(
        fake_student.pseudonyms.create!(account: root_account,
                                        unique_id: Canvas::Security.hmac_sha1("Test Student_#{fake_student.id}"))
      end
      fake_student
    else
      student_view_students.active.first
    end
  end
  private :find_or_create_student_view_student

  # we want to make sure the student view student is always enrolled in all the
  # sections of the course, so that a section limited teacher can grade them.

### associated_shards

enroll fake_student will only create the enrollment if it doesn't already exist
        enroll_user(fake_student,
                    "StudentViewEnrollment",
                    allow_multiple_enrollments: true,
                    section:,
                    enrollment_state: "active",
                    no_notify: true,
                    skip_touch_user: true)
      end
    end
    SubmissionLifecycleManager.recompute_users_for_course(fake_student.id, self)
    fake_student.update_root_account_ids
    fake_student
  end
  private :sync_enrollments

### touch_content_if_public_visibility_changed

only send one

### clear_todo_list_cache_later

RUBY 2.7 this can go away (**{} will work at the caller)
    raise ArgumentError, "Only send one hash" if !changes.empty? && !kwargs.empty?

    changes = kwargs if changes.empty? && !kwargs.empty?

    if changes[:is_public] || changes[:is_public_to_auth_users]
      assignments.touch_all
      attachments.touch_all
      calendar_events.touch_all
      context_modules.touch_all
      discussion_topics.touch_all
      quizzes.touch_all
      wiki.touch
      wiki_pages.touch_all
    end
  end

### any_assignment_in_closed_grading_period

preload favorites and nicknames
    favorite_ids = preload_favorites && user.favorite_context_ids("Course")
    nicknames = user.all_course_nicknames(courses)
    courses.each do |course|
      course.preloaded_favorite = favorite_ids.include?(course.id) if favorite_ids
      # keys in nicknames are relative to the user's shard
      course.preloaded_nickname = nicknames[Shard.relative_id_for(course.id, course.shard, user.shard)]
    end
  end

### grading_periods

Does this course have grading periods?
  # checks for both legacy and account-level grading period groups

### gradebook_backwards_incompatible_features_enabled

This method will be around while we still have two
  # gradebooks. This method should be used in situations where we want
  # to identify the user can't move backwards, such as feature flags

### grading_standard_or_default

The old gradebook can't deal with late policies at all
    return true if late_policy&.missing_submission_deduction_enabled? ||
                   late_policy&.late_submission_deduction_enabled? ||
                   feature_enabled?(:final_grades_override)

    # If you've used the grade tray status changes at all, you can't
    # go back. Even if set to none, it'll break "Message Students
    # Who..." for unsubmitted.
    expire_time = Setting.get("late_policy_tainted_submissions", 1.hour).to_i
    Rails.cache.fetch(["late_policy_tainted_submissions", self].cache_key, expires_in: expire_time) do
      submissions.except(:order).where(late_policy_status: %w[missing late extended none]).exists?
    end
  end

### post_manually

A moderated assignment must have at least 1 (non-moderator) grader.
    return 1 if count < 2
    # grader count cannot exceed the hard limit
    return MODERATED_GRADING_GRADER_LIMIT if count > MODERATED_GRADING_GRADER_LIMIT + 1

    # for any given assignment: 1 assigned moderator + N max graders = all participating instructors
    # so N max graders = all participating instructors - 1 assigned moderator
    count - 1
  end

### changes_to_course_format

Get the settings changes into a parameter
    setting_changes = saved_changes[:settings]
    old_enable_paces_setting = setting_changes[0][:enable_course_paces]
    new_enable_paces_setting = setting_changes[1][:enable_course_paces]

    # Check to see if enable_course_paces is in list of updated items
    return false if new_enable_paces_setting.nil?

    # If enable_course_paces IS in the list, then check to see if the original value is present or if it's nil
    # It can be nil when a course is initially created and published without other settings present.
    # In this case, then, it's going from nil to a value we care about one way or the other.
    if old_enable_paces_setting.nil?
      return true
    end

    # Finally this is the case where the list of settings may include enable_course_paces, but it didn't change --
    # another setting changed.
    old_enable_paces_setting != new_enable_paces_setting
  end

### log_course_pacing_settings_update

Get the settings changes into a parameter
    setting_changes = saved_changes[:settings]
    old_course_format_setting = setting_changes[0][:course_format]
    new_course_format_setting = setting_changes[1][:course_format]

    old_course_format_setting != new_course_format_setting
  end

