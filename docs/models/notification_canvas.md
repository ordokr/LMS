# Notification

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

- has_many :messages
- has_many :notification_policies
- has_many :notification_policy_overrides

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

class Notification < Switchman::UnshardedRecord
  include TextHelper

  TYPES_TO_SHOW_IN_FEED = [
    # Assignment
    "Assignment Created",
    "Checkpoints Created",
    "Assignment Changed",
    "Assignment Due Date Changed",
    "Assignment Due Date Override Changed",

    # Submissions / Grading
    "Assignment Graded",
    "Assignment Submitted Late",
    "Grade Weight Changed",
    "Group Assignment Submitted Late",

    # Mentions
    "Discussion Mention",

    # Testing
    "Show In Feed",
  ].freeze

  ALLOWED_PUSH_NOTIFICATION_CATEGORIES = %w[
    announcement
    appointment_availability
    appointment_cancelations
    calendar
    conversation_message
    course_content
    discussion_mention
    reported_reply
    due_date
    grading
    invitation
    student_appointment_signups
    submission_comment
    discussion
    discussion_entry
  ].freeze

  ALLOWED_PUSH_NOTIFICATION_TYPES = [
    "Annotation Notification",
    "Annotation Teacher Notification",
    "Announcement Reply",
    "Appointment Canceled By User",
    "Appointment Deleted For User",
    "Appointment Group Deleted",
    "Appointment Group Published",
    "Appointment Group Updated",
    "Assignment Changed",
    "Assignment Created",
    "Assignment Due Date Changed",
    "Assignment Due Date Override Changed",
    "Assignment Unmuted",
    "Collaboration Invitation",
    "Conversation Message",
    "Discussion Mention",
    "Reported Reply",
    "Event Date Changed",
    "New Announcement",
    "New Event Created",
    "Peer Review Invitation",
    "Quiz Regrade Finished",
    "Rubric Assessment Submission Reminder",
    "Submission Comment",
    "Submission Comment For Teacher",
    "Submission Grade Changed",
    "Submission Graded",
    "Submission Needs Grading",
    "Upcoming Assignment Alert",
    "Web Conference Invitation",
    "New Discussion Topic",
    "New Discussion Entry"
  ].freeze

  NON_CONFIGURABLE_TYPES = %w[Migration Registration Summaries Alert].freeze

  COURSE_TYPES = [
    # Course Activities
    "Due Date",
    "Grading Policies",
    "Course Content",
    "Files",
    "Announcement",
    "Announcement Created By You",
    "Grading",
    "Invitation",
    "All Submissions",
    "Late Grading",
    "Submission Comment",
    "Blueprint",

    # Discussions
    "Discussion",
    "DiscussionEntry",
    "DiscussionMention",
    "ReportedReply",

    # Scheduling
    "Student Appointment Signups",
    "Appointment Signups",
    "Appointment Cancelations",
    "Appointment Availability",
    "Calendar",

    # Conferences
    "Recording Ready"
  ].freeze

  FREQ_IMMEDIATELY = "immediately"
  FREQ_DAILY = "daily"
  FREQ_WEEKLY = "weekly"
  FREQ_NEVER = "never"

  has_many :messages
  has_many :notification_policies, dependent: :destroy
  has_many :notification_policy_overrides, inverse_of: :notification, dependent: :destroy
  before_save :infer_default_content

  validates :name, uniqueness: true

  after_create { self.class.reset_cache! }

### self

this is ugly, but reading from file instead of defined notifications in db
    # because this is used to define valid types in our graphql which needs to
    # exists for specs to be able to use any graphql mutation in this space.
    #
    # the file is loaded by category, category_name so first, then last grabs all the types.
    #  we have a deprecated type that we consider invalid
    # graphql types cannot have spaces we have used underscores
    # and we don't allow editing system notification types
    @configurable_types ||= YAML.safe_load(ERB.new(File.read(Canvas::MessageHelper.find_message_path("notification_types.yml"))).result)
                                .pluck("category")
                                .reject { |type| type.include?("DEPRECATED") }
                                .map { |c| c.gsub(/\s/, "_") } - NON_CONFIGURABLE_TYPES
  end

### create_message

Public: create (and dispatch, and queue delayed) a message
  #  for this notification, associated with the given asset, sent to the given recipients
  #
  # asset - what the message applies to. An assignment, a discussion, etc.
  # to_list - a list of who to send the message to. the list can contain Users, User ids, or communication channels
  # options - a hash of extra options to merge with the options used to build the Message
  #

### self

if user is given, categories that aren't relevant to that user will be
  # filtered out.

### related_user_setting

Return a hash with information for a related user option if one exists.

### names

'Content Link Error',
      # 'DiscussionEntry',
      # 'Late Grading',
      # 'Membership Update',
      # 'Other',
      # 'Reminder',
      # 'Submission Comment',
      # 'TestDaily'
      FREQ_DAILY
    end
  end

  # TODO: i18n: show the localized notification name in the dashboard (or
  # wherever), even if we continue to store the english string in the db
  # (it's actually just the titleized message template filename)

### category_names

Category names should be producible by .titleize, eg due_date => Due Date
  # TODO: i18n ... show these anywhere we show the category today

### category_display_name

Translatable display text to use when representing the category to the user.
  # NOTE: If you add a new notification category, update the mapping file for groupings to show up
  #       on notification preferences page. ui/features/notification_preferences/jquery/NotificationGroupMappings.js

### display_category

Remove the feature flag explanation when :react_discussions_post feature flag is removed
    when "DiscussionMention"
      mt(:discussion_mention_description, <<~MD)
        New Mention in a Discussion.

        *Discussion Mentions are only available
        for courses or accounts that have the
        Discussions/Announcements Redesign
        feature flag turned on.*
      MD
    when "ReportedReply"
      t(:reported_reply_description, "New reported reply in a Discussion")
    when "Due Date"
      t(:due_date_description, "Assignment due date change")
    when "Grading"
      mt(:grading_description, <<~MD)
        Includes:

        * Assignment/submission grade entered/changed
        * Grade weight changed
      MD
    when "Late Grading"
      mt(:late_grading_description, <<~MD)
        *Instructor and Admin only:*

        Late assignment submission
      MD
    when "All Submissions"
      mt(:all_submissions_description, <<~MD)
        *Instructor and Admin only:*

        Assignment (except quizzes) submission/resubmission
      MD
    when "Submission Comment"
      t(:submission_comment_description, "Assignment submission comment")
    when "Grading Policies"
      t(:grading_policies_description, "Course grading policy change")
    when "Invitation"
      mt(:invitation_description, <<~MD)
        Invitation for:

        * Web conference
        * Group
        * Collaboration
        * Peer Review & reminder
      MD
    when "Other"
      mt(:other_description, <<~MD)
        *Instructor and Admin only:*

        * Course enrollment
        * Report generated
        * Content export
        * Migration report
        * New account user
        * New student group
      MD
    when "Calendar"
      t(:calendar_description, "New and changed items on your course calendar")
    when "Student Appointment Signups"
      mt(:student_appointment_description, <<~MD)
        *Instructor and Admin only:*

        Student appointment sign-up
      MD
    when "Appointment Availability"
      t("New appointment timeslots are available for signup")
    when "Appointment Signups"
      t(:appointment_signups_description, "New appointment on your calendar")
    when "Appointment Cancelations"
      t(:appointment_cancelations_description, "Appointment cancellation")
    when "Conversation Message"
      t(:conversation_message_description, "New Inbox messages")
    when "Added To Conversation"
      t(:added_to_conversation_description, "You are added to a conversation")
    when "Conversation Created"
      t(:conversation_created_description, "You created a conversation")
    when "Recording Ready"
      t(:web_conference_recording_ready, "A conference recording is ready")
    when "Membership Update"
      mt(:membership_update_description, <<~MD)
        *Admin only: pending enrollment activated*

        * Group enrollment
        * accepted/rejected
      MD
    when "Blueprint"
      mt(:blueprint_description, <<~MD)
        *Instructor and Admin only:*

        Content was synced from a blueprint course to associated courses
      MD
    when "Content Link Error"
      mt(:content_link_error_description, <<~MD)
        *Instructor and Admin only:*

        Location and content of a failed link that a student has interacted with
      MD
    when "Account Notification"
      mt(:account_notification_description, <<~MD)
        Institution-wide announcements (also displayed on Dashboard pages)
      MD
    else
      t(:missing_description_description, "For %{category} notifications", category:)
    end
  end

