# Category

## Description

frozen_string_literal: true

## Relationships

- belongs_to :topic
- belongs_to :topic_only_relative_url
- belongs_to :user
- belongs_to :latest_post
- belongs_to :uploaded_logo
- belongs_to :uploaded_logo_dark
- belongs_to :uploaded_background
- belongs_to :uploaded_background_dark
- has_many :topics
- has_many :category_users
- has_many :category_featured_topics
- has_many :featured_topics
- has_many :category_groups
- has_many :category_moderation_groups
- has_many :groups
- has_many :moderating_groups
- has_many :topic_timers
- has_many :upload_references
- has_one :category_setting
- belongs_to :parent_category
- has_many :subcategories
- has_many :category_tags
- has_many :tags
- has_many :none_synonym_tags
- has_many :category_tag_groups
- has_many :tag_groups
- has_many :category_required_tag_groups
- has_many :sidebar_section_links
- has_many :embeddable_hosts
- has_many :category_form_templates
- has_many :form_templates

## Methods

### self

frozen_string_literal: true

class Category < ActiveRecord::Base
  RESERVED_SLUGS = ["none"]

  self.ignored_columns = [
    :suppress_from_latest, # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
    :required_tag_group_id, # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
    :min_tags_from_required_group, # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
    :reviewable_by_group_id,
  ]

  include Searchable
  include Positionable
  include HasCustomFields
  include CategoryHashtag
  include AnonCacheInvalidator
  include HasDestroyedWebHook

  SLUG_REF_SEPARATOR = ":"

  belongs_to :topic
  belongs_to :topic_only_relative_url,
             -> { select "id, title, slug" },
             class_name: "Topic",
             foreign_key: "topic_id"

  belongs_to :user
  belongs_to :latest_post, class_name: "Post"
  belongs_to :uploaded_logo, class_name: "Upload"
  belongs_to :uploaded_logo_dark, class_name: "Upload"
  belongs_to :uploaded_background, class_name: "Upload"
  belongs_to :uploaded_background_dark, class_name: "Upload"

  has_many :topics
  has_many :category_users
  has_many :category_featured_topics
  has_many :featured_topics, through: :category_featured_topics, source: :topic

  has_many :category_groups, dependent: :destroy
  has_many :category_moderation_groups, dependent: :destroy
  has_many :groups, through: :category_groups
  has_many :moderating_groups, through: :category_moderation_groups, source: :group
  has_many :topic_timers, dependent: :destroy
  has_many :upload_references, as: :target, dependent: :destroy

  has_one :category_setting, dependent: :destroy

  delegate :auto_bump_cooldown_days,
           :num_auto_bump_daily,
           :num_auto_bump_daily=,
           :require_reply_approval,
           :require_reply_approval=,
           :require_reply_approval?,
           :require_topic_approval,
           :require_topic_approval=,
           :require_topic_approval?,
           to: :category_setting,
           allow_nil: true

  has_and_belongs_to_many :web_hooks

  accepts_nested_attributes_for :category_setting, update_only: true

  validates :user_id, presence: true

  validates :name,
            if:
              Proc.new { |c|
                c.new_record? || c.will_save_change_to_name? ||
                  c.will_save_change_to_parent_category_id?
              },
            presence: true,
            uniqueness: {
              scope: :parent_category_id,
              case_sensitive: false,
            },
            length: {
              in: 1..50,
            }

  validates :num_featured_topics, numericality: { only_integer: true, greater_than: 0 }
  validates :search_priority, inclusion: { in: Searchable::PRIORITIES.values }

  validate :parent_category_validator
  validate :email_in_validator
  validate :ensure_slug
  validate :permissions_compatibility_validator

  validates :default_slow_mode_seconds,
            numericality: {
              only_integer: true,
              greater_than: 0,
            },
            allow_nil: true
  validates :auto_close_hours,
            numericality: {
              greater_than: 0,
              less_than_or_equal_to: 87_600,
            },
            allow_nil: true
  validates :slug, exclusion: { in: RESERVED_SLUGS }

  after_create :create_category_definition
  after_destroy :trash_category_definition
  after_destroy :clear_related_site_settings

  before_save :apply_permissions
  before_save :downcase_email
  before_save :downcase_name
  before_save :ensure_category_setting

  after_save :reset_topic_ids_cache
  after_save :clear_subcategory_ids
  after_save :clear_url_cache
  after_save :publish_discourse_stylesheet
  after_save :publish_category

  after_save do
    if saved_change_to_uploaded_logo_id? || saved_change_to_uploaded_logo_dark_id? ||
         saved_change_to_uploaded_background_id? || saved_change_to_uploaded_background_dark_id?
      upload_ids = [
        self.uploaded_logo_id,
        self.uploaded_logo_dark_id,
        self.uploaded_background_id,
        self.uploaded_background_dark_id,
      ]
      UploadReference.ensure_exist!(upload_ids: upload_ids, target: self)
    end
  end

  after_destroy :reset_topic_ids_cache
  after_destroy :clear_subcategory_ids
  after_destroy :publish_category_deletion
  after_destroy :remove_site_settings

  after_create :delete_category_permalink

  after_update :rename_category_definition, if: :saved_change_to_name?
  after_update :create_category_permalink, if: :saved_change_to_slug?

  after_commit :trigger_category_created_event, on: :create
  after_commit :trigger_category_updated_event, on: :update
  after_commit :trigger_category_destroyed_event, on: :destroy
  after_commit :clear_site_cache

  after_save_commit :index_search

  belongs_to :parent_category, class_name: "Category"
  has_many :subcategories, class_name: "Category", foreign_key: "parent_category_id"

  has_many :category_tags, dependent: :destroy
  has_many :tags, through: :category_tags
  has_many :none_synonym_tags,
           -> { where(target_tag_id: nil) },
           through: :category_tags,
           source: :tag
  has_many :category_tag_groups, dependent: :destroy
  has_many :tag_groups, through: :category_tag_groups

  has_many :category_required_tag_groups, -> { order(order: :asc) }, dependent: :destroy
  has_many :sidebar_section_links, as: :linkable, dependent: :delete_all
  has_many :embeddable_hosts, dependent: :destroy

  has_many :category_form_templates, dependent: :destroy
  has_many :form_templates, through: :category_form_templates

  scope :latest, -> { order("topic_count DESC") }

  scope :secured,
        ->(guardian = nil) do
          ids = guardian.secure_category_ids if guardian

          if ids.present?
            where(
              "NOT categories.read_restricted OR categories.id IN (:cats)",
              cats: ids,
            ).references(:categories)
          else
            where("NOT categories.read_restricted").references(:categories)
          end
        end

  TOPIC_CREATION_PERMISSIONS = [:full]
  POST_CREATION_PERMISSIONS = %i[create_post full]

  scope :topic_create_allowed,
        ->(guardian) do
          scoped = scoped_to_permissions(guardian, TOPIC_CREATION_PERMISSIONS)

          if !SiteSetting.allow_uncategorized_topics && !guardian.is_staff?
            scoped = scoped.where.not(id: SiteSetting.uncategorized_category_id)
          end

          scoped
        end

  scope :post_create_allowed,
        ->(guardian) { scoped_to_permissions(guardian, POST_CREATION_PERMISSIONS) }

  scope :with_ancestors, ->(id) { where(<<~SQL, id) }
        id IN (
          WITH RECURSIVE ancestors(category_id) AS (
            SELECT ?
            UNION
            SELECT parent_category_id
            FROM categories, ancestors
            WHERE id = ancestors.category_id
          )
          SELECT category_id FROM ancestors
        )
      SQL

  scope :with_parents, ->(ids) { where(<<~SQL, ids: ids) }
    id IN (:ids)
    OR
    id IN (SELECT DISTINCT parent_category_id FROM categories WHERE id IN (:ids))
  SQL

  delegate :post_template, to: "self.class"

  # permission is just used by serialization
  # we may consider wrapping this in another spot
  attr_accessor :displayable_topics,
                :permission,
                :subcategory_ids,
                :subcategory_list,
                :notification_level,
                :has_children,
                :subcategory_count

  # Allows us to skip creating the category definition topic in tests.
  attr_accessor :skip_category_definition

  enum :style_type, { square: 0, icon: 1, emoji: 2 }

### self

Load notification levels
    notification_levels = CategoryUser.notification_levels_for(guardian.user)
    notification_levels.default = CategoryUser.default_notification_level

    # Load permissions
    allowed_topic_create_ids =
      if !guardian.is_admin? && !guardian.is_anonymous?
        Category.topic_create_allowed(guardian).where(id: category_ids).pluck(:id).to_set
      end

    # Load subcategory counts (used to fill has_children property)
    subcategory_count =
      Category.secured(guardian).where.not(parent_category_id: nil).group(:parent_category_id).count

    # Update category attributes
    categories.each do |category|
      category.notification_level = notification_levels[category[:id]]

      category.permission = CategoryGroup.permission_types[:full] if guardian.is_admin? ||
        allowed_topic_create_ids&.include?(category[:id])

      category.has_children = subcategory_count.key?(category[:id])

      category.subcategory_count = subcategory_count[category[:id]] if category.has_children
    end
  end

### self

Perform a search. If a category exists in the result, its ancestors do too.
  # Also check for prefix matches. If a category has a prefix match, its
  # ancestors report a match too.
  scope :tree_search,
        ->(only, except, term) do
          term = term.strip
          escaped_term = ActiveRecord::Base.connection.quote(term.downcase)
          prefix_match = "starts_with(LOWER(categories.name), #{escaped_term})"

          word_match = <<~SQL
            COALESCE(
              (
                SELECT BOOL_AND(position(pattern IN LOWER(categories.name)) <> 0)
                FROM unnest(regexp_split_to_array(#{escaped_term}, '\s+')) AS pattern
              ),
              true
            )
          SQL

          if except
            prefix_match =
              "NOT categories.id IN (#{except.reselect(:id).to_sql}) AND #{prefix_match}"
            word_match = "NOT categories.id IN (#{except.reselect(:id).to_sql}) AND #{word_match}"
          end

          if only
            prefix_match = "categories.id IN (#{only.reselect(:id).to_sql}) AND #{prefix_match}"
            word_match = "categories.id IN (#{only.reselect(:id).to_sql}) AND #{word_match}"
          end

          categories =
            Category.select(
              "categories.*",
              "#{prefix_match} AS has_prefix_match",
              "#{word_match} AS has_word_match",
            )

          (1...SiteSetting.max_category_nesting).each do
            categories = Category.from("(#{categories.to_sql}) AS categories")

            subcategory_matches =
              categories
                .where.not(parent_category_id: nil)
                .group("categories.parent_category_id")
                .select(
                  "categories.parent_category_id AS id",
                  "BOOL_OR(categories.has_prefix_match) AS has_prefix_match",
                  "BOOL_OR(categories.has_word_match) AS has_word_match",
                )

            categories =
              Category.joins(
                "LEFT JOIN (#{subcategory_matches.to_sql}) AS subcategory_matches ON categories.id = subcategory_matches.id",
              ).select(
                "categories.*",
                "#{prefix_match} OR COALESCE(subcategory_matches.has_prefix_match, false) AS has_prefix_match",
                "#{word_match} OR COALESCE(subcategory_matches.has_word_match, false) AS has_word_match",
              )
          end

          categories =
            Category.from("(#{categories.to_sql}) AS categories").where(has_word_match: true)

          categories.select("has_prefix_match AS matches", :id)
        end

  # Given a relation, 'matches', which contains category ids and a 'matches'
  # boolean, and a limit (the maximum number of subcategories per category),
  # produce a subset of the matches categories annotated with information about
  # their ancestors.
  scope :select_descendants,
        ->(matches, limit) do
          max_nesting = SiteSetting.max_category_nesting

          categories =
            joins("INNER JOIN (#{matches.to_sql}) AS matches ON matches.id = categories.id").select(
              "categories.id",
              "categories.name",
              "ARRAY[]::record[] AS ancestors",
              "0 AS depth",
              "matches.matches",
            )

          categories = Category.from("(#{categories.to_sql}) AS c1")

          (1...max_nesting).each { |i| categories = categories.joins(<<~SQL) }
            INNER JOIN LATERAL (
              (SELECT c#{i}.id, c#{i}.name, c#{i}.ancestors, c#{i}.depth, c#{i}.matches)
              UNION ALL
              (SELECT
                categories.id,
                categories.name,
                c#{i}.ancestors || ARRAY[ROW(NOT c#{i}.matches, c#{i}.name)] AS ancestors,
                c#{i}.depth + 1 as depth,
                matches.matches
              FROM categories
              INNER JOIN matches
              ON matches.id = categories.id
              WHERE categories.parent_category_id = c#{i}.id
              AND c#{i}.depth = #{i - 1}
              ORDER BY (NOT matches.matches, categories.name)
              LIMIT #{limit})
            ) c#{i + 1} ON true
          SQL

          categories.select(
            "c#{max_nesting}.id",
            "c#{max_nesting}.ancestors",
            "c#{max_nesting}.name",
            "c#{max_nesting}.matches",
          )
        end

  scope :limited_categories_matching,
        ->(only, except, parent_id, term) do
          joins(<<~SQL).order("c.ancestors || ARRAY[ROW(NOT c.matches, c.name)]")
            INNER JOIN (
              WITH matches AS (#{Category.tree_search(only, except, term).to_sql})
              #{Category.where(parent_category_id: parent_id).select_descendants(Category.from("matches").select(:matches, :id), 5).to_sql}
            ) AS c
            ON categories.id = c.id
          SQL
        end

### self

Accepts an array of slugs with each item in the array
  # Returns the category ids of the last slug in the array. The slugs array has to follow the proper category
  # nesting hierarchy. If any of the slug in the array is invalid or if the slugs array does not follow the proper
  # category nesting hierarchy, nil is returned.
  #
  # When only a single slug is provided, the category id of all the categories with that slug is returned.

### visible_posts

Yes, there are a lot of queries happening below.
    # Performing a lot of queries is actually faster than using one big update
    # statement with sub-selects on large databases with many categories,
    # topics, and posts.
    #
    # The old method with the one query is here:
    # https://github.com/discourse/discourse/blob/5f34a621b5416a53a2e79a145e927fca7d5471e8/app/models/category.rb
    #
    # If you refactor this, test performance on a large database.

    Category.all.each do |c|
      topics = c.topics.visible
      topics = topics.where(["topics.id <> ?", c.topic_id]) if c.topic_id
      c.topics_year = topics.created_since(1.year.ago).count
      c.topics_month = topics.created_since(1.month.ago).count
      c.topics_week = topics.created_since(1.week.ago).count
      c.topics_day = topics.created_since(1.day.ago).count

      posts = c.visible_posts
      c.posts_year = posts.created_since(1.year.ago).count
      c.posts_month = posts.created_since(1.month.ago).count
      c.posts_week = posts.created_since(1.week.ago).count
      c.posts_day = posts.created_since(1.day.ago).count

      c.save if c.changed?
    end
  end

### self

Internal: Generate the text of post prompting to enter category description.

### slug_for_url

if we don't unescape it first we strip the % from the encoded version
      slug = SiteSetting.slug_generation_method == "encoded" ? CGI.unescape(self.slug) : self.slug
      self.slug = Slug.for(slug, "", method: :encoded)

      if self.slug.blank?
        errors.add(:slug, :invalid)
      elsif SiteSetting.slug_generation_method == "ascii" && !CGI.unescape(self.slug).ascii_only?
        errors.add(:slug, I18n.t("category.errors.slug_contains_non_ascii_chars"))
      elsif duplicate_slug?
        errors.add(:slug, I18n.t("category.errors.is_already_in_use"))
      end
    else
      # auto slug
      self.slug = Slug.for(name, "")
      self.slug = "" if duplicate_slug?
    end

    # only allow to use category itself id.
    match_id = /\A(\d+)-category/.match(self.slug)
    if match_id.present?
      errors.add(:slug, :invalid) if new_record? || (match_id[1] != self.id.to_s)
    end
  end

### height_of_ancestors

This is used in a validation so has to produce accurate results before the
  # record has been saved

### depth_of_descendants

This is used in a validation so has to produce accurate results before the
  # record has been saved

### set_permissions

this line bothers me, destroying in AR can not seem to be queued, thinking of extending it
    category_groups.destroy_all unless new_record?
    ids = Group.where(name: names.split(",")).pluck(:id)
    ids.each { |id| category_groups.build(group_id: id) }
  end

  # will reset permission on a topic to a particular
  # set.
  #
  # Available permissions are, :full, :create_post, :readonly
  #   hash can be:
  #
  # :everyone => :full - everyone has everything
  # :everyone => :readonly, :staff => :full
  # 7 => 1  # you can pass a group_id and permission id

### permissions

Ideally we can just call .clear here, but it runs SQL, we only want to run it
    # on save.
  end

### auto_bump_topic

will automatically bump a single topic
  # if number of automatically bumped topics is smaller than threshold

### rename_category_definition

If the name changes, try and update the category definition topic too if it's an exact match

### self

when saving subcategories
    if @permissions && parent_category_id.present?
      return if parent_category.category_groups.empty?

      parent_permissions = parent_category.category_groups.pluck(:group_id, :permission_type)
      child_permissions =
        (
          if @permissions.empty?
            [[Group[:everyone].id, CategoryGroup.permission_types[:full]]]
          else
            @permissions
          end
        )
      check_permissions_compatibility(parent_permissions, child_permissions)

      # when saving parent category
    elsif @permissions && subcategories.present?
      return if @permissions.empty?

      parent_permissions = @permissions
      child_permissions = subcategories_permissions.uniq

      check_permissions_compatibility(parent_permissions, child_permissions)
    end
  end

### has_restricted_tags

This is a weird case, probably indicating a bug.
        I18n.t("category.cannot_delete.topic_exists_no_oldest", count: self.topic_count)
      end
    end
  end

