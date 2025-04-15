# Enrollment

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

- belongs_to :course
- belongs_to :course_section
- belongs_to :root_account
- belongs_to :user
- belongs_to :sis_pseudonym
- belongs_to :associated_user
- belongs_to :temporary_enrollment_pairing
- belongs_to :role
- has_one :enrollment_state
- has_many :role_overrides
- has_many :pseudonyms
- has_many :course_account_associations
- has_many :scores
- has_one :through

## Methods

### ensure_role_id

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

class Enrollment < ActiveRecord::Base
  SIS_TYPES = {
    "TeacherEnrollment" => "teacher",
    "TaEnrollment" => "ta",
    "DesignerEnrollment" => "designer",
    "StudentEnrollment" => "student",
    "ObserverEnrollment" => "observer"
  }.freeze

  self.ignored_columns += ["graded_at"]

  include Workflow

  belongs_to :course, inverse_of: :enrollments
  belongs_to :course_section, inverse_of: :enrollments
  belongs_to :root_account, class_name: "Account", inverse_of: :enrollments
  belongs_to :user, inverse_of: :enrollments
  belongs_to :sis_pseudonym, class_name: "Pseudonym", inverse_of: :sis_enrollments
  belongs_to :associated_user, class_name: "User"
  belongs_to :temporary_enrollment_pairing, inverse_of: :enrollments

  belongs_to :role
  include Role::AssociationHelper

  has_one :enrollment_state, dependent: :destroy, inverse_of: :enrollment

  has_many :role_overrides, as: :context, inverse_of: :context
  has_many :pseudonyms, primary_key: :user_id, foreign_key: :user_id, inverse_of: false
  has_many :course_account_associations, foreign_key: "course_id", primary_key: "course_id", inverse_of: false
  has_many :scores, -> { active }

  validates :user_id, :course_id, :type, :root_account_id, :course_section_id, :workflow_state, :role_id, presence: true
  validates :limit_privileges_to_course_section, inclusion: { in: [true, false] }
  validates :associated_user_id, inclusion: { in: [nil],
                                              unless: ->(enrollment) { enrollment.type == "ObserverEnrollment" },
                                              message: ->(_object, _data) { t("only ObserverEnrollments may have an associated_user_id") } }
  validate :cant_observe_self, if: ->(enrollment) { enrollment.type == "ObserverEnrollment" }
  validate :cant_observe_observer, if: ->(enrollment) { enrollment.type == "ObserverEnrollment" }

  validate :valid_role?
  validate :valid_course?
  validate :not_template_course?
  validate :valid_section?
  validate :not_student_view

  # update bulk destroy if changing or adding an after save
  before_save :assign_uuid
  before_validation :assert_section
  after_save :recalculate_enrollment_state
  after_save :update_user_account_associations_if_necessary
  before_save :audit_groups_for_deleted_enrollments
  before_validation :ensure_role_id
  after_create :create_linked_enrollments
  after_create :create_enrollment_state
  after_save :copy_scores_from_existing_enrollment, if: :need_to_copy_scores?
  after_save :clear_email_caches
  after_save :cancel_future_appointments
  after_save :update_linked_enrollments
  after_save :set_update_cached_due_dates
  after_save :touch_graders_if_needed
  after_save :reset_notifications_cache
  after_save :dispatch_invitations_later
  after_save :add_to_favorites_later
  after_commit :update_cached_due_dates
  after_save :update_assignment_overrides_if_needed
  after_create :needs_grading_count_updated, if: :active_student?
  after_update :needs_grading_count_updated, if: :active_student_changed?

  after_commit :sync_microsoft_group
  scope :microsoft_sync_relevant, -> { active_or_pending.accepted.not_fake }
  scope :microsoft_sync_irrelevant_but_not_fake, -> { not_fake.where("enrollments.workflow_state IN ('rejected', 'completed', 'inactive', 'invited')") }

  attr_accessor :already_enrolled, :need_touch_user, :skip_touch_user

  scope :current, -> { joins(:course).where(QueryBuilder.new(:active).conditions).readonly(false) }
  scope :current_and_invited, -> { joins(:course).where(QueryBuilder.new(:current_and_invited).conditions).readonly(false) }
  scope :current_and_future, -> { joins(:course).where(QueryBuilder.new(:current_and_future).conditions).readonly(false) }
  scope :concluded, -> { joins(:course).where(QueryBuilder.new(:completed).conditions).readonly(false) }
  scope :current_and_concluded, -> { joins(:course).where(QueryBuilder.new(:current_and_concluded).conditions).readonly(false) }
  scope :horizon, -> { joins(:course).where(courses: { horizon_course: true }) }
  scope :not_horizon, -> { joins(:course).where(courses: { horizon_course: false }) }

### self

see #active_student?

### active_student

see .active_student_conditions

### self

if in an invited state but not frd "invited?" because of future date restrictions, send it later
    if (just_created || saved_change_to_workflow_state? || @re_send_confirmation) && workflow_state == "invited" && inactive? && available_at &&
       !self_enrolled && !(observer? && user.registered?)
      # this won't work if they invite them and then change the course/term/section dates _afterwards_ so hopefully people don't do that
      delay(run_at: available_at, singleton: "send_enrollment_invitations_#{global_id}").re_send_confirmation_if_invited!
    end
  end

  scope :active, -> { where("enrollments.workflow_state<>'deleted'") }
  scope :deleted, -> { where(workflow_state: "deleted") }

  scope :admin, lambda {
                  select(:course_id)
                    .joins(:course)
                    .where("enrollments.type IN ('TeacherEnrollment','TaEnrollment', 'DesignerEnrollment') AND (courses.workflow_state IN ('created', 'claimed') OR (enrollments.workflow_state='active' AND courses.workflow_state='available'))")
                }

  scope :instructor, lambda {
                       select(:course_id)
                         .joins(:course)
                         .where("enrollments.type IN ('TeacherEnrollment','TaEnrollment') AND (courses.workflow_state IN ('created', 'claimed') OR (enrollments.workflow_state='active' AND courses.workflow_state='available'))")
                     }

  scope :of_student_type, -> { where(type: "StudentEnrollment") }

  scope :of_admin_type, -> { where(type: %w[TeacherEnrollment TaEnrollment DesignerEnrollment]) }

  scope :of_instructor_type, -> { where(type: ["TeacherEnrollment", "TaEnrollment"]) }

  scope :of_content_admins, -> { where(type: ["TeacherEnrollment", "DesignerEnrollment"]) }

  scope :of_observer_type, -> { where(type: "ObserverEnrollment") }

  scope :not_of_observer_type, -> { where.not(type: "ObserverEnrollment") }

  scope :student, lambda {
                    select(:course_id)
                      .joins(:course)
                      .where(type: "StudentEnrollment", workflow_state: "active", courses: { workflow_state: "available" })
                  }

  scope :student_in_claimed_or_available, lambda {
                                            select(:course_id)
                                              .joins(:course)
                                              .where(type: "StudentEnrollment", workflow_state: "active", courses: { workflow_state: %w[available claimed created] })
                                          }

  scope :all_student, lambda {
                        eager_load(:course)
                          .where("
                            (
                              enrollments.type = 'StudentEnrollment'
                              AND enrollments.workflow_state IN ('invited', 'active', 'completed')
                            )
                              OR
                            (
                              enrollments.type = 'StudentViewEnrollment'
                              AND enrollments.workflow_state = 'active'
                            )
                          ")
                          .merge(Course.active)
                      }

  scope :not_deleted, lambda {
    joins(:course)
      .where("(courses.workflow_state<>'deleted') AND (enrollments.workflow_state<>'deleted')")
  }

  scope :not_fake, -> { where("enrollments.type<>'StudentViewEnrollment'") }

  scope :temporary_enrollment_recipients_for_provider, lambda { |user|
    active.joins(:course).where(temporary_enrollment_source_user_id: user,
                                courses: { workflow_state: %w[available claimed created] })
  }

  scope :temporary_enrollments_for_recipient, lambda { |user|
    active.joins(:course).where(user_id: user, courses: { workflow_state: %w[available claimed created] })
          .where.not(temporary_enrollment_source_user_id: nil)
  }

### self

with enough use, even translations can add up
    RequestCache.cache("enrollment_readable_types") do
      {
        "TeacherEnrollment" => t("#enrollment.roles.teacher", "Teacher"),
        "TaEnrollment" => t("#enrollment.roles.ta", "TA"),
        "DesignerEnrollment" => t("#enrollment.roles.designer", "Designer"),
        "StudentEnrollment" => t("#enrollment.roles.student", "Student"),
        "StudentViewEnrollment" => t("#enrollment.roles.student", "Student"),
        "ObserverEnrollment" => t("#enrollment.roles.observer", "Observer")
      }
    end
  end

### audit_groups_for_deleted_enrollments

If other active sessions that the user is enrolled in exist.
    course.student_enrollments.where.not(workflow_state: ["deleted", "rejected"]).for_user(user).where.not(id:).exists?
  end

### observers

did the student cease to be enrolled in a non-deleted state in a section?
    had_section = course_section_id_was.present?
    deleted_states = ["deleted", "rejected"]
    was_active = !deleted_states.include?(workflow_state_was)
    is_deleted = deleted_states.include?(workflow_state)
    return unless had_section && was_active &&
                  (course_section_id_changed? || is_deleted)

    # what section the user is abandoning, and the section they're moving to
    # (if it's in the same course and the enrollment's not deleted)
    section = CourseSection.find(course_section_id_was)

    # ok, consider groups the user is in from the abandoned section's course
    user.groups.preload(:group_category).where(
      context_type: "Course", context_id: section.course_id
    ).each do |group|
      # check group deletion criteria if either enrollment is not a deletion
      # or it may be a deletion/unenrollment from a section but not from the course as a whole (still enrolled in another section)
      if !is_deleted || other_section_enrollment_exists?
        # don't bother unless the group's category has section restrictions
        next unless group.group_category&.restricted_self_signup?

        # skip if the user is the only user in the group. there's no one to have
        # a conflicting section.
        next unless group.users.where.not(id: user_id).exists?

        # check if the group has the section the user is abandoning as a common
        # section (from CourseSection#common_to_users? view, the enrollment is
        # still there since it queries the db directly and we haven't saved yet);
        # if not, dropping the section is not necessary
        next unless section.common_to_users?(group.users)
      end

      # at this point, the group is restricted, there's more than one user and
      # it appears that the group is common to the section being left by the user so
      # remove the user from the group. Or the student was only enrolled in one section and
      # by leaving the section he/she is completely leaving the course so remove the
      # user from any group related to the course.
      membership = group.group_memberships.where(user_id:).first
      membership&.destroy
    end

    user.differentiation_tags.preload(:group_category).where(
      context_type: "Course", context_id: section.course_id
    ).find_each do |tag|
      # Only remove differentiation tag memberships if the enrollment is being deleted/rejected
      next unless is_deleted

      membership = tag.group_memberships.where(user_id:).first
      membership&.destroy
    end
  end
  protected :audit_groups_for_deleted_enrollments

### linked_enrollment_for

we don't want to create a new observer enrollment if one exists
    self.class.unique_constraint_retry do
      enrollment = linked_enrollment_for(observer)
      return true if enrollment && !enrollment.deleted? && !enrollment.inactive?
      return false unless observer.can_be_enrolled_in_course?(course)

      enrollment ||= observer.observer_enrollments.build
      enrollment.associated_user_id = user_id
      enrollment.shard = shard if enrollment.new_record?
      enrollment.update_from(self, !!@skip_broadcasts)
    end
  end

### set_update_cached_due_dates

we don't want to "undelete" observer enrollments that have been
    # explicitly deleted
    return nil if enrollment&.deleted? && workflow_state_before_last_save != "deleted"

    enrollment
  end

  # This is Part 1 of the update_cached_due_dates callback.  It sets @update_cached_due_dates which determines
  # whether or not the update_cached_due_dates after_commit callback runs after this record has been committed.
  # This split allows us to suspend this callback and affect the update_cached_due_dates callback since after_commit
  # callbacks aren't being suspended properly.  We suspend this callback during some bulk operations.

### rank_sortable

don't call rank_sql during class load
    rank_sql(TYPE_RANKS[order], "enrollments.type")
  end

### state_sortable

don't call rank_sql during class load
    @state_rank_sql ||= rank_sql(STATE_RANK, "enrollments.workflow_state")
  end

### enrollment_dates

this method was written by Alan Smithee
    user.shard.activate do
      if user.favorites.where(context_type: "Course").exists? # only add a favorite if they've ever favorited anything even if it's no longer in effect
        Favorite.create_or_find_by(user:, context: course)
      end
    end
  end

  workflow do
    state :invited do
      event :reject, transitions_to: :rejected
      event :complete, transitions_to: :completed
    end

    state :creation_pending do
      event :invite, transitions_to: :invited
    end

    state :active do
      event :reject, transitions_to: :rejected
      event :complete, transitions_to: :completed
    end

    state :deleted
    state :rejected do
      event :unreject, transitions_to: :invited
    end
    state :completed

    # Inactive is a "hard" state, i.e. tuition not paid
    state :inactive
  end

### create_enrollment_state

ensure we have an enrollment state object present with a reverse association
    result ||= create_enrollment_state
    result.enrollment = self
    result
  end

### available_at

when view restrictions are in place, the effective state_based_on_date is :inactive, but
    # to admins we should show that they are :completed or :pending
    enrollment_state.get_display_state
  end

### can_be_concluded_by

Determine if a user has permissions to conclude this enrollment.
  #
  # user    - The user requesting permission to conclude/delete enrollment.
  # context - The current context, e.g. course or section.
  # session - The current user's session (pass nil if not available).
  #
  # return Boolean

### can_be_deleted_by

Determine if a user has permissions to delete this enrollment.
  #
  # user    - The user requesting permission to conclude/delete enrollment.
  # context - The current context, e.g. course or section.
  # session - The current user's session (pass nil if not available).
  #
  # return Boolean

### self

This is called to recompute the users' cached scores for a given course
  # when:
  #
  # * The user is merged with another user; the scores are recomputed for the
  #   new user in each of his/her courses.
  #
  # * An assignment's default grade is changed; all users in the assignment's
  #   course have their scores for that course recomputed.
  #
  # * A course is merged into another, a section is crosslisted/uncrosslisted,
  #   or a section is otherwise moved between courses; scores are recomputed
  #   for all users in the target course.
  #
  # * A course's group_weighting_scheme is changed; scores are recomputed for
  #   all users in the course.
  #
  # * Assignments are reordered (since an assignment may change groups, which
  #   may have weights); scores are recomputed for all users in the associated
  #   course.
  #
  # * An assignment's points_possible is changed; scores are recomputed for all
  #   users in the associated course.
  #
  # * An assignment group's rules or group_weight are changed; scores are
  #   recomputed for all users in the associated course.
  #
  # * A submission's score is changed; scores for the submission owner in the
  #   associated course are recomputed.
  #
  # * An assignment is deleted/undeleted
  #
  # * An enrollment is accepted (to address the scenario where a student
  #   is transferred from one section to another, and final grades need
  #   to be transferred)
  #
  # If some new feature comes up that affects calculation of a user's score,
  # please add appropriate calls to this so that the cached values don't get
  # stale! And once you've added the call, add the condition to the comment
  # here for future enlightenment.

### self

This method is intended to not duplicate work for a single user.

### self

Guard against getting more than one user_id
    raise ArgumentError, "Cannot call with more than one user" if Array(user_id).size > 1

    delay_if_production(singleton: "Enrollment.recompute_final_score:#{user_id}:#{course_id}:#{opts[:grading_period_id]}",
                        max_attempts: 10)
      .recompute_final_score(user_id, course_id, **opts)
  end

### graded_at

have to go through gymnastics to force-preload a has_one :through without causing a db transaction
      if association(:course).loaded?
        assn = result.association(:course)
        assn.target = course
      end
    end
    result
  end

### student

overridden to return true in appropriate subclasses

### self

read_services says this person has permission to see what web services this enrollment has linked to their account
    given { |user, session| grants_right?(user, session, :read) && self.user.show_user_services }
    can :read_services
  end

  scope :before, lambda { |date|
    where("enrollments.created_at<?", date)
  }

  scope :for_user, ->(user) { where(user_id: user) }

  scope :for_courses_with_user_name, lambda { |courses|
    where(course_id: courses)
      .joins(:user)
      .select("user_id, course_id, users.name AS user_name")
  }
  scope :invited, -> { where(workflow_state: "invited") }
  scope :accepted, -> { where("enrollments.workflow_state<>'invited'") }
  scope :active_or_pending, -> { where("enrollments.workflow_state NOT IN ('rejected', 'completed', 'deleted', 'inactive')") }
  scope :all_active_or_pending, -> { where("enrollments.workflow_state NOT IN ('rejected', 'completed', 'deleted')") } # includes inactive

  scope :excluding_pending, -> { joins(:enrollment_state).where.not(enrollment_states: { state: EnrollmentState::PENDING_STATES }) }
  scope :active_by_date, -> { joins(:enrollment_state).where("enrollment_states.state = 'active'") }
  scope :invited_by_date, lambda {
                            joins(:enrollment_state).where(enrollment_states: { restricted_access: false })
                                                    .where("enrollment_states.state IN ('invited', 'pending_invited')")
                          }
  scope :active_or_pending_by_date, lambda {
                                      joins(:enrollment_state).where(enrollment_states: { restricted_access: false })
                                                              .where("enrollment_states.state IN ('active', 'invited', 'pending_invited', 'pending_active')")
                                    }
  scope :invited_or_pending_by_date, lambda {
                                       joins(:enrollment_state).where(enrollment_states: { restricted_access: false })
                                                               .where("enrollment_states.state IN ('invited', 'pending_invited', 'pending_active')")
                                     }
  scope :completed_by_date,
        -> { joins(:enrollment_state).where(enrollment_states: { restricted_access: false, state: "completed" }) }
  scope :not_inactive_by_date, lambda {
                                 joins(:enrollment_state).where(enrollment_states: { restricted_access: false })
                                                         .where("enrollment_states.state IN ('active', 'invited', 'completed', 'pending_invited', 'pending_active')")
                               }

  scope :active_or_pending_by_date_ignoring_access, lambda {
                                                      joins(:enrollment_state)
                                                        .where("enrollment_states.state IN ('active', 'invited', 'pending_invited', 'pending_active')")
                                                    }
  scope :not_inactive_by_date_ignoring_access, lambda {
                                                 joins(:enrollment_state)
                                                   .where("enrollment_states.state IN ('active', 'invited', 'completed', 'pending_invited', 'pending_active')")
                                               }
  scope :new_or_active_by_date, lambda {
                                  joins(:enrollment_state)
                                    .where("enrollment_states.state IN ('active', 'invited', 'pending_invited', 'pending_active', 'creation_pending')")
                                }

  scope :currently_online, -> { joins(:pseudonyms).where("pseudonyms.last_request_at>?", 5.minutes.ago) }
  # this returns enrollments for creation_pending users; should always be used in conjunction with the invited scope
  scope :for_email, lambda { |email|
    joins(user: :communication_channels)
      .where("users.workflow_state='creation_pending' AND communication_channels.workflow_state='unconfirmed' AND path_type='email' AND LOWER(path)=LOWER(?)", email)
      .select("enrollments.*")
      .readonly(false)
  }

### uuid

DON'T use ||=, because that will cause an immediate save to the db if it
    # doesn't already exist
    self.uuid = CanvasSlug.generate_securish_uuid unless self["uuid"]
  end
  protected :assign_uuid

### effective_start_at

enrollment term per-section is deprecated; a section's term is inherited from the
  # course it is currently tied to
  delegate :enrollment_term, to: :course

### effective_end_at

try and use the enrollment dates logic first, since it knows about
    # overrides, etc. but if it doesn't find anything, start guessing by
    # looking at the enrollment, section, course, then term. if we still didn't
    # find it, fall back to the section or course creation date.
    enrollment_dates.filter_map(&:first).min ||
      start_at ||
      course_section&.start_at ||
      course.start_at ||
      course.enrollment_term&.start_at ||
      course_section&.created_at ||
      course.created_at
  end

### self

try and use the enrollment dates logic first, since it knows about
    # overrides, etc. but if it doesn't find anything, start guessing by
    # looking at the enrollment, section, course, then term.
    enrollment_dates.filter_map(&:last).max ||
      end_at ||
      course_section&.end_at ||
      course.conclude_at ||
      course.enrollment_term&.end_at
  end

### section_or_course_date_in_past

this is handled in after_commit :update_cached_due_dates
      AssignmentOverrideStudent.suspend_callbacks(:update_cached_due_dates) do
        override_scope.where(assignment_id: assignment_ids).find_each(&:destroy)
      end
    end

    if being_accepted?
      return unless ConditionalRelease::Service.enabled_in_context?(course)

      # Deleted student overrides associated with assignments with a Mastery Path override
      releases = override_scope.where(workflow_state: "deleted")
                               .where(assignment: assignment_scope)
                               .joins(assignment: :assignment_overrides)
                               .where(assignment_overrides: {
                                        set_type: AssignmentOverride::SET_TYPE_NOOP,
                                        set_id: AssignmentOverride::NOOP_MASTERY_PATHS,
                                        workflow_state: "active"
                                      }).distinct
      return unless releases.exists?

      # Add parent join to reduce duplication, which are used in both cases below
      releases = releases
                 .joins("INNER JOIN #{AssignmentOverride.quoted_table_name} parent ON assignment_override_students.assignment_override_id = parent.id")
      # Restore student overrides associated with an active assignment override
      releases.where("parent.workflow_state = 'active'").update(workflow_state: "active")
      # Restore student overrides and assignment overrides if assignment override is deleted
      releases.preload(:assignment_override).where("parent.workflow_state = 'deleted'").find_each do |release|
        release.update(workflow_state: "active")
        release.assignment_override.update(workflow_state: "active")
      end
    end
  end

