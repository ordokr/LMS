# Notification

## Description

frozen_string_literal: true

## Relationships

- belongs_to :user
- belongs_to :topic
- has_one :shelved_notification

## Methods

### self

frozen_string_literal: true

class Notification < ActiveRecord::Base
  self.ignored_columns = [
    :old_id, # TODO: Remove once 20240829140226_drop_old_notification_id_columns has been promoted to pre-deploy
  ]

  attr_accessor :acting_user
  attr_accessor :acting_username

  belongs_to :user
  belongs_to :topic

  has_one :shelved_notification

  MEMBERSHIP_REQUEST_CONSOLIDATION_WINDOW_HOURS = 24

  validates_presence_of :data
  validates_presence_of :notification_type

  scope :unread, lambda { where(read: false) }
  scope :recent,
        lambda { |n = nil|
          n ||= 10
          order("notifications.created_at desc").limit(n)
        }
  scope :visible,
        lambda {
          joins("LEFT JOIN topics ON notifications.topic_id = topics.id").where(
            "topics.id IS NULL OR topics.deleted_at IS NULL",
          )
        }
  scope :unread_type, ->(user, type, limit = 30) { unread_types(user, [type], limit) }
  scope :unread_types,
        ->(user, types, limit = 30) do
          where(user_id: user.id, read: false, notification_type: types)
            .visible
            .includes(:topic)
            .limit(limit)
        end
  scope :prioritized,
        ->(deprioritized_types = []) do
          scope = order("notifications.high_priority AND NOT notifications.read DESC")

          if deprioritized_types.present?
            scope =
              scope.order(
                DB.sql_fragment(
                  "NOT notifications.read AND notifications.notification_type NOT IN (?) DESC",
                  deprioritized_types,
                ),
              )
          else
            scope = scope.order("NOT notifications.read DESC")
          end

          scope.order("notifications.created_at DESC")
        end

  scope :for_user_menu,
        ->(user_id, limit: 30) do
          where(user_id: user_id).visible.prioritized.includes(:topic).limit(limit)
        end

  attr_accessor :skip_send_email

  after_commit :refresh_notification_count, on: %i[create update destroy]
  after_commit :send_email, on: :create

  after_commit(on: :create) { DiscourseEvent.trigger(:notification_created, self) }

  before_create do
    # if we have manually set the notification to high_priority on create then
    # make sure that is respected
    self.high_priority =
      self.high_priority || Notification.high_priority_types.include?(self.notification_type)
  end

### self

Remove any duplicates by type and topic
    if result.present?
      seen = {}
      to_remove = Set.new

      result.each do |r|
        seen[r.notification_type] ||= Set.new
        if seen[r.notification_type].include?(r.topic_id)
          to_remove << r.id
        else
          seen[r.notification_type] << r.topic_id
        end
      end
      result.reject! { |r| to_remove.include?(r.id) }
    end

    result
  end

  # Clean up any notifications the user can no longer see. For example, if a topic was previously
  # public then turns private.

### data_hash

Be wary of calling this frequently. O(n) JSON parsing can suck.

### self

Update `index_notifications_user_menu_ordering_deprioritized_likes` index when updating this as this is used by
  # `Notification.prioritized_list` to deprioritize like typed notifications. Also See
  # `db/migrate/20240306063428_add_indexes_to_notifications.rb`.

