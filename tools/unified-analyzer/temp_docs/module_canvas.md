# ContextModule

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
- belongs_to :root_account
- has_many :context_module_progressions
- has_many :content_tags
- has_many :assignment_overrides
- has_many :assignment_override_students
- has_one :master_content_tag

## Methods

### relock_warning_check

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

class ContextModule < ActiveRecord::Base
  include Workflow
  include SearchTermHelper
  include DuplicatingObjects
  include LockedFor
  include DifferentiableAssignment

  include MasterCourses::Restrictor
  restrict_columns :state, [:workflow_state]
  restrict_columns :settings, %i[prerequisites completion_requirements requirement_count require_sequential_progress]

  belongs_to :context, polymorphic: [:course]
  belongs_to :root_account, class_name: "Account"
  has_many :context_module_progressions, dependent: :destroy
  has_many :content_tags, -> { order("content_tags.position, content_tags.title") }, dependent: :destroy
  has_many :assignment_overrides, dependent: :destroy, inverse_of: :context_module
  has_many :assignment_override_students, dependent: :destroy
  has_one :master_content_tag, class_name: "MasterCourses::MasterContentTag", inverse_of: :context_module
  acts_as_list scope: { context: self, workflow_state: ["active", "unpublished"] }

  serialize :prerequisites
  serialize :completion_requirements
  before_save :infer_position
  before_save :validate_prerequisites
  before_save :confirm_valid_requirements
  before_save :set_root_account_id

  after_save :touch_context
  after_save :invalidate_progressions
  after_save :relock_warning_check
  after_save :clear_discussion_stream_items
  after_save :send_items_to_stream
  validates :workflow_state, :context_id, :context_type, presence: true
  validates :name, presence: { if: :require_presence_of_name }
  attr_accessor :require_presence_of_name

### relock_warning

if the course is already active and we're adding more stringent requirements
    # then we're going to give the user an option to re-lock students out of the modules
    # otherwise they will be able to continue as before
    @relock_warning = false
    return if new_record?

    if context.available? && active?
      if saved_change_to_workflow_state? && workflow_state_before_last_save == "unpublished"
        # should trigger when publishing a prerequisite for an already active module
        @relock_warning = true if context.context_modules.active.any? { |mod| is_prerequisite_for?(mod) }
        # if any of these changed while we were unpublished, then we also need to trigger
        @relock_warning = true if prerequisites.any? || completion_requirements.any? || unlock_at.present?
      end
      if saved_change_to_completion_requirements? && (completion_requirements.to_a - completion_requirements_before_last_save.to_a).present?
        # removing a requirement shouldn't trigger
        @relock_warning = true
      end
      if saved_change_to_prerequisites? && (prerequisites.to_a - prerequisites_before_last_save.to_a).present?
        # ditto with removing a prerequisite
        @relock_warning = true
      end
      if saved_change_to_unlock_at? && unlock_at.present? && unlock_at_before_last_save.blank?
        # adding a unlock_at date should trigger
        @relock_warning = true
      end
    end
  end

### evaluate_all_progressions

don't queue a job unless necessary
        delay_if_production(n_strand: ["evaluate_module_progressions", global_context_id],
                            singleton: "evaluate_module_progressions:#{global_id}")
          .evaluate_all_progressions
      end
      @discussion_topics_to_recalculate&.each do |dt|
        dt.delay_if_production(n_strand: ["evaluate_discussion_topic_progressions", global_context_id],
                               singleton: "evaluate_discussion_topic_progressions:#{dt.global_id}")
          .recalculate_context_module_actions!
      end
    end
  end

### remove_completion_requirement

Keep a cached hash of all modules for a given context and their
    # respective positions -- used when enforcing valid prerequisites
    # and when generating the list of downstream modules
    Rails.cache.fetch(["module_positions", context].cache_key) do
      hash = {}
      context.context_modules.not_deleted.each { |m| hash[m.id] = m.position || 0 }
      hash
    end
  end

### duplicate_content_tag_base_model

This is intended for duplicating a content tag when we are duplicating a module
  # Not intended for duplicating a content tag to keep in the original module

### duplicate_content_tag

Intended for taking a content_tag in this module and duplicating it
  # into a new module.  Not intended for duplicating a content tag to be
  # kept in the same module.

### set_root_account_id

If we have multiple assignments (e.g.) make sure they each get unused titles.
      # A title isn't marked used if the assignment hasn't been saved yet.
      new_tag.content.save!
      new_tag.title = nil
    end
    new_tag
  end
  private :duplicate_content_tag

### update_downstreams

only restore tags deleted (approximately) when the module was deleted
      # (tags are currently set to exactly deleted_at but older deleted modules used the current time on each tag)
      tags_to_restore = content_tags.where(workflow_state: "deleted")
                                    .where("updated_at BETWEEN ? AND ?", deleted_at - 5.seconds, deleted_at + 5.seconds)
                                    .preload(:content)
      tags_to_restore.each do |tag|
        # don't restore the item if the asset has been deleted too
        next if tag.asset_workflow_state == "deleted"

        # although the module will be restored unpublished, the items should match the asset's published state
        tag.workflow_state = if tag.content && tag.sync_workflow_state_to_asset?
                               tag.asset_workflow_state
                             else
                               "unpublished"
                             end
        # deal with the possibility that the asset has been renamed after the module was deleted
        tag.title = Context.asset_name(tag.content) if tag.content && tag.sync_title_to_asset_title?
        tag.save
      end
    end
    self.workflow_state = "unpublished"
    save
  end

### publish_items

TODO: remove the unused argument; it's not sent anymore, but it was sent through a delayed job
    # so compatibility was maintained when sender was updated to not send it
    positions = ContextModule.module_positions(context).to_a.sort_by { |a| a[1] }
    downstream_ids = positions.select { |a| a[1] > (position || 0) }.pluck(0)
    downstreams = downstream_ids.empty? ? [] : context.context_modules.not_deleted.where(id: downstream_ids)
    downstreams.each(&:save_without_touching_context)
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

  scope :active, -> { where(workflow_state: "active") }
  scope :unpublished, -> { where(workflow_state: "unpublished") }
  scope :not_deleted, -> { where("context_modules.workflow_state<>'deleted'") }
  scope :starting_with_name, lambda { |name|
    where("name ILIKE ?", "#{name}%")
  }
  scope :visible_to_students_in_course_with_da, lambda { |user_ids, course_ids|
    visible_module_ids = ModuleVisibility::ModuleVisibilityService.modules_visible_to_students(course_ids:, user_ids:).map(&:context_module_id)
    if visible_module_ids.any?
      where(id: visible_module_ids)
    else
      none
    end
  }

  alias_method :published?, :active?

### locked_for_tag

if the progression is locked, then position in the progression doesn't
    # matter. we're not available.

    tag = opts[:tag]
    avail = progression && !progression.locked? && !locked_for_tag?(tag, progression)
    if !avail && opts[:deep_check_if_needed]
      progression = evaluate_for(progression)
      avail = progression && !progression.locked? && !locked_for_tag?(tag, progression)
    end
    avail
  end

### completion_requirements

validate format, skipping invalid ones
      prereqs = prereqs.select do |pre|
        pre.key?(:id) && pre.key?(:name) && pre[:type] == "context_module"
      end
    when String
      res = []
      module_names = ContextModule.module_names(context)
      pres = prereqs.split(",")
      pre_regex = /module_(\d+)/
      pres.each do |pre|
        next unless (match = pre_regex.match(pre))

        id = match[1].to_i
        if module_names.key?(id)
          res << { id:, type: "context_module", name: module_names[id] }
        end
      end
      prereqs = res
    else
      prereqs = nil
    end
    @prerequisites = nil
    @active_prerequisites = nil
    super
  end

### validate_completion_requirements

requirements hash can contain invalid data (e.g. {"none"=>"none"}) from the ui,
      # filter & manipulate the data to something more reasonable
      val = val.map do |id, req|
        if req.is_a?(Hash)
          req[:id] = id unless req[:id]
          req
        end
      end
      val = validate_completion_requirements(val.compact)
    else
      val = nil
    end
    super
  end

### content_tags_for

always return an array now because filter_tags_for_da *might* return one
      tags.to_a
    end
  end

### cached_not_deleted_tags

don't reload the preloaded content
                              content_tags.select(&:active?)
                            else
                              content_tags.active.to_a
                            end
  end

### add_item

don't reload the preloaded content
                                   content_tags.reject(&:deleted?)
                                 else
                                   content_tags.not_deleted.to_a
                                 end
  end

### insert_items

This method is called both to create a module item and to update one
        # (e.g. in a blueprint course sync.)
        #
        # For new module items (or old module items that don't have a resource
        # link), we create a new ResourceLink if one cannot be found for the
        # lookup_uuid, or if lookup_uuid is not given.
        added_item.associated_asset ||=
          Lti::ResourceLink.find_or_initialize_for_context_and_lookup_uuid(
            context:,
            lookup_uuid: params[:lti_resource_link_lookup_uuid].presence,
            custom: Lti::DeepLinkingUtil.validate_custom_params(params[:custom_params]),
            context_external_tool: content,
            url: params[:url]
          )
      end
    when "context_module_sub_header", "sub_header"
      title = params[:title]
      added_item ||= content_tags.build(context:)
      added_item.attributes = {
        tag_type: "context_module",
        title:,
        indent: params[:indent],
        position:
      }
      added_item.content_id = 0
      added_item.content_type = "ContextModuleSubHeader"
      added_item.context_module_id = id
      added_item.indent = params[:indent] || 0
      added_item.workflow_state = "unpublished" if added_item.new_record?
    else
      return nil unless item

      title = params[:title] || item.try(:title) || item.name
      added_item ||= content_tags.build(context:)
      added_item.attributes = {
        content: item,
        tag_type: "context_module",
        title:,
        indent: params[:indent],
        position:
      }
      added_item.context_module_id = id
      added_item.indent = params[:indent] || 0
      added_item.workflow_state = workflow_state if added_item.new_record?
    end
    added_item.save
    added_item
  end

  # specify a 1-based position to insert the items at; leave nil to append to the end of the module
  # ignores current module item positions in favor of an objective position

### find_or_create_progressions

the write accessor validates for us
    self.completion_requirements = completion_requirements || []
    save if do_save && completion_requirements_changed?
    completion_requirements
  end

