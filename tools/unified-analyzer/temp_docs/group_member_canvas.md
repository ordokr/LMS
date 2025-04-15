# GroupMembership

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

- belongs_to :group
- belongs_to :user

## Methods

### course_broadcast_data

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

class GroupMembership < ActiveRecord::Base
  include Workflow
  extend RootAccountResolver

  belongs_to :group
  belongs_to :user

  validates :group_id, :user_id, :workflow_state, :uuid, presence: true
  before_validation :assign_uuid
  before_validation :verify_section_homogeneity_if_necessary
  validate :validate_within_group_limit

  before_save :auto_join
  before_save :capture_old_group_id

  after_save :ensure_mutually_exclusive_membership
  after_save :touch_groups
  after_save :update_group_leadership
  after_save :invalidate_user_membership_cache
  after_commit :update_cached_due_dates
  after_destroy :touch_groups
  after_destroy :update_group_leadership
  after_destroy :invalidate_user_membership_cache

  has_a_broadcast_policy

  scope :include_user, -> { preload(:user) }

  scope :active, -> { where("group_memberships.workflow_state<>'deleted'") }
  scope :moderators, -> { where(moderator: true) }
  scope :active_for_context_and_users, lambda { |context, users|
    joins(:group).active.where(user_id: users, groups: { context_id: context, workflow_state: "available" })
  }

  scope :for_assignments, lambda { |ids|
    active.joins(group: { group_category: :assignments })
          .merge(Group.active)
          .merge(GroupCategory.active)
          .merge(Assignment.active).where(assignments: { id: ids })
  }

  scope :for_collaborative_groups, -> { joins(:group).merge(Group.collaborative) }
  scope :for_non_collaborative_groups, -> { joins(:group).merge(Group.non_collaborative) }

  scope :for_students, ->(ids) { where(user_id: ids) }

  resolves_root_account through: :group

  alias_method :context, :group

  attr_writer :updating_user

### auto_join

auto accept 'requested' or 'invited' memberships until we implement
  # accepting requests/invitations

### update_cached_due_dates

This method is meant to be used in an after_commit setting

### active_given_enrollments

true iff 'active' and the pair of user and group's course match one of the
  # provided enrollments

