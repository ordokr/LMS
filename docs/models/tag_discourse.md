# Tag

## Description

frozen_string_literal: true

## Relationships

- has_many :tag_users
- has_many :topic_tags
- has_many :topics
- has_many :category_tag_stats
- has_many :category_tags
- has_many :categories
- has_many :tag_group_memberships
- has_many :tag_groups
- belongs_to :target_tag
- has_many :synonyms
- has_many :sidebar_section_links
- has_many :embeddable_host_tags
- has_many :embeddable_hosts

## Methods

### self

frozen_string_literal: true

class Tag < ActiveRecord::Base
  include Searchable
  include HasDestroyedWebHook
  include HasSanitizableFields

  self.ignored_columns = [
    "topic_count", # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
  ]

  RESERVED_TAGS = [
    "none",
    "constructor", # prevents issues with javascript's constructor of objects
  ]

  validates :name, presence: true, uniqueness: { case_sensitive: false }

  validate :target_tag_validator,
           if: Proc.new { |t| t.new_record? || t.will_save_change_to_target_tag_id? }
  validate :name_validator
  validates :description, length: { maximum: 1000 }

  scope :where_name,
        ->(name) do
          name = Array(name).map(&:downcase)
          where("lower(tags.name) IN (?)", name)
        end

  # tags that have never been used and don't belong to a tag group
  scope :unused,
        -> do
          where(staff_topic_count: 0, pm_topic_count: 0, target_tag_id: nil).joins(
            "LEFT JOIN tag_group_memberships tgm ON tags.id = tgm.tag_id",
          ).where("tgm.tag_id IS NULL")
        end

  scope :used_tags_in_regular_topics,
        ->(guardian) { where("tags.#{Tag.topic_count_column(guardian)} > 0") }

  scope :base_tags, -> { where(target_tag_id: nil) }
  scope :visible, ->(guardian = nil) { merge(DiscourseTagging.visible_tags(guardian)) }

  has_many :tag_users, dependent: :destroy # notification settings

  has_many :topic_tags, dependent: :destroy
  has_many :topics, through: :topic_tags

  has_many :category_tag_stats, dependent: :destroy
  has_many :category_tags, dependent: :destroy
  has_many :categories, through: :category_tags

  has_many :tag_group_memberships, dependent: :destroy
  has_many :tag_groups, through: :tag_group_memberships

  belongs_to :target_tag, class_name: "Tag", optional: true
  has_many :synonyms, class_name: "Tag", foreign_key: "target_tag_id", dependent: :destroy
  has_many :sidebar_section_links, as: :linkable, dependent: :delete_all

  has_many :embeddable_host_tags
  has_many :embeddable_hosts, through: :embeddable_host_tags

  before_save :sanitize_description

  after_save :index_search
  after_save :update_synonym_associations

  after_commit :trigger_tag_created_event, on: :create
  after_commit :trigger_tag_updated_event, on: :update
  after_commit :trigger_tag_destroyed_event, on: :destroy

### self

we add 1 to max_tags_in_filter_list to efficiently know we have more tags
    # than the limit. Frontend is responsible to enforce limit.
    limit = limit_arg || (SiteSetting.max_tags_in_filter_list + 1)
    scope_category_ids = guardian.allowed_category_ids
    scope_category_ids &= ([category.id] + category.subcategories.pluck(:id)) if category

    return [] if scope_category_ids.empty?

    filter_sql =
      (
        if guardian.is_staff?
          ""
        else
          " AND tags.id IN (#{DiscourseTagging.visible_tags(guardian).select(:id).to_sql})"
        end
      )

    tag_names_with_counts = DB.query <<~SQL
      SELECT tags.name as tag_name, SUM(stats.topic_count) AS sum_topic_count
        FROM category_tag_stats stats
        JOIN tags ON stats.tag_id = tags.id AND stats.topic_count > 0
       WHERE stats.category_id in (#{scope_category_ids.join(",")})
       #{filter_sql}
    GROUP BY tags.name
    ORDER BY sum_topic_count DESC, tag_name ASC
       LIMIT #{limit}
    SQL

    tag_names_with_counts.map { |row| row.tag_name }
  end

