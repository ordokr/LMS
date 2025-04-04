# Post

## Description

frozen_string_literal: true
require "archetype"
require "digest/sha1"

## Relationships

- belongs_to :user
- belongs_to :topic
- belongs_to :reply_to_user
- has_many :post_replies
- has_many :replies
- has_many :post_actions
- has_many :topic_links
- has_many :group_mentions
- has_many :upload_references
- has_many :uploads
- has_one :post_stat
- has_many :bookmarks
- has_one :incoming_email
- has_many :post_details
- has_many :post_revisions
- has_many :revisions
- has_many :moved_posts_as_old_post
- has_many :moved_posts_as_new_post
- has_many :user_actions
- belongs_to :image_upload
- has_many :post_hotlinked_media
- has_many :reviewables

## Methods

### self

frozen_string_literal: true

require "archetype"
require "digest/sha1"

class Post < ActiveRecord::Base
  include RateLimiter::OnCreateRecord
  include Trashable
  include Searchable
  include HasCustomFields
  include LimitedEdit

  self.ignored_columns = [
    "avg_time", # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
    "image_url", # TODO: Remove when 20240212034010_drop_deprecated_columns has been promoted to pre-deploy
  ]

  cattr_accessor :plugin_permitted_create_params, :plugin_permitted_update_params
  self.plugin_permitted_create_params = {}
  self.plugin_permitted_update_params = {}

  # increase this number to force a system wide post rebake
  # Recreate `index_for_rebake_old` when the number is increased
  # Version 1, was the initial version
  # Version 2 15-12-2017, introduces CommonMark and a huge number of onebox fixes
  BAKED_VERSION = 2

  # Time between the delete and permanent delete of a post
  PERMANENT_DELETE_TIMER = 5.minutes

  rate_limit
  rate_limit :limit_posts_per_day

  belongs_to :user
  belongs_to :topic

  belongs_to :reply_to_user, class_name: "User"

  has_many :post_replies
  has_many :replies, through: :post_replies
  has_many :post_actions, dependent: :destroy
  has_many :topic_links
  has_many :group_mentions, dependent: :destroy

  has_many :upload_references, as: :target, dependent: :destroy
  has_many :uploads, through: :upload_references

  has_one :post_stat

  has_many :bookmarks, as: :bookmarkable

  has_one :incoming_email

  has_many :post_details

  has_many :post_revisions
  has_many :revisions, -> { order(:number) }, foreign_key: :post_id, class_name: "PostRevision"

  has_many :moved_posts_as_old_post,
           class_name: "MovedPost",
           foreign_key: :old_post_id,
           dependent: :destroy
  has_many :moved_posts_as_new_post,
           class_name: "MovedPost",
           foreign_key: :new_post_id,
           dependent: :destroy

  has_many :user_actions, foreign_key: :target_post_id

  belongs_to :image_upload, class_name: "Upload"

  has_many :post_hotlinked_media, dependent: :destroy, class_name: "PostHotlinkedMedia"
  has_many :reviewables, as: :target, dependent: :destroy

  validates_with PostValidator, unless: :skip_validation
  validates :edit_reason, length: { maximum: 1000 }

  after_commit :index_search

  # We can pass several creating options to a post via attributes
  attr_accessor :image_sizes,
                :quoted_post_numbers,
                :no_bump,
                :invalidate_oneboxes,
                :cooking_options,
                :skip_unique_check,
                :skip_validation

  MISSING_UPLOADS = "missing uploads"
  MISSING_UPLOADS_IGNORED = "missing uploads ignored"
  NOTICE = "notice"

  SHORT_POST_CHARS = 1200

  register_custom_field_type(MISSING_UPLOADS, :json)
  register_custom_field_type(MISSING_UPLOADS_IGNORED, :boolean)

  register_custom_field_type(NOTICE, :json)

  scope :private_posts_for_user,
        ->(user) do
          where(
            "topics.id IN (#{Topic::PRIVATE_MESSAGES_SQL_USER})
      OR topics.id IN (#{Topic::PRIVATE_MESSAGES_SQL_GROUP})",
            user_id: user.id,
          )
        end

  scope :by_newest, -> { order("created_at DESC, id DESC") }
  scope :by_post_number, -> { order("post_number ASC") }
  scope :with_user, -> { includes(:user) }
  scope :created_since, ->(time_ago) { where("posts.created_at > ?", time_ago) }
  scope :public_posts,
        -> { joins(:topic).where("topics.archetype <> ?", Archetype.private_message) }
  scope :private_posts,
        -> { joins(:topic).where("topics.archetype = ?", Archetype.private_message) }
  scope :with_topic_subtype, ->(subtype) { joins(:topic).where("topics.subtype = ?", subtype) }
  scope :visible, -> { joins(:topic).where("topics.visible = true").where(hidden: false) }
  scope :secured,
        ->(guardian) { where("posts.post_type IN (?)", Topic.visible_post_types(guardian&.user)) }

  scope :for_mailing_list,
        ->(user, since) do
          q =
            created_since(since).joins(
              "INNER JOIN (#{Topic.for_digest(user, Time.at(0)).select(:id).to_sql}) AS digest_topics ON digest_topics.id = posts.topic_id",
            ) # we want all topics with new content, regardless when they were created
              .order("posts.created_at ASC")

          q = q.where.not(post_type: Post.types[:whisper]) unless user.staff?
          q
        end

  scope :raw_match,
        ->(pattern, type = "string") do
          type = type&.downcase

          case type
          when "string"
            where("raw ILIKE ?", "%#{pattern}%")
          when "regex"
            where("raw ~* ?", "(?n)#{pattern}")
          end
        end

  scope :have_uploads,
        -> do
          where(
            "
          (
            posts.cooked LIKE '%<a %' OR
            posts.cooked LIKE '%<img %' OR
            posts.cooked LIKE '%<video %'
          ) AND (
            posts.cooked LIKE ? OR
            posts.cooked LIKE '%/original/%' OR
            posts.cooked LIKE '%/optimized/%' OR
            posts.cooked LIKE '%data-orig-src=%' OR
            posts.cooked LIKE '%/uploads/short-url/%'
          )",
            "%/uploads/#{RailsMultisite::ConnectionManagement.current_db}/%",
          )
        end

  delegate :username, to: :user

### publish_message

special failsafe for posts missing topics consistency checks should fix,
    # but message is safe to skip
    return unless topic

    skip_topic_stats = opts.delete(:skip_topic_stats)

    message = {
      id: id,
      post_number: post_number,
      updated_at: Time.now,
      user_id: user_id,
      last_editor_id: last_editor_id,
      type: type,
      version: version,
    }.merge(opts)

    publish_message!("/topic/#{topic_id}", message)
    Topic.publish_stats_to_clients!(topic.id, type) unless skip_topic_stats
  end

### unique_post_key

The key we use in redis to ensure unique posts

### acting_user

For some posts, for example those imported via RSS, we support raw HTML. In that
    # case we can skip the rendering pipeline.
    return raw if cook_method == Post.cook_methods[:raw_html]

    options = opts.dup
    options[:cook_method] = cook_method

    # A rule in our Markdown pipeline may have Guardian checks that require a
    # user to be present. The last editing user of the post will be more
    # generally up to date than the creating user. For example, we use
    # this when cooking #hashtags to determine whether we should render
    # the found hashtag based on whether the user can access the category it
    # is referencing.
    options[:user_id] = self.last_editor_id
    options[:omit_nofollow] = true if omit_nofollow?
    options[:post_id] = self.id

    if self.should_secure_uploads?
      each_upload_url do |url|
        uri = URI.parse(url)
        if FileHelper.is_supported_media?(File.basename(uri.path))
          raw =
            raw.sub(
              url,
              Rails.application.routes.url_for(
                controller: "uploads",
                action: "show_secure",
                path: uri.path[1..-1],
                host: Discourse.current_hostname,
              ),
            )
        end
      end
    end

    cooked = post_analyzer.cook(raw, options)

    new_cooked = Plugin::Filter.apply(:after_post_cook, self, cooked)

    if post_type == Post.types[:regular]
      if new_cooked != cooked && new_cooked.blank?
        Rails.logger.debug("Plugin is blanking out post: #{self.url}\nraw: #{raw}")
      elsif new_cooked.blank?
        Rails.logger.debug("Blank post detected post: #{self.url}\nraw: #{raw}")
      end
    end

    new_cooked
  end

  # Sometimes the post is being edited by someone else, for example, a mod.
  # If that's the case, they should not be bound by the original poster's
  # restrictions, for example on not posting images.

### has_host_spam

Prevent new users from posting the same hosts too many times.

### delete_post_notices

percent rank has tons of ties
    where(topic_id: topic_id).where(
      [
        "posts.id = ANY(
          (
            SELECT posts.id
            FROM posts
            WHERE posts.topic_id = #{topic_id.to_i}
            AND posts.post_number = 1
          ) UNION
          (
            SELECT p1.id
            FROM posts p1
            WHERE p1.percent_rank <= ?
            AND p1.topic_id = #{topic_id.to_i}
            ORDER BY p1.percent_rank
            LIMIT ?
          )
        )",
        SiteSetting.summary_percent_filter.to_f / 100.0,
        SiteSetting.summary_max_results,
      ],
    )
  end

### external_id

We only filter quotes when there is exactly 1
    return cooked unless (quote_count == 1)

    parent_raw = parent_post.raw.sub(%r{\[quote.+/quote\]}m, "")

    if raw[parent_raw] || (parent_raw.size < SHORT_POST_CHARS)
      return cooked.sub(%r{\<aside.+\</aside\>}m, "")
    end

    cooked
  end

### excerpt

Strip out most of the markup

### should_secure_uploads

NOTE (martin): This is turning into hack city; when changing this also
  # consider how it interacts with UploadSecurity and the uploads.rake tasks.

### hide

NOTE: This is to be used for plugins where adding a new public upload
    # type that should not be secured via UploadSecurity.register_custom_public_type
    # is not an option. This also is not taken into account in the secure upload
    # rake tasks, and will more than likely change in future.
    modifier_result =
      DiscoursePluginRegistry.apply_modifier(
        :post_should_secure_uploads?,
        nil,
        self,
        topic_including_deleted,
      )
    return modifier_result if !modifier_result.nil?

    # NOTE: This is meant to be a stopgap solution to prevent secure uploads
    # in a single place (private messages) for sensitive admin data exports.
    # Ideally we would want a more comprehensive way of saying that certain
    # upload types get secured which is a hybrid/mixed mode secure uploads,
    # but for now this will do the trick.
    return topic_including_deleted.private_message? if SiteSetting.secure_uploads_pm_only?

    SiteSetting.login_required? || topic_including_deleted.private_message? ||
      topic_including_deleted.read_restricted_category?
  end

### unhide

We need to do this because TopicStatusUpdater also does the decrement
      # and we don't want to double count for the OP.
      UserStatCountUpdater.decrement!(self) if should_update_user_stat
    end

    # inform user
    if user.present?
      options = {
        url: url,
        edit_delay: SiteSetting.cooldown_minutes_after_hiding_posts,
        flag_reason:
          I18n.t(
            "flag_reasons.#{post_action_type_view.types[post_action_type_id]}",
            locale: SiteSetting.default_locale,
            base_path: Discourse.base_path,
            default: PostActionType.names[post_action_type_id],
          ),
      }

      message = custom_message
      message = hiding_again ? :post_hidden_again : :post_hidden if message.nil?

      Jobs.enqueue_in(
        5.seconds,
        :send_system_message,
        user_id: user.id,
        message_type: message.to_s,
        message_options: options,
      )
    end
  end

### full_url

NOTE: We have to consider `nil` a valid reason here because historically
      # topics didn't have a visibility_reason_id, if we didn't do this we would
      # break backwards compat since we cannot backfill data.
      hidden_because_of_op_flagging =
        self.topic.visibility_reason_id == Topic.visibility_reasons[:op_flag_threshold_reached] ||
          self.topic.visibility_reason_id.nil?

      if is_first_post? && hidden_because_of_op_flagging
        self.topic.update_status(
          "visible",
          true,
          Discourse.system_user,
          { visibility_reason_id: Topic.visibility_reasons[:op_unhidden] },
        )
        should_update_user_stat = false
      end

      # We need to do this because TopicStatusUpdater also does the increment
      # and we don't want to double count for the OP.
      UserStatCountUpdater.increment!(self) if should_update_user_stat

      save(validate: false)
    end

    publish_change_to_clients!(:acted)
  end

### set_owner

Extracts urls from the body
    TopicLink.extract_from(self)
    QuotedPost.extract_from(self)

    # make sure we trigger the post process
    trigger_post_process(bypass_bump: true, priority: priority)

    publish_change_to_clients!(:rebaked)

    new_cooked != old_cooked
  end

### extract_quoted_post_numbers

TODO: move to post-analyzer?
  # Determine what posts are quoted by this post

### save_reply_relationships

Create relationships for the quotes
    raw
      .scan(/\[quote=\"([^"]+)"\]/)
      .each do |quote|
        args = parse_quote_into_arguments(quote)
        # If the topic attribute is present, ensure it's the same topic
        if !(args[:topic].present? && topic_id != args[:topic]) && args[:post] != post_number
          temp_collector << args[:post]
        end
      end

    temp_collector.uniq!
    self.quoted_post_numbers = temp_collector
    self.quote_count = temp_collector.size
  end

### trigger_post_process

Create a reply relationship between quoted posts and this new post
    self.quoted_post_numbers.each do |p|
      post = Post.find_by(topic_id: topic_id, post_number: p)
      create_reply_relationship_with(post)
    end
  end

  # Enqueue post processing for this post

### revert_to

ignore posts that aren't replies to exactly one post
    # for example it skips a post when it contains 2 quotes (which are replies) from different posts
    builder.where("count = 1") if only_replies_to_single_post

    replies = builder.query_hash(post_id: id, max_reply_level: MAX_REPLY_LEVEL, topic_id: topic_id)
    replies.each { |r| r.symbolize_keys! }

    secured_ids = Post.secured(guardian).where(id: replies.map { |r| r[:id] }).pluck(:id).to_set

    replies.reject { |r| !secured_ids.include?(r[:id]) }
  end

### update_uploads_secure_status

Link any video thumbnails
      if SiteSetting.video_thumbnails_enabled && upload.present? &&
           FileHelper.supported_video.include?(upload.extension&.downcase)
        # Video thumbnails have the filename of the video file sha1 with a .png or .jpg extension.
        # This is because at time of upload in the composer we don't know the topic/post id yet
        # and there is no thumbnail info added to the markdown to tie the thumbnail to the topic/post after
        # creation.
        thumbnail =
          Upload
            .where("original_filename like ?", "#{upload.sha1}.%")
            .order(id: :desc)
            .first if upload.sha1.present?
        if thumbnail.present?
          upload_ids << thumbnail.id
          if self.is_first_post? && !self.topic.image_upload_id
            self.topic.update_column(:image_upload_id, thumbnail.id)
            extra_sizes =
              ThemeModifierHelper.new(
                theme_ids: Theme.user_selectable.pluck(:id),
              ).topic_thumbnail_sizes
            self.topic.generate_thumbnails!(extra_sizes: extra_sizes)
          end
        end
      end
      upload_ids << upload.id if upload.present?
    end

    upload_references =
      upload_ids.map do |upload_id|
        {
          target_id: self.id,
          target_type: self.class.name,
          upload_id: upload_id,
          created_at: Time.zone.now,
          updated_at: Time.zone.now,
        }
      end

    UploadReference.transaction do
      UploadReference.where(target: self).delete_all
      UploadReference.insert_all(upload_references) if upload_references.size > 0

      if SiteSetting.secure_uploads?
        Upload
          .where(id: upload_ids, access_control_post_id: nil)
          .where("id NOT IN (SELECT upload_id FROM custom_emojis)")
          .update_all(access_control_post_id: self.id)
      end
    end
  end

