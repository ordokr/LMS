# Topic

## Description

frozen_string_literal: true

## Relationships

- belongs_to :category
- has_many :category_users
- has_many :posts
- has_many :bookmarks
- has_many :ordered_posts
- has_many :topic_allowed_users
- has_many :topic_allowed_groups
- has_many :incoming_email
- has_many :group_archived_messages
- has_many :user_archived_messages
- has_many :topic_view_stats
- has_many :allowed_groups
- has_many :allowed_group_users
- has_many :allowed_users
- has_many :topic_tags
- has_many :tags
- has_many :tag_users
- has_many :moved_posts_as_old_topic
- has_many :moved_posts_as_new_topic
- has_one :top_topic
- has_one :topic_hot_score
- has_one :shared_draft
- has_one :published_page
- belongs_to :user
- belongs_to :last_poster
- belongs_to :featured_user1
- belongs_to :featured_user2
- belongs_to :featured_user3
- belongs_to :featured_user4
- has_many :topic_users
- has_many :dismissed_topic_users
- has_many :topic_links
- has_many :topic_invites
- has_many :invites
- has_many :topic_timers
- has_many :reviewables
- has_many :user_profiles
- has_one :user_warning
- has_one :first_post
- has_one :topic_search_data
- has_one :topic_embed
- has_one :linked_topic
- belongs_to :image_upload
- has_many :topic_thumbnails

## Methods

### self

frozen_string_literal: true

class Topic < ActiveRecord::Base
  class UserExists < StandardError
  end

  class NotAllowed < StandardError
  end
  include RateLimiter::OnCreateRecord
  include HasCustomFields
  include Trashable
  include Searchable
  include LimitedEdit
  extend Forwardable

  EXTERNAL_ID_MAX_LENGTH = 50

  self.ignored_columns = [
    "avg_time", # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
    "image_url", # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
  ]

  def_delegator :featured_users, :user_ids, :featured_user_ids
  def_delegator :featured_users, :choose, :feature_topic_users

  def_delegator :notifier, :watch!, :notify_watch!
  def_delegator :notifier, :track!, :notify_tracking!
  def_delegator :notifier, :regular!, :notify_regular!
  def_delegator :notifier, :mute!, :notify_muted!
  def_delegator :notifier, :toggle_mute, :toggle_mute

  attr_accessor :allowed_user_ids, :allowed_group_ids, :tags_changed, :includes_destination_category

### initialize_default_values

Note parens are required because superclass doesn't take `recovered_by`
    super()

    DiscourseEvent.trigger(:topic_recovered, self) if trigger_event

    unless (topic_embed = TopicEmbed.with_deleted.find_by_topic_id(id)).nil?
      topic_embed.recover!
    end
  end

  rate_limit :default_rate_limiter
  rate_limit :limit_topics_per_day
  rate_limit :limit_private_messages_per_day

  validates :title,
            if: Proc.new { |t| t.new_record? || t.title_changed? || t.category_id_changed? },
            presence: true,
            topic_title_length: true,
            censored_words: true,
            watched_words: true,
            quality_title: {
              unless: :private_message?,
            },
            max_emojis: true,
            unique_among: {
              unless:
                Proc.new { |t| (SiteSetting.allow_duplicate_topic_titles? || t.private_message?) },
              message: :has_already_been_used,
              allow_blank: true,
              case_sensitive: false,
              collection:
                Proc.new { |t|
                  if SiteSetting.allow_duplicate_topic_titles_category?
                    Topic.listable_topics.where("category_id = ?", t.category_id)
                  else
                    Topic.listable_topics
                  end
                },
            }

  validates :category_id,
            presence: true,
            exclusion: {
              in: Proc.new { [SiteSetting.uncategorized_category_id] },
            },
            if:
              Proc.new { |t|
                (t.new_record? || t.category_id_changed?) &&
                  !SiteSetting.allow_uncategorized_topics && (t.archetype.nil? || t.regular?)
              }

  validates :featured_link, allow_nil: true, url: true
  validate if: :featured_link do
    if featured_link_changed? && !Guardian.new(user).can_edit_featured_link?(category_id)
      errors.add(:featured_link)
    end
  end

  validates :external_id,
            allow_nil: true,
            uniqueness: {
              case_sensitive: false,
            },
            length: {
              maximum: EXTERNAL_ID_MAX_LENGTH,
            },
            format: {
              with: /\A[\w-]+\z/,
            }

  before_validation do
    self.title = TextCleaner.clean_title(TextSentinel.title_sentinel(title).text) if errors[
      :title
    ].empty?
    self.featured_link = self.featured_link.strip.presence if self.featured_link
  end

  belongs_to :category
  has_many :category_users, through: :category
  has_many :posts

  # NOTE: To get all Post _and_ Topic bookmarks for a topic by user,
  # use the Bookmark.for_user_in_topic scope.
  has_many :bookmarks, as: :bookmarkable

  has_many :ordered_posts, -> { order(post_number: :asc) }, class_name: "Post"
  has_many :topic_allowed_users
  has_many :topic_allowed_groups
  has_many :incoming_email

  has_many :group_archived_messages, dependent: :destroy
  has_many :user_archived_messages, dependent: :destroy
  has_many :topic_view_stats, dependent: :destroy

  has_many :allowed_groups, through: :topic_allowed_groups, source: :group
  has_many :allowed_group_users, through: :allowed_groups, source: :users
  has_many :allowed_users, through: :topic_allowed_users, source: :user

  has_many :topic_tags
  has_many :tags, through: :topic_tags, dependent: :destroy # dependent destroy applies to the topic_tags records
  has_many :tag_users, through: :tags

  has_many :moved_posts_as_old_topic,
           class_name: "MovedPost",
           foreign_key: :old_topic_id,
           dependent: :destroy
  has_many :moved_posts_as_new_topic,
           class_name: "MovedPost",
           foreign_key: :new_topic_id,
           dependent: :destroy

  has_one :top_topic
  has_one :topic_hot_score
  has_one :shared_draft, dependent: :destroy
  has_one :published_page

  belongs_to :user
  belongs_to :last_poster, class_name: "User", foreign_key: :last_post_user_id
  belongs_to :featured_user1, class_name: "User", foreign_key: :featured_user1_id
  belongs_to :featured_user2, class_name: "User", foreign_key: :featured_user2_id
  belongs_to :featured_user3, class_name: "User", foreign_key: :featured_user3_id
  belongs_to :featured_user4, class_name: "User", foreign_key: :featured_user4_id

  has_many :topic_users
  has_many :dismissed_topic_users
  has_many :topic_links
  has_many :topic_invites
  has_many :invites, through: :topic_invites, source: :invite
  has_many :topic_timers, dependent: :destroy
  has_many :reviewables
  has_many :user_profiles

  has_one :user_warning
  has_one :first_post, -> { where post_number: 1 }, class_name: "Post"
  has_one :topic_search_data
  has_one :topic_embed, dependent: :destroy
  has_one :linked_topic, dependent: :destroy

  belongs_to :image_upload, class_name: "Upload"
  has_many :topic_thumbnails, through: :image_upload

  # When we want to temporarily attach some data to a forum topic (usually before serialization)
  attr_accessor :user_data
  attr_accessor :category_user_data
  attr_accessor :dismissed

  attr_accessor :posters # TODO: can replace with posters_summary once we remove old list code
  attr_accessor :participants
  attr_accessor :participant_groups
  attr_accessor :topic_list
  attr_accessor :include_last_poster
  attr_accessor :import_mode # set to true to optimize creation and save for imports

  # The regular order
  scope :topic_list_order, -> { order("topics.bumped_at desc") }

  # Return private message topics
  scope :private_messages, -> { where(archetype: Archetype.private_message) }

  PRIVATE_MESSAGES_SQL_USER = <<~SQL
    SELECT topic_id
    FROM topic_allowed_users
    WHERE user_id = :user_id
  SQL

  PRIVATE_MESSAGES_SQL_GROUP = <<~SQL
    SELECT tg.topic_id
    FROM topic_allowed_groups tg
    JOIN group_users gu ON gu.user_id = :user_id AND gu.group_id = tg.group_id
  SQL

  scope :private_messages_for_user,
        ->(user) do
          private_messages.where(
            "topics.id IN (#{PRIVATE_MESSAGES_SQL_USER})
      OR topics.id IN (#{PRIVATE_MESSAGES_SQL_GROUP})",
            user_id: user.id,
          )
        end

  scope :listable_topics, -> { where("topics.archetype <> ?", Archetype.private_message) }

  scope :by_newest, -> { order("topics.created_at desc, topics.id desc") }

  scope :visible, -> { where(visible: true) }

  scope :created_since, lambda { |time_ago| where("topics.created_at > ?", time_ago) }

  scope :exclude_scheduled_bump_topics, -> { where.not(id: TopicTimer.scheduled_bump_topics) }

  scope :secured,
        lambda { |guardian = nil|
          ids = guardian.secure_category_ids if guardian

          # Query conditions
          condition =
            if ids.present?
              ["NOT read_restricted OR id IN (:cats)", cats: ids]
            else
              ["NOT read_restricted"]
            end

          where(
            "topics.category_id IS NULL OR topics.category_id IN (SELECT id FROM categories WHERE #{condition[0]})",
            condition[1],
          )
        }

  scope :in_category_and_subcategories,
        lambda { |category_id|
          if category_id
            where("topics.category_id IN (?)", Category.subcategory_ids(category_id.to_i))
          end
        }

  scope :with_subtype, ->(subtype) { where("topics.subtype = ?", subtype) }

  attr_accessor :ignore_category_auto_close
  attr_accessor :skip_callbacks
  attr_accessor :advance_draft

  before_create { initialize_default_values }

  after_create do
    unless skip_callbacks
      changed_to_category(category)
      advance_draft_sequence if advance_draft
    end
  end

  before_save do
    ensure_topic_has_a_category unless skip_callbacks

    write_attribute(:fancy_title, Topic.fancy_title(title)) if title_changed?

    if category_id_changed? || new_record?
      inherit_auto_close_from_category
      inherit_slow_mode_from_category
    end
  end

  after_save do
    banner = "banner"

    if archetype_before_last_save == banner || archetype == banner
      ApplicationLayoutPreloader.banner_json_cache.clear
    end

    if tags_changed || saved_change_to_attribute?(:category_id) ||
         saved_change_to_attribute?(:title)
      SearchIndexer.queue_post_reindex(self.id)

      if tags_changed
        TagUser.auto_watch(topic_id: id)
        TagUser.auto_track(topic_id: id)
        self.tags_changed = false
      end
    end

    SearchIndexer.index(self)
  end

  after_update do
    if saved_changes[:category_id] && self.tags.present?
      CategoryTagStat.topic_moved(self, *saved_changes[:category_id])
    elsif saved_changes[:category_id] && self.category&.read_restricted?
      UserProfile.remove_featured_topic_from_all_profiles(self)
    end
  end

### all_allowed_users

all users (in groups or directly targeted) that are going to get the pm

### limit_topics_per_day

Additional rate limits on topics: per day and private messages per day

### self

make sure data is set in table, this also allows us to change algorithm
        # by simply nulling this column
        DB.exec(
          "UPDATE topics SET fancy_title = :fancy_title where id = :id",
          id: self.id,
          fancy_title: fancy_title,
        )
      end
    end

    fancy_title
  end

  # Returns hot topics since a date for display in email digest.

### reload

Remove category topics
    topics = topics.where.not(id: Category.select(:topic_id).where.not(topic_id: nil))

    # Remove suppressed categories
    if SiteSetting.digest_suppress_categories.present?
      topics =
        topics.where.not(category_id: SiteSetting.digest_suppress_categories.split("|").map(&:to_i))
    end

    # Remove suppressed tags
    if SiteSetting.digest_suppress_tags.present?
      tag_ids = Tag.where_name(SiteSetting.digest_suppress_tags.split("|")).pluck(:id)

      topics =
        topics.where.not(id: TopicTag.where(tag_id: tag_ids).select(:topic_id)) if tag_ids.present?
    end

    # Remove muted and shared draft categories
    remove_category_ids =
      CategoryUser.where(
        user_id: user.id,
        notification_level: CategoryUser.notification_levels[:muted],
      ).pluck(:category_id)

    remove_category_ids << SiteSetting.shared_drafts_category if SiteSetting.shared_drafts_enabled?

    if remove_category_ids.present?
      remove_category_ids.uniq!
      topics =
        topics.where(
          "topic_users.notification_level != ? OR topics.category_id NOT IN (?)",
          TopicUser.notification_levels[:muted],
          remove_category_ids,
        )
    end

    # Remove muted tags
    muted_tag_ids = TagUser.lookup(user, :muted).pluck(:tag_id)
    unless muted_tag_ids.empty?
      # If multiple tags per topic, include topics with tags that aren't muted,
      # and don't forget untagged topics.
      topics =
        topics.where(
          "EXISTS ( SELECT 1 FROM topic_tags WHERE topic_tags.topic_id = topics.id AND tag_id NOT IN (?) )
        OR NOT EXISTS (SELECT 1 FROM topic_tags WHERE topic_tags.topic_id = topics.id)",
          muted_tag_ids,
        )
    end

    topics
  end

### update_status

{excluded_category_ids_sql}
      UNION
      #{CategoryUser.muted_category_ids_query(user, include_direct: true).select("categories.id").to_sql}
      SQL

    candidates =
      Topic
        .visible
        .listable_topics
        .secured(guardian)
        .joins("JOIN topic_search_data s ON topics.id = s.topic_id")
        .joins("LEFT JOIN categories c ON topics.id = c.topic_id")
        .where("search_data @@ #{tsquery}")
        .where("c.topic_id IS NULL")
        .where("topics.category_id NOT IN (#{excluded_category_ids_sql})")
        .order("ts_rank(search_data, #{tsquery}) DESC")
        .limit(SiteSetting.max_similar_results * 3)

    candidate_ids = candidates.pluck(:id)

    return [] if candidate_ids.blank?

    similars =
      Topic
        .joins("JOIN posts AS p ON p.topic_id = topics.id AND p.post_number = 1")
        .where("topics.id IN (?)", candidate_ids)
        .order("similarity DESC")
        .limit(SiteSetting.max_similar_results)

    if raw.present?
      similars.select(
        DB.sql_fragment(
          "topics.*, similarity(topics.title, :title) + similarity(p.raw, :raw) AS similarity, p.cooked AS blurb",
          title: title,
          raw: raw,
        ),
      ).where(
        "similarity(topics.title, :title) + similarity(p.raw, :raw) > 0.2",
        title: title,
        raw: raw,
      )
    else
      similars.select(
        DB.sql_fragment(
          "topics.*, similarity(topics.title, :title) AS similarity, p.cooked AS blurb",
          title: title,
        ),
      ).where("similarity(topics.title, :title) > 0.2", title: title)
    end
  end

### self

Atomically creates the next post number

### self

{reply_sql}
            #{posts_sql}
        WHERE id = :topic_id
        RETURNING highest_post_number
      SQL

      result.first.to_i
    end
  end

### self

If a post is deleted we have to update our highest post counters and last post information

### changed_to_category

ignore small_action replies for private messages
    post_type =
      archetype == Archetype.private_message ? " AND post_type <> #{Post.types[:small_action]}" : ""

    result = DB.query_single(<<~SQL, topic_id: topic_id)
      UPDATE topics
      SET
        highest_staff_post_number = (
          SELECT COALESCE(MAX(post_number), 0) FROM posts
          WHERE topic_id = :topic_id AND
                deleted_at IS NULL
        ),
        highest_post_number = (
          SELECT COALESCE(MAX(post_number), 0) FROM posts
          WHERE topic_id = :topic_id AND
                deleted_at IS NULL AND
                post_type <> 4
                #{post_type}
        ),
        posts_count = (
          SELECT count(*) FROM posts
          WHERE deleted_at IS NULL AND
                topic_id = :topic_id AND
                post_type <> 4
                #{post_type}
        ),
        word_count = (
          SELECT SUM(COALESCE(posts.word_count, 0)) FROM posts
          WHERE topic_id = :topic_id AND
                deleted_at IS NULL AND
                post_type <> 4
                #{post_type}
        ),
        last_posted_at = (
          SELECT MAX(created_at) FROM posts
          WHERE topic_id = :topic_id AND
                deleted_at IS NULL AND
                post_type <> 4
                #{post_type}
        ),
        last_post_user_id = COALESCE((
          SELECT user_id FROM posts
          WHERE topic_id = :topic_id AND
                deleted_at IS NULL AND
                post_type <> 4
                #{post_type}
          ORDER BY created_at desc
          LIMIT 1
        ), last_post_user_id)
      WHERE id = :topic_id
      RETURNING highest_post_number
    SQL

    highest_post_number = result.first.to_i

    # Update the forum topic user records
    DB.exec(<<~SQL, highest: highest_post_number, topic_id: topic_id)
      UPDATE topic_users
      SET last_read_post_number = CASE
                                  WHEN last_read_post_number > :highest THEN :highest
                                  ELSE last_read_post_number
                                  END
      WHERE topic_id = :topic_id
    SQL
  end

  cattr_accessor :update_featured_topics

### add_small_action

when a topic changes category we may have to start watching it
        # if we happen to have read state for it
        CategoryUser.auto_watch(category_id: new_category.id, topic_id: self.id)
        CategoryUser.auto_track(category_id: new_category.id, topic_id: self.id)

        if !SiteSetting.disable_category_edit_notifications && (post = self.ordered_posts.first)
          notified_user_ids = [post.user_id, post.last_editor_id].uniq
          DB.after_commit do
            Jobs.enqueue(
              :notify_category_change,
              post_id: post.id,
              notified_user_ids: notified_user_ids,
            )
          end
        end

        # when a topic changes category we may need to make uploads
        # linked to posts secure/not secure depending on whether the
        # category is private. this is only done if the category
        # has actually changed to avoid noise.
        DB.after_commit { Jobs.enqueue(:update_topic_upload_security, topic_id: self.id) }
      end

      Category.where(id: new_category.id).update_all("topic_count = topic_count + 1")

      if Topic.update_featured_topics != false
        CategoryFeaturedTopic.feature_topics_for(old_category) unless @import_mode
        unless @import_mode || old_category.try(:id) == new_category.id
          CategoryFeaturedTopic.feature_topics_for(new_category)
        end
      end
    end

    true
  end

### change_category_to_id

If we are moving posts, we want to insert the moderator post where the previous posts were
      # in the stream, not at the end.
      if opts[:post_number].present?
        new_post.update!(post_number: opts[:post_number], sort_order: opts[:post_number])
      end

      # Grab any links that are present
      TopicLink.extract_from(new_post)
      QuotedPost.extract_from(new_post)
    end

    new_post
  end

### remove_allowed_group

if the category name is blank, reset the attribute
    new_category_id = SiteSetting.uncategorized_category_id if new_category_id == 0

    return true if self.category_id == new_category_id

    cat = Category.find_by(id: new_category_id)
    return false unless cat

    reviewables.update_all(category_id: new_category_id)

    changed_to_category(cat)
  end

### invite

If the group invited includes the OP of the topic as one of is members,
    # we cannot strip the topic_allowed_user record since it will be more
    # complicated to recover the topic_allowed_user record for the OP if the
    # group is removed.
    allowed_user_where_clause = <<~SQL
      users.id IN (
        SELECT topic_allowed_users.user_id
        FROM topic_allowed_users
        INNER JOIN group_users ON group_users.user_id = topic_allowed_users.user_id
        INNER JOIN topic_allowed_groups ON topic_allowed_groups.group_id = group_users.group_id
        WHERE topic_allowed_groups.group_id = :group_id AND
              topic_allowed_users.topic_id = :topic_id AND
              topic_allowed_users.user_id != :op_user_id
      )
    SQL
    User
      .where(
        [
          allowed_user_where_clause,
          { group_id: group.id, topic_id: self.id, op_user_id: self.user_id },
        ],
      )
      .find_each { |allowed_user| remove_allowed_user(Discourse.system_user, allowed_user) }

    true
  end

### update_statistics

Updates the denormalized statistics of a topic including featured posters. They shouldn't
  # go out of sync unless you do something drastic live move posts from one topic to another.
  # this recalculates everything.

### remove_banner

only one banner at the same time
    previous_banner = Topic.where(archetype: Archetype.banner).first
    previous_banner.remove_banner!(user) if previous_banner.present?

    UserProfile.where("dismissed_banner_key IS NOT NULL").update_all(dismissed_banner_key: nil)

    self.archetype = Archetype.banner
    self.bannered_until = bannered_until
    self.add_small_action(user, "banner.enabled")
    self.save

    MessageBus.publish("/site/banner", banner)

    Jobs.cancel_scheduled_job(:remove_banner, topic_id: self.id)
    Jobs.enqueue_at(bannered_until, :remove_banner, topic_id: self.id) if bannered_until
  end

### slug

this is a hook for plugins that need to modify the generated slug
    self.class.slug_computed_callbacks.each { |callback| slug = callback.call(self, slug, title) }

    slug
  end

  # Even if the slug column in the database is null, topic.slug will return something:

### last_post_url

NOTE: These are probably better off somewhere else.
  #       Having a model know about URLs seems a bit strange.

### inherit_slow_mode_from_category

unpin topics that might have been missed
    Topic.where("pinned_until < ?", Time.now).update_all(
      pinned_at: nil,
      pinned_globally: false,
      pinned_until: nil,
    )
    Topic
      .where("bannered_until < ?", Time.now)
      .find_each { |topic| topic.remove_banner!(Discourse.system_user) }
  end

### public_topic_timer

the timer time can be a timestamp or an integer based
      # on the number of hours
      auto_close_time = auto_close_hours

      if !based_on_last_post
        # set auto close to the original time it should have been
        # when the topic was first created.
        start_time = self.created_at || Time.zone.now
        auto_close_time = start_time + auto_close_hours.hours

        # if we have already passed the original close time then
        # we should not recreate the auto-close timer for the topic
        return if auto_close_time < Time.zone.now

        # timestamp must be a string for set_or_create_timer
        auto_close_time = auto_close_time.to_s
      end

      self.set_or_create_timer(
        TopicTimer.types[timer_type],
        auto_close_time,
        by_user: Discourse.system_user,
        based_on_last_post: based_on_last_post,
        duration_minutes: duration_minutes,
      )
    end
  end

### set_or_create_timer

Valid arguments for the time:
  #  * An integer, which is the number of hours from now to update the topic's status.
  #  * A timestamp, like "2013-11-25 13:00", when the topic's status should update.
  #  * A timestamp with timezone in JSON format. (e.g., "2013-11-26T21:00:00.000Z")
  #  * `nil` to delete the topic's status update.
  # Options:
  #  * by_user: User who is setting the topic's status update.
  #  * based_on_last_post: True if time should be based on timestamp of the last post.
  #  * category_id: Category that the update will apply to.
  #  * duration_minutes: The duration of the timer in minutes, which is used if the timer is based
  #                      on the last post or if the timer type is delete_replies.
  #  * silent: Affects whether the close topic timer status change will be silent or not.

### read_restricted_category

a timestamp in client's time zone, like "2015-5-27 12:00"
        topic_timer.execute_at = timestamp
      end
    end

    if topic_timer.execute_at
      if by_user&.staff? || by_user&.trust_level == TrustLevel[4]
        topic_timer.user = by_user
      else
        topic_timer.user ||=
          (
            if self.user.staff? || self.user.trust_level == TrustLevel[4]
              self.user
            else
              Discourse.system_user
            end
          )
      end

      if self.persisted?
        # See TopicTimer.after_save for additional context; the topic
        # status may be changed by saving.
        topic_timer.save!
      else
        self.topic_timers << topic_timer
      end

      topic_timer
    end
  end

### self

tricky query but this checks to see if message is archived for ALL groups you belong to
    # OR if you have it archived as a user explicitly

    sql = <<~SQL
      SELECT 1
      WHERE
        (
        SELECT count(*) FROM topic_allowed_groups tg
        JOIN group_archived_messages gm
              ON gm.topic_id = tg.topic_id AND
                 gm.group_id = tg.group_id
          WHERE tg.group_id IN (SELECT g.group_id FROM group_users g WHERE g.user_id = :user_id)
            AND tg.topic_id = :topic_id
        ) =
        (
          SELECT case when count(*) = 0 then -1 else count(*) end FROM topic_allowed_groups tg
          WHERE tg.group_id IN (SELECT g.group_id FROM group_users g WHERE g.user_id = :user_id)
            AND tg.topic_id = :topic_id
        )

        UNION ALL

        SELECT 1 FROM topic_allowed_users tu
        JOIN user_archived_messages um ON um.user_id = tu.user_id AND um.topic_id = tu.topic_id
        WHERE tu.user_id = :user_id AND tu.topic_id = :topic_id
    SQL

    DB.exec(sql, user_id: user.id, topic_id: id) > 0
  end

  TIME_TO_FIRST_RESPONSE_SQL = <<-SQL
    SELECT AVG(t.hours)::float AS "hours", t.created_at AS "date"
    FROM (
      SELECT t.id, t.created_at::date AS created_at, EXTRACT(EPOCH FROM MIN(p.created_at) - t.created_at)::float / 3600.0 AS "hours"
      FROM topics t
      LEFT JOIN posts p ON p.topic_id = t.id
      /*where*/
      GROUP BY t.id
    ) t
    GROUP BY t.created_at
    ORDER BY t.created_at
  SQL

  TIME_TO_FIRST_RESPONSE_TOTAL_SQL = <<-SQL
    SELECT AVG(t.hours)::float AS "hours"
    FROM (
      SELECT t.id, EXTRACT(EPOCH FROM MIN(p.created_at) - t.created_at)::float / 3600.0 AS "hours"
      FROM topics t
      LEFT JOIN posts p ON p.topic_id = t.id
      /*where*/
      GROUP BY t.id
    ) t
  SQL

### create_invite_notification

We only care about the emails addressed to the group or CC'd to the
        # group if the group is present. If combined addresses is empty we do
        # not need to do this check, and instead can proceed on to adding the
        # from address.
        #
        # Will not include test1@gmail.com if the only IncomingEmail
        # is:
        #
        # from: test1@gmail.com
        # to: test+support@discoursemail.com
        #
        # Because we don't care about the from addresses and also the to address
        # is not the email_username, which will be something like test1@gmail.com.
        if group.present? && combined_addresses.any?
          next if combined_addresses.none? { |address| address =~ group.email_username_regex }
        end

        email_addresses.add(incoming_email.from_address)
        email_addresses.merge(combined_addresses)
      end

    email_addresses.subtract([nil, ""])
    email_addresses.delete(group.email_username) if group.present?

    email_addresses.to_a
  end

