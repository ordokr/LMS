# we

## Description

frozen_string_literal: true
A

## Methods

### self

frozen_string_literal: true

# A class we can use to serialize the site data
class Site
  include ActiveModel::Serialization

  cattr_accessor :preloaded_category_custom_fields

### self

#
  # Sometimes plugins need to have additional data or options available
  # when rendering custom markdown features/rules that are not available
  # on the default opts.discourse object. These additional options should
  # be namespaced to the plugin adding them.
  #
  # ```
  # Site.markdown_additional_options["chat"] = { limited_pretty_text_markdown_rules: [] }
  # ```
  #
  # These are passed down to markdown rules on opts.discourse.additionalOptions.
  cattr_accessor :markdown_additional_options
  self.markdown_additional_options = {}

### categories

Categories do not change often so there is no need for us to run the
    # same query and spend time creating ActiveRecord objects for every requests.
    #
    # Do note that any new association added to the eager loading needs a
    # corresponding ActiveRecord callback to clear the categories cache.
    Discourse
      .cache
      .fetch(categories_cache_key, expires_in: 30.minutes) do
        categories =
          begin
            query =
              Category
                .includes(
                  :uploaded_logo,
                  :uploaded_logo_dark,
                  :uploaded_background,
                  :uploaded_background_dark,
                  :tags,
                  :tag_groups,
                  :form_templates,
                  category_required_tag_groups: :tag_group,
                )
                .joins("LEFT JOIN topics t on t.id = categories.topic_id")
                .select("categories.*, t.slug topic_slug")
                .order(:position)

            query =
              DiscoursePluginRegistry.apply_modifier(:site_all_categories_cache_query, query, self)

            query.to_a
          end

        if preloaded_category_custom_fields.present?
          Category.preload_custom_fields(categories, preloaded_category_custom_fields)
        end

        ActiveModel::ArraySerializer.new(
          categories,
          each_serializer: SiteCategorySerializer,
        ).as_json
      end
  end

### self

publishing forces the sequence up
    # the cache is validated based on the sequence
    MessageBus.publish(SITE_JSON_CHANNEL, "")
  end

