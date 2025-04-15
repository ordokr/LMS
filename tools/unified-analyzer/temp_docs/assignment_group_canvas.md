# AssignmentGroup

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

- belongs_to :context
- has_many :scores
- has_many :assignments
- has_many :active_assignments
- has_many :published_assignments

## Methods

### generate_default_values

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

class AssignmentGroup < ActiveRecord::Base
  include Workflow
  # Unlike our other soft-deletable models, assignment groups use 'available' instead of 'active'
  # to indicate a not-deleted state. This means we have to add the 'available' state here before
  # Canvas::SoftDeletable adds the 'active' and 'deleted' states, so that 'available' becomes the
  # initial state for this model.
  workflow { state :available }
  include Canvas::SoftDeletable

  include MasterCourses::Restrictor
  restrict_columns :content, [:group_weight, :rules]

  attr_readonly :context_id, :context_type

  attr_accessor :saved_by, :validate_rules

  belongs_to :context, polymorphic: [:course]
  acts_as_list scope: { context: self, workflow_state: "available" }
  has_a_broadcast_policy
  serialize :integration_data, type: Hash

  has_many :scores, -> { active }
  has_many :assignments, -> { order("position, due_at, title") }

  has_many :active_assignments,
           lambda {
             where("assignments.workflow_state<>'deleted'").order("assignments.position, assignments.due_at, assignments.title")
           },
           class_name: "Assignment",
           dependent: :destroy

  has_many :published_assignments,
           lambda {
             where(workflow_state: "published").order("assignments.position, assignments.due_at, assignments.title")
           },
           class_name: "Assignment"

  validates :context_id, :context_type, :workflow_state, presence: true
  validates :rules, length: { maximum: maximum_text_length }, allow_blank: true
  validates :default_assignment_name, length: { maximum: maximum_string_length }, allow_nil: true
  validates :name, length: { maximum: maximum_string_length }, allow_nil: true
  validate :validate_assignment_group_rules, if: :validate_rules

  before_create :set_root_account_id
  before_save :set_context_code
  before_save :generate_default_values
  after_save :course_grading_change
  after_save :touch_context
  after_save :update_student_grades

  before_destroy :destroy_scores
  after_destroy :clear_context_has_assignment_group_cache

### rules_hash

It's a pretty good guess that if an assignment was modified at the same
      # time that this group was last modified, that assignment was deleted
      # along with this group. This might help avoid undeleting assignments that
      # were deleted earlier.
      to_restore = to_restore.where(updated_at: updated_at.utc..)
    end
    undestroy(active_state: "available")
    restore_scores
    to_restore.each { |assignment| assignment.restore(:assignment_group) }
  end

### rules_hash

Converts a hash representation of rules to the string representation of rules in the database
  # {
  #   "drop_lowest" => '1',
  #   "drop_highest" => '1',
  #   "never_drop" => ['33','17','24']
  # }
  #
  # drop_lowest:2\ndrop_highest:1\nnever_drop:12\nnever_drop:14\n

### clear_context_has_assignment_group_cache

this is just in case we happen to delete the last assignment_group in a course

### move_assignments_to

We need to update the scope to use AbstractAssignment instead of its subclass Assignment so that we can merge the
      # scope query with the checkpoints_scope query
      scope_assignment_ids = scope.pluck(:id)
      scope = AbstractAssignment.where(id: scope_assignment_ids)
      checkpoints_scope = SubAssignment.active.where(parent_assignment_id: scope_assignment_ids)
      # merge the queries
      scope = scope.or(checkpoints_scope)
    end

    if assignment_ids&.any?
      scope = scope.where(id: assignment_ids)
    end

    includes.any? ? scope.preload(includes) : scope
  end

### restore_scores

TODO: soft-delete score metadata as part of GRADE-746
    set_scores_workflow_state_in_batches(:deleted)
  end

### set_scores_workflow_state_in_batches

TODO: restore score metadata as part of GRADE-746
    set_scores_workflow_state_in_batches(:active, exclude_workflow_states: [:completed, :deleted])
  end

