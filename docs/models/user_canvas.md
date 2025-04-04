# User

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

- has_many :communication_channels
- has_many :notification_policies
- has_many :notification_policy_overrides
- has_one :communication_channel
- has_many :ignores
- has_many :planner_notes
- has_many :viewed_submission_comments
- has_many :enrollments
- has_many :course_paces
- has_many :course_reports
- has_many :not_ended_enrollments
- has_many :not_removed_enrollments
- has_many :observer_enrollments
- has_many :observee_enrollments
- has_many :observer_pairing_codes
- has_many :as_student_observation_links
- has_many :as_observer_observation_links
- has_many :as_student_observer_alert_thresholds
- has_many :as_student_observer_alerts
- has_many :as_observer_observer_alert_thresholds
- has_many :as_observer_observer_alerts
- has_many :linked_observers
- has_many :linked_students
- has_many :all_courses
- has_many :all_courses_for_active_enrollments
- has_many :polls
- has_many :group_memberships
- has_many :current_group_memberships
- has_many :groups
- has_many :current_groups
- has_many :differentiation_tag_memberships
- has_many :current_differentiation_tag_memberships
- has_many :differentiation_tags
- has_many :current_differentiation_tags
- has_many :user_account_associations
- has_many :unordered_associated_accounts
- has_many :associated_accounts
- has_many :associated_root_accounts
- has_many :developer_keys
- has_many :access_tokens
- has_many :masquerade_tokens
- has_many :notification_endpoints
- has_many :context_external_tools
- has_many :lti_results
- has_many :student_enrollments
- has_many :ta_enrollments
- has_many :teacher_enrollments
- has_many :all_submissions
- has_many :submissions
- has_many :pseudonyms
- has_many :active_pseudonyms
- has_many :pseudonym_accounts
- has_one :pseudonym
- has_many :attachments
- has_many :active_images
- has_many :active_assignments
- has_many :mentions
- has_many :discussion_entries
- has_many :discussion_entry_drafts
- has_many :discussion_entry_versions
- has_many :all_attachments
- has_many :folders
- has_many :submissions_folders
- has_many :active_folders
- has_many :calendar_events
- has_many :eportfolios
- has_many :quiz_submissions
- has_many :dashboard_messages
- has_many :user_services
- has_many :rubric_associations
- has_many :rubrics
- has_many :context_rubrics
- has_many :grading_standards
- has_many :context_module_progressions
- has_many :assessment_question_bank_users
- has_many :assessment_question_banks
- has_many :learning_outcome_results
- has_many :collaborators
- has_many :collaborations
- has_many :assigned_submission_assessments
- has_many :assigned_assessments
- has_many :web_conference_participants
- has_many :web_conferences
- has_many :account_users
- has_many :media_objects
- has_many :user_generated_media_objects
- has_many :content_shares
- has_many :received_content_shares
- has_many :sent_content_shares
- has_many :account_reports
- has_many :stream_item_instances
- has_many :all_conversations
- has_many :conversation_batches
- has_many :favorites
- has_many :messages
- has_many :sis_batches
- has_many :sis_post_grades_statuses
- has_many :content_migrations
- has_many :content_exports
- has_many :usage_rights
- has_many :gradebook_csvs
- has_many :block_editor_templates
- has_many :asset_user_accesses
- has_one :profile
- has_many :progresses
- has_many :one_time_passwords
- has_many :past_lti_ids
- has_many :user_preference_values
- has_many :auditor_authentication_records
- has_many :auditor_course_records
- has_many :auditor_student_grade_change_records
- has_many :auditor_grader_grade_change_records
- has_many :auditor_feature_flag_records
- has_many :created_lti_registrations
- has_many :updated_lti_registrations
- has_many :created_lti_registration_account_bindings
- has_many :updated_lti_registration_account_bindings
- has_many :lti_overlays
- has_many :lti_overlay_versions
- has_many :lti_asset_processor_eula_acceptances
- has_many :comment_bank_items
- has_many :microsoft_sync_partial_sync_changes
- has_many :gradebook_filters
- has_many :quiz_migration_alerts
- has_many :custom_data
- belongs_to :otp_communication_channel
- belongs_to :merged_into_user

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

class User < ActiveRecord::Base
  self.ignored_columns += ["last_user_note", "page_views_count"]

  GRAVATAR_PATTERN = %r{^https?://[a-zA-Z0-9.-]+\.gravatar\.com/}
  MAX_ROOT_ACCOUNT_ID_SYNC_ATTEMPTS = 5
  MINIMAL_COLUMNS_TO_SAVE = %i[avatar_image_source
                               avatar_image_url
                               created_at
                               id
                               initial_enrollment_type
                               lti_id
                               name
                               preferences
                               reminder_time_for_due_dates
                               reminder_time_for_grading
                               short_name
                               sortable_name
                               uuid
                               workflow_state].freeze

  include ManyRootAccounts
  include TurnitinID
  include Pronouns

  # this has to be before include Context to prevent a circular dependency in Course

### self

this has to be before include Context to prevent a circular dependency in Course

### starred_conversations

i.e. exclude any where the user has deleted all the messages
    all_conversations.visible.order("last_message_at DESC, conversation_id DESC")
  end

### reload

NOTE: only use for courses with differentiated assignments on
  scope :able_to_see_assignment_in_course_with_da, lambda { |assignment_id, course_id, user_ids = nil|
    visible_user_id = AssignmentVisibility::AssignmentVisibilityService.assignments_visible_to_students(assignment_ids: assignment_id, course_ids: course_id, user_ids:).map(&:user_id)
    if visible_user_id.any?
      where(id: visible_user_id)
    else
      none
    end
  }

  # NOTE: only use for courses with differentiated assignments on
  scope :able_to_see_quiz_in_course_with_da, lambda { |quiz_id, course_id|
    visible_user_ids = QuizVisibility::QuizVisibilityService.quizzes_visible_to_students(quiz_ids: quiz_id, course_ids: course_id).map(&:user_id)
    where(id: visible_user_ids)
  }

  scope :observing_students_in_course, lambda { |observee_ids, course_ids|
    joins(:enrollments).where(enrollments: { type: "ObserverEnrollment", associated_user_id: observee_ids, course_id: course_ids, workflow_state: "active" })
  }

  # when an observer is added to a course they get an enrollment where associated_user_id is nil. when they are linked to
  # a student, this first enrollment stays the same, but a new one with an associated_user_id is added. thusly to find
  # course observers, you take the difference between all active observers and active observers with associated users
  scope :observing_full_course, lambda { |course_ids|
    active_observer_scope = joins(:enrollments).where(enrollments: { type: "ObserverEnrollment", course_id: course_ids, workflow_state: ["active", "invited"] })
    users_observing_students = active_observer_scope.where.not(enrollments: { associated_user_id: nil }).pluck(:id)

    if users_observing_students == [] || users_observing_students.nil?
      active_observer_scope
    else
      active_observer_scope.where.not(users: { id: users_observing_students })
    end
  }

  scope :linked_through_root_account, lambda { |root_account|
    where(UserObservationLink.table_name => { root_account_id: [root_account.id, nil] + root_account.trusted_account_ids })
  }

### courses_for_enrollments

add a field to each user that is the aggregated max from current_login_at and last_login_at from their pseudonyms
    select_clause = "MAX(current_login_at) as last_login"
    select_clause = "users.*, #{select_clause}" if select_values.blank?
    scope = select(select_clause).
            # left outer join ensures we get the user even if they don't have a pseudonym
            joins(sanitize_sql([<<~SQL.squish, root_account_id])).where(enrollments: { course_id: course })
              LEFT OUTER JOIN #{Pseudonym.quoted_table_name} ON pseudonyms.user_id = users.id AND pseudonyms.account_id = ?
              INNER JOIN #{Enrollment.quoted_table_name} ON enrollments.user_id = users.id
            SQL
    scope = scope.where("enrollments.workflow_state<>'deleted'")
    scope = scope.where(enrollments: { type: enrollment_type }) if enrollment_type
    # the trick to get unique users
    scope.group("users.id")
  }

  attr_accessor :require_acceptance_of_terms,
                :require_presence_of_name,
                :require_self_enrollment_code,
                :self_enrollment_code,
                :self_enrollment_course,
                :validation_root_account,
                :sortable_name_explicitly_set
  attr_reader :self_enrollment

  validates :name, length: { maximum: maximum_string_length, allow_nil: true }
  validates :short_name, length: { maximum: maximum_string_length, allow_nil: true }
  validates :sortable_name, length: { maximum: maximum_string_length, allow_nil: true }
  validates :name, presence: { if: :require_presence_of_name }
  validates_locale :locale, :browser_locale, allow_nil: true
  validates :terms_of_use, acceptance: { if: :require_acceptance_of_terms, allow_nil: false }
  validates :instructure_identity_id, uniqueness: true, allow_nil: true
  validates_each :self_enrollment_code do |record, attr, value|
    next unless record.require_self_enrollment_code

    if value.blank?
      record.errors.add(attr, "blank")
    elsif record.validation_root_account
      course = record.validation_root_account.self_enrollment_course_for(value)
      record.self_enrollment_course = course
      if course&.self_enrollment_enabled?
        record.errors.add(attr, "full") if course.self_enrollment_limit_met?
        record.errors.add(attr, "concluded") if course.concluded?("StudentEnrollment")
        record.errors.add(attr, "already_enrolled") if course.user_is_student?(record, include_future: true)
      else
        record.errors.add(attr, "invalid")
      end
    else
      record.errors.add(attr, "account_required")
    end
  end

  before_save :assign_uuid
  before_save :record_acceptance_of_terms
  after_save :update_account_associations_if_necessary
  after_save :self_enroll_if_necessary

### update_root_account_ids

Update the root_account_ids column on the user
  # and all the users CommunicationChannels

### update_root_account_ids_later

We don't need to worry about relative ids here because test students are never cross-shard
      refreshed_root_account_ids << enrollments.where(type: "StudentViewEnrollment").pick(:root_account_id)
    else
      # See User#associated_shards in MRA for an explanation of
      # shard association levels
      shards = associated_shards(:strong) + associated_shards(:weak)

      Shard.with_each_shard(shards) do
        root_account_ids = user_account_associations.for_root_accounts.shard(Shard.current).distinct.pluck(:account_id)
        root_account_ids.concat(if deleted? || creation_pending?
                                  # if the user is deleted, they'll have no user_account_associations, so we need to add
                                  # back in associations from both active and deleted objects
                                  pseudonyms.shard(Shard.current).except(:order).distinct.pluck(:account_id) +
                                  enrollments.shard(Shard.current).distinct.pluck(:root_account_id) +
                                  account_users.shard(Shard.current).distinct.pluck(:root_account_id)
                                else
                                  # need to add back in deleted associations
                                  pseudonyms.deleted.shard(Shard.current).except(:order).distinct.pluck(:account_id) +
                                  enrollments.deleted.shard(Shard.current).distinct.pluck(:root_account_id) +
                                  account_users.deleted.shard(Shard.current).distinct.pluck(:root_account_id)
                                end)
        root_account_ids.each do |account_id|
          refreshed_root_account_ids << Shard.relative_id_for(account_id, Shard.current, shard)
        end
      end
    end

    # Update the user
    self.root_account_ids = refreshed_root_account_ids.to_a.sort
    if root_account_ids_changed?
      save!
      # Update communication channel and feature flag records associated with the user
      communication_channels.update_all(root_account_ids:)
      feature_flags.update_all(root_account_ids:)
    end
  end

### enrollments_for_account_and_sub_accounts

incremental is only for the current shard
    return User.update_account_associations([self], opts) if opts[:incremental]

    shard.activate do
      User.update_account_associations([self], opts)
    end
  end

### self

enrollments are always on the course's shard
    # and courses are always on the root account's shard
    account.shard.activate do
      Enrollment.where(user_id: self).active.joins(:course).where("courses.account_id=? OR courses.root_account_id=?", account, account)
    end
  end

### registration_approval_required

Split it up into manageable chunks
    if users_or_user_ids.length > 500
      users_or_user_ids.uniq.compact.each_slice(500) do |users_or_user_ids_slice|
        update_account_associations(users_or_user_ids_slice, opts)
      end
      return
    end

    incremental = opts[:incremental]
    precalculated_associations = opts[:precalculated_associations]

    user_ids = users_or_user_ids
    user_ids = user_ids.map(&:id) if user_ids.first.is_a?(User)
    shards = [Shard.current]
    unless precalculated_associations
      users = if users_or_user_ids.first.is_a?(User)
                users_or_user_ids
              else
                users_or_user_ids = User.select(%i[id preferences workflow_state updated_at]).where(id: user_ids).to_a
              end

      if opts[:all_shards]
        shards = Set.new
        users.each { |u| shards += u.associated_shards }
        shards = shards.to_a
      end

      # Users are tied to accounts a couple ways:
      #   Through enrollments:
      #      User -> Enrollment -> Section -> Course -> Account
      #      User -> Enrollment -> Section -> Non-Xlisted Course -> Account
      #   Through pseudonyms:
      #      User -> Pseudonym -> Account
      #   Through account_users
      #      User -> AccountUser -> Account
      account_mappings = Hash.new { |h, k| h[k] = Set.new }
      base_shard = Shard.current
      Shard.with_each_shard(shards) do
        courses_relation = Course.select("enrollments.user_id", :account_id).distinct
                                 .joins(course_sections: :enrollments)
                                 .where("enrollments.user_id": users)
                                 .where.not("enrollments.workflow_state": [:deleted, :rejected])
                                 .where.not("enrollments.type": "StudentViewEnrollment")
        non_xlist_relation = Course.select("enrollments.user_id", :account_id).distinct
                                   .joins("INNER JOIN #{CourseSection.quoted_table_name} on course_sections.nonxlist_course_id=courses.id")
                                   .joins("INNER JOIN #{Enrollment.quoted_table_name} on enrollments.course_section_id=course_sections.id")
                                   .where("enrollments.user_id": users)
                                   .where.not("enrollments.workflow_state": [:deleted, :rejected])
                                   .where.not("enrollments.type": "StudentViewEnrollment")
        pseudonym_relation = Pseudonym.active.select(:user_id, :account_id).distinct.where(user: users)
        account_user_relation = AccountUser.active.select(:user_id, :account_id).distinct.where(user: users)

        results = connection.select_rows(<<~SQL.squish)
          #{courses_relation.to_sql} UNION
          #{non_xlist_relation.to_sql} UNION
          #{pseudonym_relation.to_sql} UNION
          #{account_user_relation.to_sql}
        SQL

        results.each do |row|
          account_mappings[Shard.relative_id_for(row.first, Shard.current, base_shard)] << Shard.relative_id_for(row.second, Shard.current, base_shard)
        end
      end
    end

    # TODO: transaction on each shard?
    UserAccountAssociation.transaction do
      current_associations = {}
      to_delete = []
      Shard.with_each_shard(shards) do
        # if shards is more than just the current shard, users will be set; otherwise
        # we never loaded users, but it doesn't matter, cause it's all the current shard
        shard_user_ids = users ? users.map(&:id) : user_ids
        UserAccountAssociation.where(user_id: shard_user_ids).to_a
      end.each do |aa|
        key = [aa.user_id, aa.account_id]
        current_associations[key] = [aa.id, aa.depth]
      end

      account_id_to_root_account_id = Account.where(id: precalculated_associations&.keys).pluck(:id, Arel.sql(Account.resolved_root_account_id_sql)).to_h

      users_or_user_ids.uniq.sort_by { |u| u.try(:id) || u }.each do |user_id|
        if user_id.is_a? User
          user = user_id
          user_id = user.id
        end

        account_ids_with_depth = precalculated_associations
        if account_ids_with_depth.nil?
          user ||= User.find(user_id)
          account_ids_with_depth = if %w[creation_pending deleted].include?(user.workflow_state) || user.fake_student?
                                     []
                                   else
                                     calculate_account_associations_from_accounts(account_mappings[user.id], account_chain_cache)
                                   end
        end

        account_ids_with_depth.sort_by(&:first).each do |account_id, depth|
          key = [user_id, account_id]
          association = current_associations[key]
          if association.nil?
            # new association, create it
            aa = UserAccountAssociation.new
            aa.user_id = user_id
            aa.account_id = account_id
            aa.root_account_id = account_id_to_root_account_id[account_id]
            aa.depth = depth
            aa.shard = Shard.shard_for(account_id)
            aa.shard.activate do
              UserAccountAssociation.transaction(requires_new: true) do
                aa.save!
              end
            rescue ActiveRecord::RecordNotUnique
              # race condition - someone else created the UAA after we queried for existing ones
              old_aa = UserAccountAssociation.where(user_id: aa.user_id, account_id: aa.account_id).first
              raise unless old_aa # wtf!

              # make sure we don't need to change the depth
              if depth < old_aa.depth
                old_aa.depth = depth
                old_aa.save!
              end
            end
          else
            # for incremental, only update the old association if it is deeper than the new one
            # for non-incremental, update it if it changed
            if (incremental && association[1] > depth) || (!incremental && association[1] != depth)
              UserAccountAssociation.where(id: association[0]).update_all(depth:)
            end
            # remove from list of existing for non-incremental
            current_associations.delete(key) unless incremental
          end
        end
      end

      to_delete += current_associations.map { |_k, v| v[0] }
      UserAccountAssociation.where(id: to_delete).delete_all unless incremental || to_delete.empty?
    end
  end

  # These methods can be overridden by a plugin if you want to have an approval
  # process or implement additional tracking for new users

### new_teacher_registration

DEPRECATED, override new_registration instead

### group_memberships_for

DON'T use ||=, because that will cause an immediate save to the db if it
    # doesn't already exist
    self.uuid = CanvasSlug.generate_securish_uuid unless self["uuid"]
  end
  protected :assign_uuid

  scope :with_service, lambda { |service|
    service = service.service if service.is_a?(UserService)
    eager_load(:user_services).where(user_services: { service: service.to_s })
  }
  scope :enrolled_before, ->(date) { where("enrollments.created_at<?", date) }

### visible_groups

Returns an array of groups which are currently visible for the user.

### last_name_first

compatibility only - this isn't really last_name_first

### self

Feel free to add, but the "authoritative" list (http://en.wikipedia.org/wiki/Title_(name)) is quite large
  SUFFIXES = /^(Sn?r\.?|Senior|Jn?r\.?|Junior|II|III|IV|V|VI|Esq\.?|Esquire)$/i

  # see also user_sortable_name.js

### self

Doe, John, Sr.
    # Otherwise change Ho, Chi, Min to Ho, Chi Min
    if suffix && suffix !~ SUFFIXES
      given = "#{given} #{suffix}"
      suffix = nil
    end

    if given
      # John Doe, Sr.
      if !likely_already_surname_first && !suffix && surname =~ /\s/ && given =~ SUFFIXES
        suffix = given
        given = surname
        surname = nil
      end
    else
      # John Doe
      given = name.strip
      surname = nil
    end

    given_parts = given.split
    # John Doe Sr.
    if !suffix && given_parts.length > 1 && given_parts.last =~ SUFFIXES
      suffix = given_parts.pop
    end
    # Use prior information on the last name to try and reconstruct it
    prior_surname_parts = nil
    surname = given_parts.pop(prior_surname_parts.length).join(" ") if !surname && prior_surname.present? && (prior_surname_parts = prior_surname.split) && !prior_surname_parts.empty? && given_parts.length >= prior_surname_parts.length && given_parts[-prior_surname_parts.length..] == prior_surname_parts
    # Last resort; last name is just the last word given
    surname = given_parts.pop if !surname && given_parts.length > 1

    [given_parts.empty? ? nil : given_parts.join(" "), surname, suffix]
  end

### lookup_lti_id

recalculate the sortable name if the name changed, but the sortable name didn't, and the sortable_name matches the old name
    self.sortable_name = nil if !sortable_name_changed? &&
                                !sortable_name_explicitly_set &&
                                name_changed? &&
                                User.name_parts(sortable_name, likely_already_surname_first: true).compact.join(" ") == name_was
    unless self["sortable_name"]
      self.sortable_name = User.last_name_first(self.name, sortable_name_was, likely_already_surname_first: true)
    end
    self.reminder_time_for_due_dates ||= 48.hours.to_i
    self.reminder_time_for_grading ||= 0
    self.initial_enrollment_type = nil unless %w[student teacher ta observer].include?(initial_enrollment_type)
    self.lti_id ||= SecureRandom.uuid
    true
  end

  # Because some user's can have old lti ids that differ from self.lti_id,
  # which also depends on the current context.

### email

It's already ordered, so find the first one, if there's one.
    if communication_channels.loaded?
      communication_channels.to_a.find { |cc| cc.path_type == "email" && cc.workflow_state != "retired" }
    else
      communication_channels.email.unretired.first
    end
  end

### email_cache_key

this sillyness is because rails equates falsey as not in the cache
      (value == :none) ? nil : value
    end
  end

### sms_channel

If the email already exists but with different casing this allows us to change it
      cc.path = e
      cc.user = self
    end
    cc.move_to_top
    cc.workflow_state = "unconfirmed" if cc.retired?
    cc.save!
    reload
    clear_email_cache!
    cc.path
  end

### sms

It's already ordered, so find the first one, if there's one.
    communication_channels.sms.first
  end

### unavailable

Not listing this first so it is not the default.
    state :pending_approval do
      event :approve, transitions_to: :pre_registered
      event :reject, transitions_to: :deleted
    end

    state :creation_pending do
      event :create_user, transitions_to: :pre_registered
      event :register, transitions_to: :registered
    end

    state :registered

    state :deleted
  end

### delete_enrollments

avoid extraneous callbacks when enrolled in multiple sections

### associate_with_shard

make sure to hit all shards
        enrollment_scope = enrollments.shard(self)
        user_observer_scope = as_student_observation_links.shard(self)
        user_observee_scope = as_observer_observation_links.shard(self)
        pseudonym_scope = pseudonyms.active.shard(self)
        account_users = self.account_users.active.shard(self)
        has_other_root_accounts = false
        group_memberships_scope = group_memberships.active.shard(self)

        # eportfolios will only be in the users home shard
        eportfolio_scope = eportfolios.active
      else
        # make sure to do things on the root account's shard. but note,
        # root_account.enrollments won't include the student view user's
        # enrollments, so we need to fetch them off the user instead; the
        # student view user won't be cross shard, so that will still be the
        # right shard
        enrollment_scope = fake_student? ? enrollments : root_account.enrollments.where(user_id: self)
        user_observer_scope = as_student_observation_links.shard(self)
        user_observee_scope = as_observer_observation_links.shard(self)

        pseudonym_scope = root_account.pseudonyms.active.where(user_id: self)

        account_users = root_account.account_users.where(user_id: self).to_a +
                        self.account_users.shard(root_account).where(account_id: root_account.all_accounts).to_a
        has_other_root_accounts = associated_accounts.shard(self).where.not(accounts: { id: root_account }).exists?
        group_memberships_scope = group_memberships.active.shard(root_account.shard).joins(:group).where(groups: { root_account_id: root_account })

        eportfolio_scope = eportfolios.active if shard == root_account.shard
      end

      delete_enrollments(enrollment_scope, updating_user:)
      group_memberships_scope.destroy_all
      user_observer_scope.destroy_all
      user_observee_scope.destroy_all
      eportfolio_scope&.in_batches&.destroy_all
      pseudonym_scope.each(&:destroy)
      account_users.each(&:destroy)

      # only delete the user's communication channels when the last account is
      # removed (they don't belong to any particular account). they will always
      # be on the user's shard
      communication_channels.unretired.each(&:destroy) unless has_other_root_accounts

      update_account_associations
    end
    reload
  end

### assert_name

Overwrites the old user name, if there was one.  Fills in the new one otherwise.

### check_courses_right

this list should be longer if the person has admin privileges...
    courses
  end

### check_accounts_right

Look through the currently enrolled courses first.  This should
    # catch most of the calls.  If none of the current courses grant
    # the right then look at the concluded courses.
    enrollments_to_check ||= enrollments.current_and_concluded

    shards = associated_shards & user.associated_shards
    # search the current shard first
    shards.delete(Shard.current) && shards.unshift(Shard.current) if shards.include?(Shard.current)

    courses_for_enrollments(enrollments_to_check.shard(shards)).any? { |c| c.grants_right?(user, sought_right) }
  end

### active_merged_into_user

check if the user we are given is an admin in one of this user's accounts
    return false unless user && sought_right
    return account.grants_right?(user, sought_right) if fake_student? # doesn't have account association

    # Intentionally include deleted pseudonyms when checking deleted users (important for diagnosing deleted users)
    accounts_to_search =
      if associated_accounts.empty?
        if merged_into_user && active_merged_into_user
          return active_merged_into_user.check_accounts_right?(user, sought_right)
        elsif Account.joins(:pseudonyms).where(pseudonyms: { user_id: id }).exists?
          Account.joins(:pseudonyms).where(pseudonyms: { user_id: id })
        else
          associated_accounts
        end
      else
        associated_accounts
      end

    common_shards = associated_shards & user.associated_shards
    search_method = lambda do |shard|
      # new users with creation pending enrollments don't have account associations
      if accounts_to_search.shard(shard).empty? && common_shards.length == 1 && !unavailable?
        account.grants_right?(user, sought_right)
      else
        accounts_to_search.shard(shard).any? { |a| a.grants_right?(user, sought_right) }
      end
    end
    # search shards the two users have in common first, since they're most likely
    return true if common_shards.any?(&search_method)
    # now do an exhaustive search, since it's possible to have admin permissions for accounts
    # you're not associated with
    return true if (associated_shards - common_shards).any?(&search_method)

    false
  end

### can_change_pronunciation

by default this means that the user we are given is an administrator
    # of an account of one of the courses that this user is enrolled in, or
    # an admin (teacher/ta/designer) in the course
    given { |user| check_courses_right?(user, :read_reports) }
    can :read_profile and can :remove_avatar and can :read_reports

    %i[read_email_addresses read_sis manage_sis].each do |permission|
      given { |user| check_courses_right?(user, permission) }
      can permission
    end

    given { |user| check_courses_right?(user, :generate_observer_pairing_code, enrollments.not_deleted) }
    can :generate_observer_pairing_code

    given { |user| check_accounts_right?(user, :view_statistics) }
    can :view_statistics

    given { |user| check_accounts_right?(user, :manage_students) }
    can :read_profile and can :read_reports and can :read_grades

    given { |user| check_accounts_right?(user, :manage_user_logins) }
    can %i[read read_reports read_profile api_show_user terminate_sessions read_files]

    given { |user| check_accounts_right?(user, :read_roster) }
    can :read_full_profile and can :api_show_user

    given { |user| check_accounts_right?(user, :view_all_grades) }
    can :read_grades

    given { |user| check_accounts_right?(user, :view_user_logins) }
    can :view_user_logins

    given { |user| check_accounts_right?(user, :read_email_addresses) }
    can :read_email_addresses

    given do |user|
      check_accounts_right?(user, :manage_user_logins) && adminable_accounts.select(&:root_account?).all? { |a| has_subset_of_account_permissions?(user, a) }
    end
    can :manage_user_details and can :rename and can :update_avatar and can :remove_avatar and
      can :manage_feature_flags and can :view_feature_flags and can :update_profile

    given { |user| pseudonyms.shard(self).any? { |p| p.grants_right?(user, :update) } }
    can :merge

    given do |user|
      # a user can reset their own MFA, but only if the setting isn't required
      (self == user && mfa_settings != :required) ||

        # a site_admin with permission to reset_any_mfa
        Account.site_admin.grants_right?(user, :reset_any_mfa) ||
        # an admin can reset another user's MFA only if they can manage *all*
        # of the user's pseudonyms
        (self != user && pseudonyms.shard(self).all? do |p|
          p.grants_right?(user, :update) ||
            # the account does not have mfa enabled
            p.account.mfa_settings == :disabled ||
            # they are an admin user and have reset MFA permission
            p.account.grants_right?(user, :reset_any_mfa)
        end)
    end
    can :reset_mfa

    given { |user| user && user.as_observer_observation_links.where(user_id: id).exists? }
    can %i[read read_as_parent read_files]

    given { |user| check_accounts_right?(user, :moderate_user_content) }
    can :moderate_user_content

    given { |user| trusted_account&.grants_right?(user, :manage_user_logins) }
    can :read
  end

### self

student view should only ever have enrollments in a single course
    return true if fake_student?
    return false unless
        account.grants_right?(masquerader, nil, :become_user) && SisPseudonym.for(self, account, type: :implicit, require_sis: false)

    if account.root_account.feature_enabled?(:course_admin_role_masquerade_permission_check)
      return false unless includes_subset_of_course_admin_permissions?(masquerader, account)
    end

    has_subset_of_account_permissions?(masquerader, account)
  end

### includes_subset_of_course_admin_permissions

iterate and set permissions
      # we want the highest level permission set the user is authorized for
      result[permission] = true if enrollments.any? { |e| e.has_permission_to?(permission) }
    end
    result
  end

### avatar_image

Public: Set a user's avatar image. This is a convenience method that sets
  #   the avatar_image_source, avatar_image_url, avatar_updated_at, and
  #   avatar_state on the user model.
  #
  # val - A hash of options used to configure the avatar.
  #       :type - The type of avatar. Should be 'gravatar,'
  #         'external,' or 'attachment.'
  #       :url - The URL of the gravatar. Used for types 'external' and
  #         'attachment.'
  #
  # Returns nothing if avatar is set; false if avatar is locked.

### report_avatar_image

Return here if we're passed a nil val or any non-hash val (both of which
    # will just nil the user's avatar).
    unless val.is_a?(Hash)
      assign_attributes(blank_avatar)
      return
    end

    only_includes_state = val["url"].blank? && val["type"].blank? && val["state"].present?

    self.avatar_image_updated_at = Time.zone.now

    if only_includes_state
      self.avatar_state = val["state"]
    elsif val["url"]&.match?(GRAVATAR_PATTERN)
      self.avatar_image_source = "gravatar"
      self.avatar_image_url = val["url"]
      self.avatar_state = "submitted"
    elsif val["type"] == "attachment" && val["url"]
      self.avatar_image_source = "attachment"
      self.avatar_image_url = val["url"]
      self.avatar_state = "submitted"
    elsif val["url"] && external_avatar_url_patterns.find { |p| val["url"].match?(p) }
      self.avatar_image_source = "external"
      self.avatar_image_url = val["url"]
      self.avatar_state = "submitted"
    else
      assign_attributes(blank_avatar)
    end
  end

### clear_avatar_image_url_with_uuid

something got built without request context, so we want to inherit that
        # context now that we have a request
        if uri.host == "localhost"
          uri.scheme = request.scheme
          uri.host = request.host
          uri.port = request.port unless [80, 443].include?(request.port)
        end
        uri.scheme ||= request ? request.protocol[0..-4] : HostUrl.protocol # -4 to chop off the ://
        if HostUrl.cdn_host
          uri.host = HostUrl.cdn_host
        elsif request && !uri.host
          uri.host = request.host
          uri.port = request.port unless [80, 443].include?(request.port)
        elsif !uri.host
          uri.host, port = HostUrl.default_host.split(":")
          uri.port = Integer(port) if port
        end
        return uri.to_s
      end
    rescue URI::InvalidURIError
      # ignore
    end
    avatar_fallback_url(default_avatar_fallback, request)
  end

  # Clear the avatar_image_url attribute and save it if the URL contains the given uuid.
  #
  # ==== Arguments
  # * <tt>uuid</tt> - The Attachment#uuid value for the file. Used as part of the url identifier.

### dashboard_positions

translate asset strings to be relative to current shard
      colors_hash = colors_hash.filter_map do |asset_string, value|
        opts = asset_string.split("_")
        id_relative_to_user_shard = opts.pop.to_i
        next if id_relative_to_user_shard > Shard::IDS_PER_SHARD && Shard.shard_for(id_relative_to_user_shard) == shard # this is old data and should be ignored

        new_id = Shard.relative_id_for(id_relative_to_user_shard, shard, Shard.current)
        ["#{opts.join("_")}_#{new_id}", value]
      end.to_h
    end

    return apply_contrast colors_hash if prefers_high_contrast?

    colors_hash
  end

### dashboard_view

Use the user's preferences for the default view
  # Otherwise, use the account's default (if set)
  # Fallback to using cards (default option on the Account settings page)

### unread_submission_annotations

serialize ids relative to the user
    shard.activate do
      closed << announcement.id
    end
    set_preference(:closed_notifications, closed.uniq)
  end

### unread_rubric_assessments

this will delete the user_preference_value
    set_preference(:unread_submission_annotations, submission.global_id, nil)
  end

### add_to_visited_tabs

this will delete the user_preference_value
    set_preference(:unread_rubric_comments, submission.global_id, nil)
  end

### default_notifications_disabled

if this is set then all notifications will be disabled by default
    # for the user and will need to be explicitly enabled
    preferences[:default_notifications_disabled] = val
  end

### uuid

***** OHI If you're going to add a lot of data into `preferences` here maybe take a look at app/models/user_preference_value.rb instead ***
  # it will store the data in a separate table on the db and lighten the load on poor `users`

### self

Compute a hash of the user's uuid for security and privacy reasons
    Digest::SHA256.hexdigest(uuid)
  end

### account

Legacy method - don't use this since users may belong to multiple accounts and this method is
  # not aware of context

### temporary_invitations

Limit favorite courses based on current shard.
          if association == :favorite_courses
            ids = favorite_context_ids("Course")
            if ids.empty?
              scope = scope.none
            else
              shards &= ids.map { |id| Shard.shard_for(id) }
              scope = scope.where(id: ids)
            end
          end

          GuardRail.activate(:secondary) do
            Shard.with_each_shard(shards) do
              scope.select("courses.*, enrollments.id AS primary_enrollment_id, enrollments.type AS primary_enrollment_type, enrollments.role_id AS primary_enrollment_role_id, #{Enrollment.type_rank_sql} AS primary_enrollment_rank, enrollments.workflow_state AS primary_enrollment_state, enrollments.created_at AS primary_enrollment_date")
                   .order(Arel.sql("courses.id, #{Enrollment.type_rank_sql}, #{Enrollment.state_rank_sql}"))
                   .distinct_on(:id).shard(Shard.current)
            end
          end
        end
        result.dup
      end

      if association == :current_and_invited_courses
        if enrollment_uuid && (pending_course = Course.active
          .select("courses.*, enrollments.type AS primary_enrollment,
                  #{Enrollment.type_rank_sql} AS primary_enrollment_rank,
                  enrollments.workflow_state AS primary_enrollment_state,
                  enrollments.created_at AS primary_enrollment_date")
          .joins(:enrollments)
          .where(enrollments: { uuid: enrollment_uuid, workflow_state: "invited" }).first)
          res << pending_course
          res.uniq!
        end
        pending_enrollments = temporary_invitations
        unless pending_enrollments.empty?
          ActiveRecord::Associations.preload(pending_enrollments, :course)
          res.concat(pending_enrollments.map do |e|
            c = e.course
            c.primary_enrollment_type = e.type
            c.primary_enrollment_role_id = e.role_id
            c.primary_enrollment_rank = e.rank_sortable
            c.primary_enrollment_state = e.workflow_state
            c.primary_enrollment_date = e.created_at
            c.invitation = e.uuid
            c
          end)
          res.uniq!
        end
      end

      Shard.partition_by_shard(res, ->(c) { c.shard }) do |shard_courses|
        roles = Role.where(id: shard_courses.map(&:primary_enrollment_role_id).uniq).to_a.index_by(&:id)
        shard_courses.each { |c| c.primary_enrollment_role = roles[c.primary_enrollment_role_id] }
      end
      @courses_with_primary_enrollment[cache_key] =
        res.sort_by { |c| [c.primary_enrollment_rank, Canvas::ICU.collation_key(c.name)] }
    end
  end

### cached_currentish_enrollments

http://github.com/seamusabshere/cacheable/blob/master/lib/cacheable.rb from the cacheable gem
  # to get a head start

  # this method takes an optional {:include_enrollment_uuid => uuid}   so that you can pass it the session[:enrollment_uuid] and it will include it.

### cached_invitations

this method doesn't include the "active_by_date" scope and should probably not be used since
    # it will give enrollments which are concluded by date
    # leaving this for existing instances where schools are used to the inconsistent behavior
    # participating_enrollments seems to be a more accurate representation of "current courses"
    RequestCache.cache("cached_current_enrollments", self, opts) do
      enrollments = shard.activate do
        res = Rails.cache.fetch_with_batched_keys(
          ["current_enrollments5", opts[:include_future], ApplicationController.region].cache_key,
          batch_object: self,
          batched_keys: :enrollments
        ) do
          scope = (opts[:include_future] ? self.enrollments.current_and_future : self.enrollments.current_and_invited)
          scope.shard(in_region_associated_shards).to_a
        end
        if opts[:include_enrollment_uuid] && !res.find { |e| e.uuid == opts[:include_enrollment_uuid] } &&
           (pending_enrollment = Enrollment.where(uuid: opts[:include_enrollment_uuid], workflow_state: "invited").first)
          res << pending_enrollment
        end
        res
      end + temporary_invitations

      if opts[:preload_dates]
        Canvas::Builders::EnrollmentDateBuilder.preload_state(enrollments)
      end
      if opts[:preload_courses]
        ActiveRecord::Associations.preload(enrollments, :course)
      end
      enrollments
    end
  end

### has_active_enrollment

don't need an expires_at here because user will be touched upon enrollment creation
    @_has_enrollment = Rails.cache.fetch([self, "has_enrollment", ApplicationController.region].cache_key) do
      enrollments.shard(in_region_associated_shards).active.exists?
    end
  end

### has_future_enrollment

don't need an expires_at here because user will be touched upon enrollment activation
    @_has_active_enrollment = Rails.cache.fetch([self, "has_active_enrollment", ApplicationController.region].cache_key) do
      enrollments.shard(in_region_associated_shards).current.active_by_date.exists?
    end
  end

### account_membership

We should be able to remove this method when the planner works for teachers/other course roles
    return @_non_student_enrollment if defined?(@_non_student_enrollment)

    @_non_student_enrollment = Rails.cache.fetch_with_batched_keys(["has_non_student_enrollment", ApplicationController.region].cache_key, batch_object: self, batched_keys: :enrollments) do
      enrollments.shard(in_region_associated_shards).where.not(type: %w[StudentEnrollment StudentViewEnrollment ObserverEnrollment])
                 .where.not(workflow_state: %w[rejected inactive deleted]).exists?
    end
  end

### recent_feedback

when discussion_checkpoints FF is enabled, we filter out parent assignment submissions
          # when that FF is disabled, we filter out sub_assignment submissions
          course_ids_with_active_checkpoints = Course.where(id: course_ids).select(&:discussion_checkpoints_enabled?).map(&:id)
          submissions.delete_if do |sub|
            (sub.assignment.has_sub_assignments? && course_ids_with_active_checkpoints.include?(sub.course_id)) ||
              (sub.assignment.is_a?(SubAssignment) && !course_ids_with_active_checkpoints.include?(sub.course_id))
          end
        end
      end
    end
  end

  # This is only feedback for student contexts (unless specific contexts are passed in)

### can_current_user_view_as_user

dont make the query do an stream_item_instances.context_code IN
    # ('course_20033','course_20237','course_20247' ...) if they dont pass any
    # contexts, just assume it wants any context code.
    if opts[:contexts]
      # still need to optimize the query to use a root_context_code.  that way a
      # users course dashboard even if they have groups does a query with
      # "context_code=..." instead of "context_code IN ..."
      instances = instances.where(context: opts[:contexts])
    elsif opts[:context]
      instances = instances.where(context: opts[:context])
    elsif opts[:only_active_courses]
      instances = instances.where(context_type: "Course", context_id: participating_course_ids)
    end

    instances
  end

### cached_recent_stream_items

User is viewing as themselves (always allowed)
    return true if user.id == id

    # Check if current user has admin rights in the course
    has_admin_rights = course.grants_right?(self, :read_as_admin)

    # Check if current user can masquerade as the user (handles cross-shard/tenant security)
    can_masquerade = can_masquerade?(user, course.account)

    # Check if the user has enrollments in this course (to prevent viewing users from other courses)
    has_enrollment = user.enrollments.where(course_id: course.id).exists?

    # Observer permissions - can view students they're observing
    observer_permissions = ObserverEnrollment.observed_students(course, self, include_restricted_access: false)
                                             .keys.any? { |observed_user| observed_user.id == user.id }

    # Allow if admin with user enrolled in course, or has masquerade permission, or is an observer
    (has_admin_rights && has_enrollment) || can_masquerade || observer_permissions
  end

  # NOTE: excludes submission stream items

### recent_stream_items

just cache on the user's shard... makes cache invalidation much
    # easier if we visit other shards
    shard.activate do
      if opts[:contexts]
        items = []
        Array(opts[:contexts]).each do |context|
          items.concat(
            Rails.cache.fetch(StreamItemCache.recent_stream_items_key(self, context.class.base_class.name, context.id),
                              expires_in:) do
              recent_stream_items(context:)
            end
          )
        end
        items.sort_by(&:id).reverse
      else
        # no context in cache key
        Rails.cache.fetch(StreamItemCache.recent_stream_items_key(self), expires_in:) do
          recent_stream_items
        end
      end
    end
  end

  # NOTE: excludes submission stream items

### course_ids_with_checkpoints_enabled

if we're looking through a lot of courses, we should probably not spend a lot of time
    # computing which sections are visible or not before we make the db call;
    # instead, i think we should pull for all the sections and filter after the fact
    filter_after_db = !opts[:use_db_filter] &&
                      (context_codes.grep(/\Acourse_\d+\z/).count > Setting.get("filter_events_by_section_code_threshold", "25").to_i)

    section_codes = section_context_codes(context_codes, filter_after_db)
    limit = filter_after_db ? opts[:limit] * 2 : opts[:limit] # pull extra events just in case
    events = CalendarEvent.active.for_user_and_context_codes(self, context_codes, section_codes)
                          .between(now, opts[:end_at]).limit(limit).order(:start_at).to_a.reject(&:hidden?)

    if filter_after_db
      original_count = events.count
      if events.any? { |e| e.context_code.start_with?("course_section_") }
        section_ids = events.map(&:context_code).grep(/\Acourse_section_\d+\z/).map { |s| s.delete_prefix("course_section_").to_i }
        section_course_codes = Course.joins(:course_sections).where(course_sections: { id: section_ids })
                                     .pluck(:id).map { |id| "course_#{id}" }
        visible_section_codes = section_context_codes(section_course_codes)
        events.reject! { |e| e.context_code.start_with?("course_section_") && !visible_section_codes.include?(e.context_code) }
        events = events.first(opts[:limit]) # strip down to the original limit
      end

      # if we've filtered too many (which should be unlikely), just fallback on the old behavior
      if original_count >= opts[:limit] && events.count < opts[:limit]
        return upcoming_events(opts.merge(use_db_filter: true))
      end
    end

    assignments = Assignment.published
                            .for_context_codes(context_codes)
                            .due_between_with_overrides(now, opts[:end_at])
                            .include_submitted_count.to_a

    if assignments.any?
      if AssignmentOverrideApplicator.should_preload_override_students?(assignments, self, "upcoming_events")
        AssignmentOverrideApplicator.preload_assignment_override_students(assignments, self)
      end

      events += select_available_assignments(
        select_upcoming_assignments(assignments.map { |a| a.overridden_for(self) }, opts.merge(time: now))
      )
    end

    if course_ids_with_checkpoints_enabled.any?
      sub_assignments = SubAssignment.published
                                     .for_course(course_ids_with_checkpoints_enabled)
                                     .due_between_with_overrides(now, opts[:end_at])
                                     .include_submitted_count.to_a

      if sub_assignments.any?
        if AssignmentOverrideApplicator.should_preload_override_students?(sub_assignments, self, "upcoming_events")
          AssignmentOverrideApplicator.preload_assignment_override_students(sub_assignments, self)
        end

        events += select_available_assignments(
          select_upcoming_assignments(sub_assignments.map { |a| a.overridden_for(self) }, opts.merge(time: now))
        )
      end
    end

    sorted_events = events.sort_by do |e|
      due_date = e.start_at
      if e.respond_to? :dates_hash_visible_to
        e.dates_hash_visible_to(self).any? do |due_hash|
          due_date = due_hash[:due_at] if due_hash[:due_at]
        end
      end
      [due_date ? 0 : 1, due_date || 0, Canvas::ICU.collation_key(e.title)]
    end

    sorted_events.uniq.first(opts[:limit])
  end

### cached_context_codes

TODO: All the event methods use this and it's really slow.
    Array(contexts).map(&:asset_string)
  end

### cached_course_ids_for_observed_user

(hopefully) don't need to include cross-shard because calendar events/assignments/etc are only seached for on current shard anyway
    @cached_context_codes ||=
      Rails.cache.fetch([self, "cached_context_codes", Shard.current].cache_key, expires_in: 15.minutes) do
        group_ids = groups.active.pluck(:id)
        cached_current_course_ids = Rails.cache.fetch([self, "cached_current_course_ids", Shard.current].cache_key) do
          # don't need an expires at because user will be touched if enrollment state changes from 'active'
          enrollments.shard(Shard.current).current.active_by_date.distinct.pluck(:course_id)
        end

        cached_current_course_ids.map { |id| "course_#{id}" } + group_ids.map { |id| "group_#{id}" }
      end
  end

### appointment_context_codes

context codes of things that might have a schedulable appointment for the
  # given user, i.e. courses and sections

### conversation_context_codes

Public: Return an array of context codes this user belongs to.
  #
  # include_concluded_codes - If true, include concluded courses (default: true).
  #
  # Returns an array of context code strings.

### root_admin_for

Don't include roles for deleted accounts and don't cache
    # the results.
    return user_roles(root_account, true) if exclude_deleted_accounts

    RequestCache.cache("user_roles", self, root_account) do
      root_account.shard.activate do
        base_key = ["user_roles_for_root_account5", root_account.global_id].cache_key
        Rails.cache.fetch_with_batched_keys(base_key, batch_object: self, batched_keys: [:enrollments, :account_users]) do
          user_roles(root_account)
        end
      end
    end
  end

### initiate_conversation

For jobs/rails consoles/specs where domain root account is not set
    return true unless Account.current_domain_root_account

    associated_root_accounts.empty? ||
      (associated_root_accounts.include?(Account.current_domain_root_account) && Account.current_domain_root_account.settings[:enable_eportfolios] != false)
  end

### reset_unread_conversations_counter

Public: Reset the user's cached unread conversations count.
  #
  # Returns nothing.

### favorite_context_ids

Public: Returns a unique list of favorite context type ids relative to the active shard.
  #
  # Examples
  #
  #   favorite_context_ids("Course")
  #   # => [1, 2, 3, 4]
  #
  # Returns an array of unique global ids.

### menu_courses

Only get the users favorites from their shard.
      shard.activate do
        # Get favorites and map them to their global ids.
        context_ids = favorites.where(context_type:).pluck(:context_id).map { |id| Shard.global_id_for(id) }
        @favorite_context_ids[context_type] = context_ids
      end
    end

    # Return ids relative for the current shard
    context_ids.map do |id|
      Shard.relative_id_for(id, shard, Shard.current)
    end
  end

### user_can_edit_name

this terribleness is so we try to make sure that the newest courses show up in the menu
    courses = courses_with_primary_enrollment(:current_and_invited_courses, enrollment_uuid, opts)
              .sort_by { |c| [c.primary_enrollment_rank, Time.zone.now - (c.primary_enrollment_date || Time.zone.now)] }
              .first(Setting.get("menu_course_limit", "20").to_i)
              .sort_by { |c| [c.primary_enrollment_rank, Canvas::ICU.collation_key(c.name)] }
    favorites = courses_with_primary_enrollment(:favorite_courses, enrollment_uuid, opts)
                .select { |c| can_favorite.call(c) }
    # if favoritable courses (classic courses or k5 courses with admin enrollment) exist, show those and all non-favoritable courses
    @menu_courses = if favorites.empty?
                      courses
                    else
                      favorites + courses.reject { |c| can_favorite.call(c) }
                    end
    ActiveRecord::Associations.preload(@menu_courses, :enrollment_term)
    @menu_courses.reject do |c|
      c.horizon_course? && !c.grants_right?(self, :read_as_admin)
    end
  end

### find_or_initialize_pseudonym_for_account

account = the account that you want a pseudonym for
  # preferred_template_account = pass in an actual account if you have a preference for which account the new pseudonym gets copied from
  # this may not be able to find a suitable pseudonym to copy, so would still return nil
  # if a pseudonym is created, it is *not* saved, and *not* added to the pseudonyms collection

### fake_student

list of copyable pseudonyms
      active_pseudonyms = all_active_pseudonyms(:reload).select { |p| !p.password_auto_generated? && !p.account.delegated_authentication? }
      templates = []
      # re-arrange in the order we prefer
      templates.concat(active_pseudonyms.select { |p| p.account_id == preferred_template_account.id }) if preferred_template_account
      templates.concat(active_pseudonyms.select { |p| p.account_id == Account.site_admin.id })
      templates.concat(active_pseudonyms.select { |p| p.account_id == Account.default.id })
      templates.concat(active_pseudonyms)
      templates.uniq!

      template = templates.detect { |t| !account.pseudonyms.active.by_unique_id(t.unique_id).first }
      if template
        # creating this not attached to the user's pseudonyms is intentional
        pseudonym = account.pseudonyms.build
        pseudonym.user = self
        pseudonym.unique_id = template.unique_id
        pseudonym.password_salt = template.password_salt
        pseudonym.crypted_password = template.crypted_password
      end
    end
    pseudonym
  end

### mfa_settings

mfa settings for a user are the most restrictive of any pseudonyms the user has
  # a login for

### weekly_notification_bucket

try to short-circuit site admins where it is required
    if pseudonym_hint
      mfa_settings = pseudonym_hint.account.mfa_settings
      return :required if mfa_settings == :required ||
                          (mfa_settings == :required_for_admins && !pseudonym_hint.account.cached_all_account_users_for(self).empty?)
    end
    return :required if pseudonym_hint&.authentication_provider&.mfa_required?

    pseudonyms = self.pseudonyms.shard(self).preload(:account, authentication_provider: :account)
    return :required if pseudonyms.any? { |p| p.authentication_provider&.mfa_required? }

    result = pseudonyms.map(&:account).uniq.map do |account|
      case account.mfa_settings
      when :disabled
        0
      when :optional
        1
      when :required_for_admins
        # if pseudonym_hint is given, and we got to here, we don't need
        # to redo the expensive all_account_users_for check
        if (pseudonym_hint && pseudonym_hint.account == account) ||
           account.cached_all_account_users_for(self).empty?
          1
        else
          # short circuit the entire method
          return :required
        end
      when :required
        # short circuit the entire method
        return :required
      end
    end.max
    return :disabled if result.nil?

    [:disabled, :optional][result]
  end

### daily_notification_time

place in the next 24 hours after saturday morning midnight is
    # determined by account and user. messages for any user in the same
    # account (on the same shard) map into the same 6-hour window, and then
    # are spread within that window by user. this is specifically 24 real
    # hours, not 1 day, because DST sucks. so it'll go to 1am sunday
    # morning and 11pm saturday night on the DST transition days, but
    # midnight sunday morning the rest of the time.
    account_bucket = (shard.id.to_i + pseudonym.try(:account_id).to_i) % DelayedMessage::WEEKLY_ACCOUNT_BUCKETS
    user_bucket = id % DelayedMessage::MINUTES_PER_WEEKLY_ACCOUNT_BUCKET
    (account_bucket * DelayedMessage::MINUTES_PER_WEEKLY_ACCOUNT_BUCKET) + user_bucket
  end

### weekly_notification_time

The time daily notifications are sent out is 6pm local time. This is
    # referencing the definition in our documentation and in DelayedMessage#set_send_at
    time_zone = self.time_zone || ActiveSupport::TimeZone["America/Denver"] || Time.zone
    target = time_zone.now.change(hour: 18)
    target += 1.day if target < time_zone.now
    target
  end

### weekly_notification_range

weekly notification scheduling happens in Eastern-time
    time_zone = ActiveSupport::TimeZone.us_zones.find { |zone| zone.name == "Eastern Time (US & Canada)" }

    # start at midnight saturday morning before next monday
    target = time_zone.now.next_week - 2.days

    minutes = weekly_notification_bucket.minutes

    # if we're already past that (e.g. it's sunday or late saturday),
    # advance by a week
    target += 1.week if target + minutes < time_zone.now

    # move into the 24 hours after midnight saturday morning and return
    target + minutes
  end

### self

weekly notification scheduling happens in Eastern-time
    time_zone = ActiveSupport::TimeZone.us_zones.find { |zone| zone.name == "Eastern Time (US & Canada)" }

    # start on January first instead of "today" to avoid DST, but still move to
    # a saturday from there so we get the right day-of-week on start_hour
    target = time_zone.now.change(month: 1, day: 1).next_week - 2.days + weekly_notification_bucket.minutes

    # 2 hour on-the-hour span around the target such that distance from the
    # start hour is at least 30 minutes.
    start_hour = target - 30.minutes
    start_hour = start_hour.change(hour: start_hour.hour)
    end_hour = start_hour + 2.hours

    [start_hour, end_hour]
  end

  # Given a text string, return a value suitable for the user's initial_enrollment_type.
  # It supports strings formatted as enrollment types like "StudentEnrollment" and
  # it also supports text like "student", "teacher", "observer" and "ta".
  #
  # Any unsupported types have +nil+ returned.

### self

Convert the string "StudentEnrollment" to "student".
    # Return only valid matching types. Otherwise, nil.
    type = type.to_s.downcase.sub(/(view)?enrollment/, "")
    %w[student teacher ta observer].include?(type) ? type : nil
  end

### adminable_accounts

i couldn't get EXISTS (?) to work multi-shard, so this is happening instead
    account_ids = account_users.active.shard(shard_scope).distinct.pluck(:account_id)
    Account.active.where(id: account_ids)
  end

### generate_one_time_passwords

atomically update used
    return unless one_time_passwords.where(used: false, id: result).update_all(used: true, updated_at: Time.now.utc) == 1

    result
  end

### self

user tokens are returned by UserListV2 and used to bulk-enroll users using information that isn't easy to guess

### pronouns

For jobs/rails consoles/specs where domain root account is not set
    acc = Account.current_domain_root_account || account
    return nil unless acc.can_add_pronouns?

    translate_pronouns(super)
  end

### all_account_calendars

k5 users can still create courses anywhere, even if the setting restricts them to the manually created courses account
    return :teacher if teacher_right && teacher_scope.merge(course_scope).exists? && (account.root_account.teachers_can_create_courses_anywhere? || active_k5_enrollments?)
    return :teacher if teacher_right && teacher_scope.exists? && account == account.root_account.manually_created_courses_account

    student_right = account.root_account.students_can_create_courses?
    student_scope = scope.where(type: %w[StudentEnrollment ObserverEnrollment])
    return :student if student_right && student_scope.merge(course_scope).exists? && (account.root_account.students_can_create_courses_anywhere? || active_k5_enrollments?)
    return :student if student_right && student_scope.exists? && account == account.root_account.manually_created_courses_account
    return :no_enrollments if account.root_account.no_enrollments_can_create_courses? && !scope.exists? && account == account.root_account.manually_created_courses_account

    nil
  end

### adminable_accounts_recursive

Returns all sub accounts that the user can administer
  # On the shard the starting_root_account resides on.
  #
  # This method first plucks (and caches) the adminable account
  # IDs and then makes a second query to fetch the accounts.
  #
  # This two-query approach was taken intentionally: We do have to store
  # the plucked IDs in memory and make a second query, but it
  # means we can return an ActiveRecord::Relation instead of an Array.
  #
  # This is important to prevent initializing _all_ adminable account
  # models into memory, even if this scope is used in a controller processing
  # a request with pagination params that require a single, small page.

