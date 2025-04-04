# Submission

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

require "anonymity"

## Relationships

- belongs_to :attachment
- belongs_to :assignment
- belongs_to :course
- belongs_to :custom_grade_status
- has_many :observer_alerts
- has_many :lti_assets
- belongs_to :user
- belongs_to :grader
- belongs_to :proxy_submitter
- belongs_to :grading_period
- belongs_to :group
- belongs_to :media_object
- belongs_to :root_account
- belongs_to :quiz_submission
- has_many :all_submission_comments
- has_many :all_submission_comments_for_groups
- has_many :group_memberships
- has_many :submission_comments
- has_many :visible_submission_comments
- has_many :hidden_submission_comments
- has_many :assessment_requests
- has_many :assigned_assessments
- has_many :rubric_assessments
- has_many :attachment_associations
- has_many :provisional_grades
- has_many :originality_reports
- has_one :rubric_assessment
- has_one :lti_result
- has_many :submission_drafts
- has_many :conversation_messages
- has_many :content_participations
- has_many :canvadocs_annotation_contexts
- has_many :canvadocs_submissions
- has_many :auditor_grade_change_records

## Methods

### self

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

require "anonymity"

class Submission < ActiveRecord::Base
  include Canvas::GradeValidations
  include CustomValidations
  include SendToStream
  include Workflow

  GRADE_STATUS_MESSAGES_MAP = {
    success: {
      status: true
    }.freeze,
    account_admin: {
      status: true
    }.freeze,
    unpublished: {
      status: false,
      message: I18n.t("This assignment is still unpublished")
    }.freeze,
    not_autograded: {
      status: false,
      message: I18n.t("This submission is not being autograded")
    }.freeze,
    cant_manage_grades: {
      status: false,
      message: I18n.t("You don't have permission to manage grades for this course")
    }.freeze,
    assignment_in_closed_grading_period: {
      status: false,
      message: I18n.t("This assignment is in a closed grading period for this student")
    }.freeze,
    not_applicable: {
      status: false,
      message: I18n.t("This assignment is not applicable to this student")
    }.freeze,
    moderation_in_progress: {
      status: false,
      message: I18n.t("This assignment is currently being moderated")
    }.freeze
  }.freeze

  SUBMISSION_TYPES_GOVERNED_BY_ALLOWED_ATTEMPTS = %w[online_upload online_url online_text_entry].freeze
  VALID_STICKERS = %w[
    apple
    basketball
    bell
    book
    bookbag
    briefcase
    bus
    calendar
    chem
    design
    pencil
    beaker
    paintbrush
    computer
    column
    pen
    tablet
    telescope
    calculator
    paperclip
    composite_notebook
    scissors
    ruler
    clock
    globe
    grad
    gym
    mail
    microscope
    mouse
    music
    notebook
    page
    panda1
    panda2
    panda3
    panda4
    panda5
    panda6
    panda7
    panda8
    panda9
    presentation
    science
    science2
    star
    tag
    tape
    target
    trophy
  ].freeze

  attr_readonly :assignment_id
  attr_accessor :assignment_changed_not_sub,
                :grade_change_event_author_id,
                :grade_posting_in_progress,
                :grading_error_message,
                :override_lti_id_lock,
                :require_submission_type_is_valid,
                :saved_by,
                :score_unchanged,
                :skip_grade_calc,
                :skip_grader_check,
                :visible_to_user

  # This can be set to true to force late policy behaviour that would
  # be skipped otherwise. See #late_policy_relevant_changes? and
  # #score_late_or_none. It is reset to false in an after save so late
  # policy deductions don't happen again if the submission object is
  # saved again.
  attr_writer :regraded
  attr_writer :audit_grade_changes
  attr_writer :versioned_originality_reports

  belongs_to :attachment # this refers to the screenshot of the submission if it is a url submission
  belongs_to :assignment, inverse_of: :submissions, class_name: "AbstractAssignment"
  belongs_to :course, inverse_of: :submissions
  belongs_to :custom_grade_status, inverse_of: :submissions
  has_many :observer_alerts, as: :context, inverse_of: :context, dependent: :destroy
  has_many :lti_assets, class_name: "Lti::Asset", inverse_of: :submission, dependent: :nullify
  belongs_to :user
  alias_method :student, :user
  belongs_to :grader, class_name: "User"
  belongs_to :proxy_submitter, class_name: "User", optional: true
  belongs_to :grading_period, inverse_of: :submissions
  belongs_to :group
  belongs_to :media_object
  belongs_to :root_account, class_name: "Account"

  belongs_to :quiz_submission, class_name: "Quizzes::QuizSubmission"
  has_many :all_submission_comments, -> { order(:created_at) }, class_name: "SubmissionComment", dependent: :destroy
  has_many :all_submission_comments_for_groups, -> { for_groups.order(:created_at) }, class_name: "SubmissionComment"
  has_many :group_memberships, through: :assignment
  has_many :submission_comments, -> { order(:created_at).where(provisional_grade_id: nil) }
  has_many :visible_submission_comments,
           -> { published.visible.for_final_grade.order(:created_at, :id) },
           class_name: "SubmissionComment"
  has_many :hidden_submission_comments, -> { order("created_at, id").where(provisional_grade_id: nil, hidden: true) }, class_name: "SubmissionComment"
  has_many :assessment_requests, as: :asset
  has_many :assigned_assessments, class_name: "AssessmentRequest", as: :assessor_asset
  has_many :rubric_assessments, as: :artifact
  has_many :attachment_associations, as: :context, inverse_of: :context
  has_many :provisional_grades, class_name: "ModeratedGrading::ProvisionalGrade"
  has_many :originality_reports
  has_one :rubric_assessment,
          lambda {
            joins(:rubric_association)
              .where(assessment_type: "grading")
              .where(rubric_associations: { workflow_state: "active" })
          },
          as: :artifact,
          inverse_of: :artifact
  has_one :lti_result, inverse_of: :submission, class_name: "Lti::Result", dependent: :destroy
  has_many :submission_drafts, inverse_of: :submission, dependent: :destroy

  # we no longer link submission comments and conversations, but we haven't fixed up existing
  # linked conversations so this relation might be useful
  # TODO: remove this when removing the conversationmessage asset columns
  has_many :conversation_messages, as: :asset # one message per private conversation

  has_many :content_participations, as: :content

  has_many :canvadocs_annotation_contexts, inverse_of: :submission, dependent: :destroy
  has_many :canvadocs_submissions

  has_many :auditor_grade_change_records,
           class_name: "Auditors::ActiveRecord::GradeChangeRecord",
           dependent: :destroy,
           inverse_of: :submission

  serialize :turnitin_data, type: Hash

  validates :assignment_id, :user_id, presence: true
  validates :body, length: { maximum: maximum_long_text_length, allow_blank: true }
  validates :published_grade, length: { maximum: maximum_string_length, allow_blank: true }
  validates_as_url :url
  validates :points_deducted, numericality: { greater_than_or_equal_to: 0 }, allow_nil: true
  validates :seconds_late_override, numericality: { greater_than_or_equal_to: 0 }, allow_nil: true
  validates :extra_attempts, numericality: { greater_than_or_equal_to: 0 }, allow_nil: true
  validates :late_policy_status, inclusion: %w[none missing late extended], allow_nil: true
  validates :cached_tardiness, inclusion: ["missing", "late"], allow_nil: true
  validates :sticker, inclusion: { in: VALID_STICKERS }, allow_nil: true
  validate :ensure_grader_can_grade
  validate :extra_attempts_can_only_be_set_on_online_uploads
  validate :ensure_attempts_are_in_range, unless: :proxy_submission?
  validate :submission_type_is_valid, if: :require_submission_type_is_valid
  validate :preserve_lti_id, on: :update

  scope :active, -> { where("submissions.workflow_state <> 'deleted'") }
  scope :deleted, -> { where("submissions.workflow_state = 'deleted'") }
  scope :for_enrollments, ->(enrollments) { where(user_id: enrollments.select(:user_id)) }
  scope :with_comments, -> { preload(:submission_comments) }
  scope :unread_for, lambda { |user_id|
    joins(:content_participations)
      .where(user_id:, content_participations: { workflow_state: "unread", user_id: })
  }
  scope :after, ->(date) { where("submissions.created_at>?", date) }
  scope :before, ->(date) { where("submissions.created_at<?", date) }
  scope :submitted_before, ->(date) { where("submitted_at<?", date) }
  scope :submitted_after, ->(date) { where("submitted_at>?", date) }
  scope :with_point_data, -> { where("submissions.score IS NOT NULL OR submissions.grade IS NOT NULL") }

  scope :postable, lambda {
    all.primary_shard.activate do
      graded.union(with_hidden_comments)
    end
  }
  scope :with_hidden_comments, lambda {
    where(SubmissionComment.where("submission_id = submissions.id AND hidden = true").arel.exists)
  }

  # This should only be used in the course drop down to show assignments recently graded.
  scope :recently_graded_assignments, lambda { |user_id, date, limit|
    select("assignments.id, assignments.title, assignments.points_possible, assignments.due_at,
            submissions.grade, submissions.score, submissions.graded_at, assignments.grading_type,
            assignments.context_id, assignments.context_type, courses.name AS context_name")
      .joins(:assignment)
      .joins("JOIN #{Course.quoted_table_name} ON courses.id=assignments.context_id")
      .where("graded_at>? AND user_id=? AND muted=?", date, user_id, false)
      .order("graded_at DESC")
      .limit(limit)
  }

  scope :for_course, ->(course) { where(course_id: course) }
  scope :for_assignment, ->(assignment) { where(assignment:) }

  scope :excused, -> { where(excused: true) }

  scope :missing, lambda {
    joins(:assignment)
      .where(<<~SQL.squish)
        /* excused submissions cannot be missing */
        excused IS NOT TRUE
        AND custom_grade_status_id IS NULL
        AND (late_policy_status IS DISTINCT FROM 'extended')
        AND NOT (
          /* teacher said it's missing, 'nuff said. */
          /* we're doing a double 'NOT' here to avoid 'ORs' that could slow down the query */
          late_policy_status IS DISTINCT FROM 'missing' AND NOT
          (
            cached_due_date IS NOT NULL
            /* submission is past due and */
            AND CURRENT_TIMESTAMP >= cached_due_date +
              CASE assignments.submission_types WHEN 'online_quiz' THEN interval '1 minute' ELSE interval '0 minutes' END
            /* submission is not submitted and */
            AND submission_type IS NULL
            /* we expect a digital submission */
            AND NOT (
              cached_quiz_lti IS NOT TRUE AND
              assignments.submission_types IN ('', 'none', 'not_graded', 'on_paper', 'wiki_page', 'external_tool')
            )
            AND assignments.submission_types IS NOT NULL
            AND NOT (
              late_policy_status IS NULL
              AND grader_id IS NOT NULL
            )
          )
        )
      SQL
  }

  scope :late, lambda {
    left_joins(:quiz_submission).where(<<~SQL.squish)
      submissions.excused IS NOT TRUE
      AND submissions.custom_grade_status_id IS NULL
      AND (
        submissions.late_policy_status = 'late' OR
        (submissions.late_policy_status IS NULL AND submissions.submitted_at >= submissions.cached_due_date +
           CASE submissions.submission_type WHEN 'online_quiz' THEN interval '1 minute' ELSE interval '0 minutes' END
           AND (submissions.quiz_submission_id IS NULL OR quiz_submissions.workflow_state = 'complete'))
      )
    SQL
  }

  scope :not_late, lambda {
    left_joins(:quiz_submission).where(<<~SQL.squish)
      submissions.excused IS TRUE
      OR submissions.custom_grade_status_id IS NOT NULL
      OR (late_policy_status IS NOT DISTINCT FROM 'extended')
      OR (
        submissions.late_policy_status is distinct from 'late' AND
        (submissions.submitted_at IS NULL OR submissions.cached_due_date IS NULL OR
          submissions.submitted_at < submissions.cached_due_date +
            CASE submissions.submission_type WHEN 'online_quiz' THEN interval '1 minute' ELSE interval '0 minutes' END
          OR quiz_submissions.workflow_state <> 'complete')
      )
    SQL
  }

  GradedAtBookmarker = BookmarkedCollection::SimpleBookmarker.new(Submission, :graded_at)
  IdBookmarker = BookmarkedCollection::SimpleBookmarker.new(Submission, :id)

  scope :anonymized, -> { where.not(anonymous_id: nil) }
  scope :due_in_past, -> { where(cached_due_date: ..Time.now.utc) }

  scope :posted, -> { where.not(posted_at: nil) }
  scope :unposted, -> { where(posted_at: nil) }

  scope :in_current_grading_period_for_courses, lambda { |course_ids|
    current_period_clause = ""
    course_ids.uniq.each_with_index do |course_id, i|
      grading_period_id = GradingPeriod.current_period_for(Course.find(course_id))&.id
      current_period_clause += grading_period_id.nil? ? sanitize_sql(["course_id = ?", course_id]) : sanitize_sql(["(course_id = ? AND grading_period_id = ?)", course_id, grading_period_id])
      current_period_clause += " OR " if i < course_ids.length - 1
    end
    where(current_period_clause)
  }

  workflow do
    state :submitted do
      event :grade_it, transitions_to: :graded
    end
    state :unsubmitted
    state :pending_review
    state :graded
    state :deleted
  end
  alias_method :needs_review?, :pending_review?

  delegate :auditable?, to: :assignment, prefix: true
  delegate :can_be_moderated_grader?, to: :assignment, prefix: true

### self

see #needs_grading?
  # When changing these conditions, update index_submissions_needs_grading to
  # maintain performance.

### needs_grading

see .needs_grading_conditions

### reset_regraded

Because set_anonymous_id makes database calls, delay it until just before
  # validation. Otherwise if we place it in any earlier (e.g.
  # before/after_initialize), every Submission.new will make database calls.
  before_validation :set_anonymous_id, if: :new_record?
  before_save :set_status_attributes
  before_save :apply_late_policy, if: :late_policy_relevant_changes?
  before_save :update_if_pending
  before_save :validate_single_submission, :infer_values
  before_save :prep_for_submitting_to_plagiarism
  before_save :check_is_new_attempt
  before_save :check_reset_graded_anonymously
  before_save :set_root_account_id
  before_save :reset_redo_request
  before_save :remove_sticker, if: :will_save_change_to_attempt?
  before_save :clear_body_word_count, if: -> { body.nil? }
  before_save :set_lti_id
  after_save :update_body_word_count_later, if: -> { saved_change_to_body? && get_word_count_from_body? }
  after_save :touch_user
  after_save :clear_user_submissions_cache
  after_save :touch_graders
  after_save :update_assignment
  after_save :update_attachment_associations
  after_save :submit_attachments_to_canvadocs
  after_save :queue_websnap
  after_save :aggregate_checkpoint_submissions, if: :checkpoint_changes?
  after_save :update_final_score
  after_save :submit_to_plagiarism_later
  after_save :update_admins_if_just_submitted
  after_save :check_for_media_object
  after_save :update_quiz_submission
  after_save :update_participation
  after_save :update_line_item_result
  after_save :delete_ignores
  after_save :create_alert
  after_save :reset_regraded
  after_save :create_audit_event!
  after_save :handle_posted_at_changed, if: :saved_change_to_posted_at?
  after_save :delete_submission_drafts!, if: :saved_change_to_attempt?
  after_save :send_timing_data_if_needed

### needs_grading_count_updated

AutoGrader == (quiz_id * -1)
    !!(grader_id && grader_id < 0)
  end

  after_create :needs_grading_count_updated, if: :needs_grading?
  after_update :needs_grading_count_updated, if: :needs_grading_changed?
  after_update :update_planner_override

### new_version_needed

unless it's an auto-graded quiz
      return unless workflow_state_before_last_save == "unsubmitted"
    else
      return unless workflow_state == "submitted"
    end
    PlannerHelper.complete_planner_override_for_submission(self)
  end

  attr_reader :group_broadcast_submission

  has_a_broadcast_policy

  simply_versioned explicit: true,
                   when: ->(model) { model.new_version_needed? },
                   on_create: ->(_model, version) { SubmissionVersion.index_version(version) },
                   on_load: ->(model, version) { model&.cached_due_date = version.versionable&.cached_due_date }

  # This needs to be after simply_versioned because the grade change audit uses
  # versioning to grab the previous grade.
  after_save :grade_change_audit

### observer

non-deleted students in accounts with limited access setting enabled should not be able to comment on submissions
    given do |user|
      user &&
        user.id == user_id &&
        assignment.published? &&
        !course.account.limited_access_for_user?(user)
    end
    can :comment

    # see user_can_read_grade? before editing :read_grade permissions
    given do |user|
      user &&
        user.id == user_id &&
        !hide_grade_from_student?
    end
    can :read_grade

    given do |user, session|
      assignment.published? &&
        assignment.context.grants_right?(user, session, :manage_grades)
    end
    can :read and can :comment and can :make_group_comment and can :read_grade and can :read_comments

    given do |user, _session|
      can_grade?(user)
    end
    can :grade

    given do |user, session|
      assignment.user_can_read_grades?(user, session)
    end
    can :read and can :read_grade

    given do |user|
      assignment&.context &&
        user &&
        self.user &&
        assignment.context.observer_enrollments.where(
          user_id: user,
          associated_user_id: self.user,
          workflow_state: "active"
        ).exists?
    end
    can :read and can :read_comments

    given do |user|
      assignment &&
        posted? &&
        assignment.context &&
        user &&
        self.user &&
        assignment.context.observer_enrollments.where(
          user_id: user,
          associated_user_id: self.user,
          workflow_state: "active"
        ).first.try(:grants_right?, user, :read_grades)
    end
    can :read_grade

    given { |user| peer_reviewer?(user) && !!assignment&.submitted?(user:) }
    can :read and can :comment and can :make_group_comment

    given { |user, session| can_view_plagiarism_report("turnitin", user, session) }
    can :view_turnitin_report

    given { |user, session| can_view_plagiarism_report("vericite", user, session) }
    can :view_vericite_report
  end

### user_can_read_grade

first filter by submissions for the requested reviewer
        user.id == submission.user_id &&
          submission.assigned_assessments
      end.any? do |submission|
        # next filter the assigned assessments by the submission user_id being reviewed
        submission.assigned_assessments.any? { |review| user_id == review.user_id }
      end
  end

### can_read_submission_user_name

improves performance by checking permissions on the assignment before the submission
    return true if assignment.user_can_read_grades?(user, session)
    return false if hide_grade_from_student?(for_plagiarism:)
    return true if user && user.id == user_id # this is fast, so skip the policy cache check if possible

    grants_right?(user, session, :read_grade)
  end

  on_update_send_to_streams do
    if graded_at && graded_at > 5.minutes.ago && !@already_sent_to_stream
      @already_sent_to_stream = true
      user_id
    end
  end

### create_alert

trigger assignments have to wait for ConditionalRelease::OverrideHandler#handle_grade_change
        assignment&.delay_if_production&.multiple_module_actions([user_id], :scored, score)
      end
    end
    true
  end

### turnitin_report_url

check all assets in the turnitin_data (self.turnitin_assets is only the
    # current assets) so that we get the status for assets of previous versions
    # of the submission as well
    self.turnitin_data.each_key do |asset_string|
      data = self.turnitin_data[asset_string]
      next unless data.is_a?(Hash) && data[:object_id]

      if data[:similarity_score].blank?
        if attempt < TURNITIN_STATUS_RETRY
          turnitin ||= Turnitin::Client.new(*context.turnitin_settings)
          res = turnitin.generateReport(self, asset_string)
          if res[:similarity_score]
            data[:similarity_score] = res[:similarity_score].to_f
            data[:web_overlap] = res[:web_overlap].to_f
            data[:publication_overlap] = res[:publication_overlap].to_f
            data[:student_overlap] = res[:student_overlap].to_f
            data[:state] = Turnitin.state_from_similarity_score data[:similarity_score]
            data[:status] = "scored"
          else
            needs_retry ||= true
          end
        else
          data[:status] = "error"
          data[:public_error_message] = I18n.t("turnitin.no_score_after_retries", "Turnitin has not returned a score after %{max_tries} attempts to retrieve one.", max_tries: TURNITIN_RETRY)
        end
      else
        data[:status] = "scored"
      end
      self.turnitin_data[asset_string] = data
    end

    delay(run_at: (2**attempt).minutes.from_now).check_turnitin_status(attempt + 1) if needs_retry
    turnitin_data_changed!
    save
  end

### originality_data

Make sure the assignment exists and user is enrolled
    assignment_created = assignment.create_in_turnitin
    turnitin_enrollment = turnitin.enrollStudent(context, user)
    if assignment_created && turnitin_enrollment.success?
      delete_turnitin_errors
    else
      if attempt < TURNITIN_RETRY
        delay(run_at: 5.minutes.from_now, **TURNITIN_JOB_OPTS).submit_to_turnitin(attempt + 1)
      else
        assignment_error = assignment.turnitin_settings[:error]
        self.turnitin_data[:status] = "error"
        self.turnitin_data[:assignment_error] = assignment_error if assignment_error.present?
        self.turnitin_data[:student_error] = turnitin_enrollment.error_hash if turnitin_enrollment.error?
        turnitin_data_changed!
        save
      end
      return false
    end

    # Submit the file(s)
    submission_response = turnitin.submitPaper(self)
    submission_response.each do |res_asset_string, response|
      self.turnitin_data[res_asset_string].merge!(response)
      turnitin_data_changed!
      if !response[:object_id] && attempt >= TURNITIN_RETRY
        self.turnitin_data[res_asset_string][:status] = "error"
      end
    end

    delay(run_at: 5.minutes.from_now, **TURNITIN_JOB_OPTS).check_turnitin_status
    save

    # Schedule retry if there were failures
    submit_status = submission_response.present? && submission_response.values.all? { |v| v[:object_id] }
    unless submit_status
      delay(run_at: 5.minutes.from_now, **TURNITIN_JOB_OPTS).submit_to_turnitin(attempt + 1) if attempt < TURNITIN_RETRY
      return false
    end

    true
  end

  # This method pulls data from the OriginalityReport table
  # Preload OriginalityReport before using this method in a collection of submissions

### originality_reports_for_display

Returns an array of the versioned originality reports in a sorted order. The ordering goes
  # from least preferred report to most preferred reports, assuming there are reports that share
  # the same submission and attachment combination. Otherwise, the ordering can be safely ignored.
  #
  # @return [Array<OriginalityReport>]

### originality_report_url

Preload OriginalityReport before using this method

### has_originality_report

This ordering ensures that if multiple reports exist for this submission and attachment combo,
    # we grab the desired report. This is the reversed ordering of
    # OriginalityReport::PREFERRED_STATE_ORDER
    report = scope.where(attachment: requested_attachment).order(Arel.sql("CASE
      WHEN workflow_state = 'scored' THEN 0
      WHEN workflow_state = 'error' THEN 1
      WHEN workflow_state = 'pending' THEN 2
      END"),
                                                                 updated_at: :desc).first
    report&.report_launch_path(assignment)
  end

### vericite_data

VeriCite

  # this function will check if the score needs to be updated and update/save the new score if so,
  # otherwise, it just returns the vericite_data_hash

### vericite_data_hash

check to see if the score is stale, if so, fetch it again
    update_scores = false
    if Canvas::Plugin.find(:vericite).try(:enabled?) && !readonly? && lookup_data
      self.vericite_data_hash.each_value do |data|
        next unless data.is_a?(Hash) && data[:object_id]

        update_scores ||= vericite_recheck_score(data)
      end
      # we have found at least one score that is stale, call VeriCite and save the results
      if update_scores
        check_vericite_status(0)
      end
    end
    unless self.vericite_data_hash.empty?
      # only set vericite provider flag if the hash isn't empty
      self.vericite_data_hash[:provider] = :vericite
    end
    self.vericite_data_hash
  end

### vericite_recheck_score

use the same backend structure to store "content review" data
    self.turnitin_data
  end

  # this function looks at a vericite data object and determines whether the score needs to be rechecked (i.e. cache for 20 mins)

### check_vericite_status

only recheck scores if an old score exists
    unless data[:similarity_score_time].blank?
      now = Time.now.to_i
      score_age = Time.now.to_i - data[:similarity_score_time]
      score_cache_time = 1200 # by default cache scores for 20 mins
      # change the cache based on how long it has been since the paper was submitted
      # if !data[:submit_time].blank? && (now - data[:submit_time]) > 86400
      # # it has been more than 24 hours since this was submitted, increase cache time
      #   score_cache_time = 86400
      # end
      # only cache the score for 20 minutes or 24 hours based on when the paper was submitted
      if score_age > score_cache_time
        # check if we just recently requested this score
        last_checked = 1000 # default to a high number so that if it is not set, it won't effect the outcome
        unless data[:similarity_score_check_time].blank?
          last_checked = now - data[:similarity_score_check_time]
        end
        # only update if we didn't just ask VeriCite for the scores 20 seconds again (this is in the case of an error, we don't want to keep asking immediately)
        if last_checked > 20
          update_scores = true
        end
      end
    end
    update_scores
  end

  VERICITE_STATUS_RETRY = 16 # this caps the retries off at 36 hours (checking once every 4 hours)

### vericite_report_url

check all assets in the vericite_data (self.vericite_assets is only the
    # current assets) so that we get the status for assets of previous versions
    # of the submission as well

    # flag to make sure that all scores are just updates and not new
    recheck_score_all = true
    data_changed = false
    self.vericite_data_hash.each do |asset_string, data|
      # keep track whether the score state changed
      data_orig = data.dup
      next unless data.is_a?(Hash) && data[:object_id]

      # check to see if the score is stale, if so, delete it and fetch again
      recheck_score = vericite_recheck_score(data)
      # keep track whether all scores are updates or if any are new
      recheck_score_all &&= recheck_score
      # look up scores if:
      if recheck_score || data[:similarity_score].blank?
        if attempt < VERICITE_STATUS_RETRY
          data[:similarity_score_check_time] = Time.now.to_i
          vericite ||= VeriCite::Client.new
          res = vericite.generateReport(self, asset_string)
          if res[:similarity_score]
            # keep track of when we updated the score so that we can ask VC again once it is stale (i.e. cache for 20 mins)
            data[:similarity_score_time] = Time.now.to_i
            data[:similarity_score] = res[:similarity_score].to_i
            data[:state] = VeriCite.state_from_similarity_score data[:similarity_score]
            data[:status] = "scored"
            # since we have a score, we know this report shouldn't have any errors, clear them out
            data = clear_vericite_errors(data)
          else
            needs_retry ||= true
          end
        elsif !recheck_score # if we already have a score, continue to use it and do not set an error
          data[:status] = "error"
          data[:public_error_message] = I18n.t("vericite.no_score_after_retries", "VeriCite has not returned a score after %{max_tries} attempts to retrieve one.", max_tries: VERICITE_RETRY)
        end
      else
        data[:status] = "scored"
      end
      self.vericite_data_hash[asset_string] = data
      data_changed = data_changed ||
                     data_orig[:similarity_score] != data[:similarity_score] ||
                     data_orig[:state] != data[:state] ||
                     data_orig[:status] != data[:status] ||
                     data_orig[:public_error_message] != data[:public_error_message]
    end

    if !self.vericite_data_hash.empty? && self.vericite_data_hash[:provider].nil?
      # only set vericite provider flag if the hash isn't empty
      self.vericite_data_hash[:provider] = :vericite
      data_changed = true
    end
    retry_mins = 2**attempt
    if retry_mins > 240
      # cap the retry max wait to 4 hours
      retry_mins = 240
    end
    # if attempt <= 0, then that means no retries should be attempted
    delay(run_at: retry_mins.minutes.from_now).check_vericite_status(attempt + 1) if attempt > 0 && needs_retry
    # if all we did was recheck scores, do not version this save (i.e. increase the attempt number)
    if data_changed
      vericite_data_changed!
      if recheck_score_all
        with_versioning(false, &:save!)
      else
        save
      end
    end
  end

### vericite_assets

Make sure the assignment exists and user is enrolled
    assignment_created = assignment.create_in_vericite
    # vericite_enrollment = vericite.enrollStudent(self.context, self.user)
    if assignment_created
      delete_vericite_errors
    else
      assignment_error = assignment.vericite_settings[:error]
      self.vericite_data_hash[:assignment_error] = assignment_error if assignment_error.present?
      # self.vericite_data_hash[:student_error] = vericite_enrollment.error_hash if vericite_enrollment.error?
      vericite_data_changed!
      unless self.vericite_data_hash.empty?
        # only set vericite provider flag if the hash isn't empty
        self.vericite_data_hash[:provider] = :vericite
      end
      save
    end
    # even if the assignment didn't save, VeriCite will still allow this file to be submitted
    # Submit the file(s)
    submission_response = vericite.submitPaper(self)
    # VeriCite will not resubmit a file if it already has a similarity_score (i.e. success)
    update = false
    submission_response.each do |res_asset_string, response|
      update = true
      self.vericite_data_hash[res_asset_string].merge!(response)
      # keep track of when we first submitted
      self.vericite_data_hash[res_asset_string][:submit_time] = Time.now.to_i if self.vericite_data_hash[res_asset_string][:submit_time].blank?
      vericite_data_changed!
      if !response[:object_id] && attempt >= VERICITE_RETRY
        self.vericite_data_hash[res_asset_string][:status] = "error"
      elsif response[:object_id]
        # success, make sure any error messages are cleared
        self.vericite_data_hash[res_asset_string] = clear_vericite_errors(self.vericite_data_hash[res_asset_string])
      end
    end
    # only save if there were newly submitted attachments
    if update
      delay(run_at: 5.minutes.from_now, **VERICITE_JOB_OPTS).check_vericite_status
      unless self.vericite_data_hash.empty?
        # only set vericite provider flag if the hash isn't empty
        self.vericite_data_hash[:provider] = :vericite
      end
      save

      # Schedule retry if there were failures
      submit_status = submission_response.present? && submission_response.values.all? { |v| v[:object_id] }
      unless submit_status
        delay(run_at: 5.minutes.from_now, **VERICITE_JOB_OPTS).submit_to_vericite(attempt + 1) if attempt < VERICITE_RETRY
        return false
      end
    end

    true
  end

### vericiteable

only set vericite provider flag if the hash isn't empty
      self.vericite_data_hash[:provider] = :vericite
    end

    @submit_to_vericite = true
    save
  end

### plagiarism_service_to_use

End VeriCite

  # Plagiarism functions:

### prep_for_submitting_to_plagiarism

Because vericite is new and people are moving to vericite, not
    # moving from vericite to turnitin, we'll give vericite precedence
    # for now.
    @plagiarism_service_to_use = if Canvas::Plugin.find(:vericite).try(:enabled?)
                                   :vericite
                                 elsif !context.turnitin_settings.nil?
                                   :turnitin
                                 end
  end

### tool_default_query_params

End Plagiarism functions

### submitted_at

If an object is pulled from a simply_versioned yaml it may not have a submitted at.
  # submitted_at is needed by SpeedGrader, so it is set to the updated_at value

### not_submitted

A student that has not submitted but has been graded will have a workflow_state of "graded".
  # In that case, we can check the submission_type to see if the student has submitted or not.

### annotation_context

associate previewable-document and submission for permission checks
        if a.canvadocable? && Canvadocs.annotations_supported?
          submit_to_canvadocs = true
          a.create_canvadoc! unless a.canvadoc
          a.shard.activate do
            CanvadocsSubmission.find_or_create_by(submission_id: id, canvadoc_id: a.canvadoc.id)
          end
        elsif a.crocodocable?
          submit_to_canvadocs = true
          a.create_crocodoc_document! unless a.crocodoc_document
          a.shard.activate do
            CanvadocsSubmission.find_or_create_by(submission_id: id, crocodoc_document_id: a.crocodoc_document.id)
          end
        end

        next unless submit_to_canvadocs

        opts = {
          preferred_plugins: [Canvadocs::RENDER_PDFJS, Canvadocs::RENDER_BOX, Canvadocs::RENDER_CROCODOC],
          wants_annotation: true,
        }

        if context.root_account.settings[:canvadocs_prefer_office_online]
          # Office 365 should take priority over pdfjs
          opts[:preferred_plugins].unshift Canvadocs::RENDER_O365
        end

        a.delay(
          n_strand: "canvadocs",
          priority: Delayed::LOW_PRIORITY
        )
         .submit_to_canvadocs(1, **opts)
      end
    end
  end

### infer_values

New Quizzes returned a partial grade, but manual review is needed from a human
    return workflow_state if pending_review? && cached_quiz_lti

    inferred_state = "submitted" if unsubmitted? && submitted_at
    inferred_state = "unsubmitted" if submitted? && !has_submission?
    inferred_state = "graded" if grade && score && grade_matches_current_submission
    inferred_state = "pending_review" if infer_review_needed?

    inferred_state
  end

### just_submitted

I think the idea of having unpublished scores is unnecessarily confusing.
    # It may be that we want to have that functionality later on, but for now
    # I say it's just confusing.
    # if self.assignment && self.assignment.published?
    begin
      self.published_score = score
      self.published_grade = grade
    end
    true
  end

### check_is_new_attempt

since vericite_data is a function, make sure you are cloning the most recent vericite_data_hash
        if vericiteable?
          model.turnitin_data = vericite_data(true)
        # only use originality data if it's loaded, we want to avoid making N+1 queries
        elsif association(:originality_reports).loaded?
          model.turnitin_data = originality_data
        end

        if model.submitted_at && last_submitted_at.to_i != model.submitted_at.to_i
          res << (include_version ? { model:, version: } : model)
          last_submitted_at = model.submitted_at
        end
      end

      if res.empty?
        res = versions.to_a[0, 1].map do |version|
          include_version ? { version:, model: version.model } : version.model
        end
      end

      if res.empty?
        res = include_version ? [{ model: self, version: nil }] : [self]
      end

      res.sort_by do |entry|
        sub = include_version ? entry.fetch(:model) : entry
        sub.submitted_at || CanvasSort::First
      end
    end
  end

### extra_attempts_can_only_be_set_on_online_uploads

the grade permission is cached, which seems to be OK as the user's cache_key changes when
    # an assignment is published. can_autograde? does not depend on a user so cannot be made
    # into permission that would be cached.
    return true if grants_right?(grader, :grade)

    false
  end

### versioned_attachments

Turns out the database stores timestamps with 9 decimal places, but Ruby/Rails only serves
    # up 6 (plus three zeros). However, submission versions (when deserialized into a Submission
    # model) like to show 9.
    # This logic is duplicated in the bulk_load_versioned_originality_reports method
    @versioned_originality_reports ||=
      if submitted_at.nil?
        []
      else
        originality_reports.select do |o|
          o.submission_time&.iso8601(6) == submitted_at&.iso8601(6) ||
            # ...and sometimes originality reports don't have submission times, so we're doing our
            # best to guess based on attachment_id (or the lack) and creation times
            (o.attachment_id.present? && attachment_ids&.split(",")&.include?(o.attachment_id.to_s)) ||
            (o.submission_time.nil? && o.created_at > submitted_at &&
              (attachment_ids&.split(",").presence || [""]).include?(o.attachment_id.to_s))
        end
      end
  end

### self

This helper method is used by the bulk_load_versioned_* methods

### self

The index of the submission is considered part of the key for
    # the hash that is built. This is needed for bulk loading
    # submission_histories where multiple submission histories will
    # look equal to the Hash key and the attachments for the last one
    # will cancel out the former ones.
    submissions_with_index_and_attachment_ids = submissions.each_with_index.map do |s, index|
      attachment_ids = (s.attachment_ids || "").split(",").map(&:to_i)
      attachment_ids << s.attachment_id if s.attachment_id
      [[s, index], attachment_ids]
    end
    submissions_with_index_and_attachment_ids.to_h
  end
  private_class_method :group_attachment_ids_by_submission_and_index

  # use this method to pre-load the versioned_attachments for a bunch of
  # submissions (avoids having O(N) attachment queries)
  # NOTE: all submissions must belong to the same shard

### self

use this method to pre-load the versioned_originality_reports for a bunch of
  # submissions (avoids having O(N) originality report queries)
  # NOTE: all submissions must belong to the same shard

### self

nil for originality reports with no submission time
      reports.dig(s.id, :by_attachment)&.each do |attach_id, reports_for_attach_id|
        # Handles the following cases:
        # 1) student submits same attachment multiple times. There will only be
        #    one originality report for each unique attachment. The originality
        #    report has a submission_time but it will be submission time of the
        #    first submission, so we need to match up by attachment ids.
        # 2) The originality report does not have a submission time. We link up
        #    via attachment id or lack of attachment id. That isn't particularly
        #    specific to the submission version. We don't have a good way of
        #    matching them (though at least in the case of using the same Canvas
        #    attachment id, it should be the same document) In submission
        #    histories, we're just giving all of the originality reports we can't
        #    rule out, but we can at least rule out any report that was created
        #    before a new submission as belonging to that submission

        if attach_id.present? && s.attachment_ids&.split(",")&.include?(attach_id.to_s)
          reports_for_sub += reports_for_attach_id
        elsif attach_id.blank? && s.attachment_ids.blank?
          # Sub and originality report both missing attachment ids -- add
          # just originality reports with submission_time is nil
          reports_for_sub += reports_for_attach_id.select { |r| r.submission_time.blank? && r.created_at > s.submitted_at }
        end
      end
      s.versioned_originality_reports = reports_for_sub.uniq
    end
  end

### self

Avoids having O(N) attachment queries.  Returns a hash of
  # submission to attachments.

### assignment_graded_in_the_last_hour

Submission:
  #   Online submission submitted AFTER the due date (notify the teacher) - "Grade Changes"
  #   Submission graded (or published) - "Grade Changes"
  #   Grade changed - "Grade Changes"
  set_broadcast_policy do |p|
    p.dispatch :assignment_submitted_late
    p.to { assignment.context.instructors_in_charge_of(user_id) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_assignment_submitted_late?
    end
    p.data { course_broadcast_data }

    p.dispatch :assignment_submitted
    p.to { assignment.context.instructors_in_charge_of(user_id) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_assignment_submitted?
    end
    p.data { course_broadcast_data }

    p.dispatch :assignment_resubmitted
    p.to { assignment.context.instructors_in_charge_of(user_id) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_assignment_resubmitted?
    end
    p.data { course_broadcast_data }

    p.dispatch :group_assignment_submitted_late
    p.to { assignment.context.instructors_in_charge_of(user_id) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_group_assignment_submitted_late?
    end
    p.data { course_broadcast_data }

    p.dispatch :submission_graded
    p.to { [student] + User.observing_students_in_course(student, assignment.context) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_submission_graded?
    end
    p.data { course_broadcast_data }

    p.dispatch :submission_grade_changed
    p.to { [student] + User.observing_students_in_course(student, assignment.context) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_submission_grade_changed?
    end
    p.data { course_broadcast_data }

    p.dispatch :submission_posted
    p.to { [student] + User.observing_students_in_course(student, assignment.context) }
    p.whenever do |submission|
      BroadcastPolicies::SubmissionPolicy.new(submission)
                                         .should_dispatch_submission_posted?
    end
    p.data { course_broadcast_data }
  end

### validate_single_submission

Accept attachments that were already approved, those that were just created
    # or those that were part of some outside context.  This is all to prevent
    # one student from sneakily getting access to files in another user's comments,
    # since they're all being held on the assignment for now.
    attachments ||= []
    old_ids = Array(attachment_ids || "").join(",").split(",").map(&:to_i)
    self.attachment_ids = attachments.select { |a| (a && a.id && old_ids.include?(a.id)) || (a.recently_created? && a.context == assignment) || a.context != assignment }.map(&:id).join(",")
  end

  # someday code-archaeologists will wonder how this method came to be named
  # validate_single_submission.  their guess is as good as mine

### queue_conditional_release_grade_change_handler

grade or graded status changed
    grade_changed = saved_changes.keys.intersect?(%w[grade score excused]) || (saved_change_to_workflow_state? && workflow_state == "graded")
    # any auditable conditions
    perform_audit = force_audit || grade_changed || assignment_changed_not_sub || saved_change_to_posted_at?

    if perform_audit
      if grade_change_event_author_id.present?
        self.grader_id = grade_change_event_author_id
      end
      self.class.connection.after_transaction_commit do
        Auditors::GradeChange.record(submission: self, skip_insert: !grade_changed)
        maybe_queue_conditional_release_grade_change_handler if grade_changed || (force_audit && posted_at.present?)
      end
    end
  end

### anonymous_identities

The student's annotations are what make up the submission in this case.
      allow_list.push(user)
    end

    if posted?
      allow_list.push(grader, user, current_user)
    elsif user == current_user
      # Requesting user is the student.
      allow_list << current_user
    elsif assignment.permits_moderation?(current_user)
      # Requesting user is the final grader or an administrator.
      allow_list.push(*assignment.moderation_grader_users_with_slot_taken, user, current_user)
    elsif assignment.can_be_moderated_grader?(current_user)
      # Requesting user is a provisional grader, or eligible to be one.
      if assignment.grader_comments_visible_to_graders
        allow_list.push(*assignment.moderation_grader_users_with_slot_taken, user, current_user)
      else
        allow_list.push(current_user, user)
      end
    end
    allow_list.compact.uniq
  end

### feedback_for_current_attempt

When grades are published for a moderated assignment, provisional
                             # comments made by the chosen grader are duplicated as non-provisional
                             # comments. Ignore the provisional copies of that grader's comments.
                             if association(:all_submission_comments).loaded?
                               all_submission_comments.reject { |comment| comment.provisional_grade_id.present? && comment.author_id == grader_id }
                             else
                               all_submission_comments.where.not("author_id = ? AND provisional_grade_id IS NOT NULL", grader_id)
                             end
                           else
                             all_submission_comments
                           end

    displayable_comments.select do |submission_comment|
      submission_comment.grants_right?(current_user, :read)
    end
  end

  # true if there is a comment by user other than submitter on the current attempt
  # comments prior to first attempt will count as current until a second attempt is started

### past_due

in a module so they can be included in other Submission-like objects. the
  # contract is that the including class must have the following attributes:
  #
  #  * assignment (Assignment)
  #  * submission_type (String)
  #  * workflow_state (String)
  #  * cached_due_date (Time)
  #  * submitted_at (Time)
  #  * score (Integer)
  #  * excused (Boolean)
  #  * late_policy_status (String)
  #  * seconds_late_override (Integer)
  #
  module Tardiness

### serialization_methods

include the versioned_attachments in as_json if this was loaded from a
  # specific version

### without_versioned_attachments

mechanism to turn off the above behavior for the duration of a
  # block

### attach_screenshot

This should always be called in the context of a delayed job
    return unless CutyCapt.enabled?

    if (attachment = CutyCapt.snapshot_attachment_for_url(url, context: self))
      attach_screenshot(attachment)
    else
      logger.error("Error capturing web snapshot for submission #{global_id}")
    end
  end

### comments_excluding_drafts_for

Note that this will return an Array (not an ActiveRecord::Relation) if comments are preloaded

### update_line_item_result

TODO: can we do this in bulk?
    return if assignment.deleted?

    return unless user_id

    return unless saved_change_to_score? || saved_change_to_grade? || saved_change_to_excused?

    return unless context.grants_right?(user, :participate_as_student)

    mark_item_unread("grade")
  end

### eligible_for_showing_score_statistics

Only indicate that the grade is hidden if there's an actual grade.
      # Similarly, hide the grade if the student resubmits (which will keep
      # the old grade but bump the workflow back to "submitted").
      (graded? || resubmitted?) && !posted?
    end
  end

  # You must also check the assignment.can_view_score_statistics

### posted

This checks whether this submission meets the requirements in order
    # for the submitter to be able to see score statistics for the assignment
    score.present? && !hide_grade_from_student?
  end

### rubric_assessments_for_attempt

If this submission is unposted and the viewer can't view the grade,
      # show only that viewer's assessments
      return rubric_assessments_for_attempt(attempt:).select do |assessment|
        assessment.assessor_id == viewing_user.id
      end
    end

    filtered_assessments = rubric_assessments_for_attempt(attempt:).select do |a|
      a.grants_right?(viewing_user, :read) &&
        a.rubric_association == assignment.rubric_association
    end

    if assignment.anonymous_peer_reviews? && !grants_right?(viewing_user, :grade)
      filtered_assessments.each do |a|
        if a.assessment_type == "peer_review" && viewing_user&.id != a.assessor&.id
          a.assessor = nil # hide peer reviewer's identity
        end
      end
    end

    filtered_assessments.sort_by do |a|
      [
        (a.assessment_type == "grading") ? CanvasSort::First : CanvasSort::Last,
        Canvas::ICU.collation_key(a.assessor_name)
      ]
    end
  end

### self

If the requested attempt is 0, no attempt has actually been submitted.
    # The submission's attempt will be nil (not 0), so we do actually want to
    # find assessments with a nil artifact_attempt.
    effective_attempt = (attempt == 0) ? nil : attempt

    rubric_assessments.each_with_object([]) do |assessment, assessments_for_attempt|
      # Always return self-assessments and assessments for the effective attempt
      if assessment.artifact_attempt == effective_attempt || assessment.assessment_type == "self_assessment"
        assessments_for_attempt << assessment
      else
        version = assessment.versions.find { |v| v.model.artifact_attempt == effective_attempt }
        assessments_for_attempt << version.model if version
      end
    end
  end
  private :rubric_assessments_for_attempt

### status_tag

if we don't bail here, the submissions will throw
          # errors deeper in the update because you can't change grades
          # on submissions that belong to deleted assignments
          unpublished_assignment_ids << assignment.id
          next
        end

        user_ids = user_grades.keys
        uids_for_visiblity = Api.map_ids(user_ids, User, context.root_account, grader)

        scope = assignment.students_with_visibility(context.students_visible_to(grader, include: :inactive),
                                                    uids_for_visiblity)
        if section
          scope = scope.where(enrollments: { course_section_id: section })
        end

        preloaded_users = scope.where(id: user_ids)
        preloaded_submissions = assignment.submissions.where(user_id: user_ids).group_by(&:user_id)

        Delayed::Batch.serial_batch(priority: Delayed::LOW_PRIORITY, n_strand: ["bulk_update_submissions", context.root_account.global_id]) do
          user_grades.each do |user_id, user_data|
            user = preloaded_users.detect { |u| u.global_id == Shard.global_id_for(user_id) }
            user ||= Api.sis_relation_for_collection(scope, [user_id], context.root_account).first
            unless user
              missing_ids << user_id
              next
            end

            submission = preloaded_submissions[user_id.to_i].first if preloaded_submissions[user_id.to_i]
            if !submission || user_data.key?(:posted_grade) || user_data.key?(:excuse)
              submissions =
                assignment.grade_student(user,
                                         grader:,
                                         grade: user_data[:posted_grade],
                                         excuse: Canvas::Plugin.value_to_boolean(user_data[:excuse]),
                                         skip_grade_calc: true,
                                         return_if_score_unchanged: true)
              submissions.each { |s| graded_user_ids << s.user_id unless s.score_unchanged }
              submission = submissions.first
            end
            submission.user = user

            assessment = user_data[:rubric_assessment]
            if assessment.is_a?(Hash) && assignment.active_rubric_association?
              # prepend each key with "criterion_", which is required by
              # the current RubricAssociation#assess code.
              assessment.transform_keys! do |crit_name|
                "criterion_#{crit_name}"
              end
              assignment.rubric_association.assess(
                assessor: grader,
                user:,
                artifact: submission,
                assessment: assessment.merge(assessment_type: "grading")
              )
            end

            comment = user_data.slice(:text_comment, :file_ids, :media_comment_id, :media_comment_type, :group_comment)
            next unless comment.present?

            comment = {
              comment: comment[:text_comment],
              author: grader,
              hidden: assignment.post_manually? && !submission.posted?
            }.merge(
              comment
            ).with_indifferent_access

            if (file_ids = user_data[:file_ids])
              attachments = Attachment.where(id: file_ids).to_a.select do |a|
                a.grants_right?(grader, :attach_to_submission_comment)
              end
              attachments.each { |a| a.ok_for_submission_comment = true }
              comment[:attachments] = attachments if attachments.any?
            end
            assignment.update_submission(user, comment)
          end
        end
      end
    end

    # make sure we don't pretend everything was fine if there were missing or
    # bad-state records that we couldn't handle.  We don't need to throw an exception,
    # but we do need to make the reason for lack of command compliance
    # visible.
    if missing_ids.any?
      progress.message = "Couldn't find User(s) with API ids #{missing_ids.map { |id| "'#{id}'" }.join(", ")}"
      progress.save
      progress.fail
    elsif unpublished_assignment_ids.any?
      progress.message = "Some assignments are either not published or deleted and can not be graded #{unpublished_assignment_ids.map { |id| "'#{id}'" }.join(", ")}"
      progress.save
      progress.fail
    end
  ensure
    context.clear_todo_list_cache_later(:admins)
    user_ids = graded_user_ids.to_a
    if user_ids.any?
      context.recompute_student_scores(user_ids)
    end
  end

### word_count

This logic is also implemented in SQL in
    # app/graphql/loaders/has_postable_comments_loader.rb
    # to determine if a submission has any postable comments.
    # Any changes made here should also be reflected in the loader.
    submission_comments.any?(&:allows_posting_submission?)
  end

### aggregate_checkpoint_submissions

TODO: see if we should be throwing an error here instead of defaulting to `submission`
    sub_assignment.all_submissions.find_by(user:) || self
  end

### calc_body_word_count

For large body text, this can be SLOW. Call this method in a delayed job.

### lti_id

Old records may not have an lti_id, so we need to set one
    self.lti_id ||= SecureRandom.uuid
  end

  # For internal use only.
  # The lti_id field on its own is not enough to uniquely identify a submission; use lti_attempt_id instead.

### create_audit_event

Adding a comment calls update_provisional_grade, but will not have the
    # grade or score keys included.
    if (attrs.key?(:grade) || attrs.key?(:score)) && pg.selection.present? && pg.scorer_id != assignment.final_grader_id
      raise Assignment::GradeError.new(error_code: Assignment::GradeError::PROVISIONAL_GRADE_MODIFY_SELECTED)
    end

    pg.scorer = pg.current_user = scorer
    pg.final = !!attrs[:final]
    if attrs.key?(:score)
      pg.score = attrs[:score]
      pg.grade = attrs[:grade].presence
    elsif attrs.key?(:grade)
      pg.grade = attrs[:grade]
    end
    pg.source_provisional_grade = attrs[:source_provisional_grade] if attrs.key?(:source_provisional_grade)
    pg.graded_anonymously = attrs[:graded_anonymously] unless attrs[:graded_anonymously].nil?
    pg.force_save = !!attrs[:force_save]
    pg
  end

### send_timing_data_if_needed

Outdated
    # If this submission is part of an assignment associated with a quiz, the
    # quiz object might be in a modified/readonly state (due to trying to load
    # a copy with override dates for this particular student) depending on what
    # path we took to get here. To avoid a ReadOnlyRecord error, do the actual
    # posting/hiding on a separate copy of the assignment, then reload our copy
    # of the assignment to make sure we pick up any changes to the muted status.
    if posted? && !previously_posted
      AbstractAssignment.find(assignment_id).post_submissions(submission_ids: [id], skip_updating_timestamp: true, skip_muted_changed: true)
      # This rescue is because of an error in the production environment where
      # the when a student that is also an admin creates a submission of an assignment
      # it throws a undefined method `owner' for nil:NilClass error when trying to
      # reload the assignment. This is fix to prevent the error from
      # crashing the server.
      begin
        assignment.reload
      rescue
        nil
      end
    elsif !posted? && previously_posted
      AbstractAssignment.find(assignment_id).hide_submissions(submission_ids: [id], skip_updating_timestamp: true, skip_muted_changed: true)
      begin
        assignment.reload
      rescue
        nil
      end
    end
  end

