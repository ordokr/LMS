# Discourse Source Code Analysis

_Analysis performed on 2025-04-04_

## Overview

- **Total Files**: 12894
- **Lines of Code**: 978,310
- **Models**: 265
- **Controllers**: 125

## File Types

| Extension | Count | Lines of Code |
|-----------|-------|---------------|
| .json | 104 | 0 |
| .rb | 5928 | 642,926 |
| (no extension) | 92 | 0 |
| .yml | 1253 | 0 |
| .cjs | 6 | 0 |
| .sample | 9 | 0 |
| .ico | 3 | 0 |
| .png | 107 | 0 |
| .md | 30 | 0 |
| .js | 2351 | 266,721 |
| .gjs | 1580 | 0 |
| .hbs | 136 | 0 |
| .html | 201 | 8,707 |
| .hbr | 19 | 0 |
| .css | 3 | 135 |
| .erb | 144 | 4,016 |
| .scss | 493 | 55,805 |
| .conf | 4 | 0 |
| .production-sample | 1 | 0 |
| .ru | 1 | 0 |
| .sublime-project | 1 | 0 |
| .mjs | 3 | 0 |
| .lock | 1 | 0 |
| .txt | 10 | 0 |
| .mustache | 37 | 0 |
| .rake | 57 | 0 |
| .thor | 3 | 0 |
| .sql | 18 | 0 |
| .patch | 7 | 0 |
| .mp3 | 3 | 0 |
| .response | 88 | 0 |
| .ttf | 1 | 0 |
| .gif | 5 | 0 |
| .jpg | 4 | 0 |
| .yaml | 7 | 0 |
| .svg | 19 | 0 |
| .wasm | 2 | 0 |
| .gz | 4 | 0 |
| .csv | 12 | 0 |
| .eml | 127 | 0 |
| .atom | 2 | 0 |
| .rss | 3 | 0 |
| .webp | 1 | 0 |
| .long-FileExtension | 1 | 0 |
| .not_image | 1 | 0 |
| .bin | 3 | 0 |
| .heic | 1 | 0 |
| .pdf | 3 | 0 |
| .mp4 | 1 | 0 |
| .mmdb | 1 | 0 |
| .zip | 1 | 0 |
| .woff2 | 1 | 0 |
| .icm | 1 | 0 |

## Models

| Model | File | Fields | Associations |
|-------|------|--------|-------------|
| About | app\models\about.rb | 1 | none |
| AdminDashboardData | app\models\admin_dashboard_data.rb | 1 | none |
| AdminDashboardGeneralData | app\models\admin_dashboard_general_data.rb | 0 | none |
| AdminDashboardIndexData | app\models\admin_dashboard_index_data.rb | 0 | none |
| AdminNotice | app\models\admin_notice.rb | 0 | none |
| AllowedPmUser | app\models\allowed_pm_user.rb | 0 | user, allowed_pm_user |
| AnonymousUser | app\models\anonymous_user.rb | 0 | user, master_user |
| ApiKey | app\models\api_key.rb | 0 | api_key_scopes, user, created_by |
| ApiKeyScope | app\models\api_key_scope.rb | 0 | none |
| ApplicationRequest | app\models\application_request.rb | 0 | none |
| AssociatedGroup | app\models\associated_group.rb | 0 | user_associated_groups, users, group_associated_groups, groups |
| AutoTrackDurationSiteSetting | app\models\auto_track_duration_site_setting.rb | 0 | none |
| BackupDraftPost | app\models\backup_draft_post.rb | 0 | user, post |
| BackupDraftTopic | app\models\backup_draft_topic.rb | 0 | user, topic |
| BackupFile | app\models\backup_file.rb | 1 | none |
| BackupLocationSiteSetting | app\models\backup_location_site_setting.rb | 0 | none |
| BackupMetadata | app\models\backup_metadata.rb | 0 | none |
| Badge | app\models\badge.rb | 1 | badge_type, badge_grouping, image_upload, user_badges, upload_references |
| BadgeGrouping | app\models\badge_grouping.rb | 0 | badges |
| BadgeType | app\models\badge_type.rb | 0 | badges |
| BaseFontSetting | app\models\base_font_setting.rb | 0 | none |
| Bookmark | app\models\bookmark.rb | 0 | user, bookmarkable |
| Category | app\models\category.rb | 2 | topic, topic_only_relative_url, user, latest_post, uploaded_logo, uploaded_logo_dark, uploaded_background, uploaded_background_dark, topics, category_users, category_featured_topics, featured_topics, category_groups, category_moderation_groups, groups, moderating_groups, topic_timers, upload_references, category_setting, parent_category, subcategories, category_tags, tags, none_synonym_tags, category_tag_groups, tag_groups, category_required_tag_groups, sidebar_section_links, embeddable_hosts, category_form_templates, form_templates |
| CategoryAndTopicLists | app\models\category_and_topic_lists.rb | 1 | none |
| CategoryCustomField | app\models\category_custom_field.rb | 0 | category |
| CategoryFeaturedTopic | app\models\category_featured_topic.rb | 0 | category, topic |
| CategoryFormTemplate | app\models\category_form_template.rb | 0 | category, form_template |
| CategoryGroup | app\models\category_group.rb | 0 | category, group |
| CategoryList | app\models\category_list.rb | 2 | none |
| CategoryModerationGroup | app\models\category_moderation_group.rb | 0 | category, group |
| CategoryPageStyle | app\models\category_page_style.rb | 0 | none |
| CategoryRequiredTagGroup | app\models\category_required_tag_group.rb | 0 | category, tag_group |
| CategorySearchData | app\models\category_search_data.rb | 0 | none |
| CategorySetting | app\models\category_setting.rb | 0 | category |
| CategoryTag | app\models\category_tag.rb | 0 | category, tag |
| CategoryTagGroup | app\models\category_tag_group.rb | 0 | category, tag_group |
| CategoryTagStat | app\models\category_tag_stat.rb | 0 | category, tag |
| CategoryUser | app\models\category_user.rb | 0 | category, user |
| ChildTheme | app\models\child_theme.rb | 0 | parent_theme, child_theme |
| ColorScheme | app\models\color_scheme.rb | 2 | color_scheme_colors, theme, theme_color_scheme, owning_theme |
| ColorSchemeColor | app\models\color_scheme_color.rb | 0 | color_scheme |
| ColorSchemeSetting | app\models\color_scheme_setting.rb | 0 | none |
| CustomEmoji | app\models\custom_emoji.rb | 0 | upload, user, upload_references |
| Developer | app\models\developer.rb | 0 | user |
| DigestEmailSiteSetting | app\models\digest_email_site_setting.rb | 0 | none |
| DirectoryColumn | app\models\directory_column.rb | 0 | user_field |
| DirectoryItem | app\models\directory_item.rb | 0 | user, user_stat |
| DiscourseConnect | app\models\discourse_connect.rb | 0 | none |
| DiscourseVersionCheck | app\models\discourse_version_check.rb | 1 | none |
| DismissedTopicUser | app\models\dismissed_topic_user.rb | 0 | user, topic |
| DoNotDisturbTiming | app\models\do_not_disturb_timing.rb | 0 | user |
| Draft | app\models\draft.rb | 0 | user, upload_references |
| DraftSequence | app\models\draft_sequence.rb | 0 | none |
| EmailChangeRequest | app\models\email_change_request.rb | 0 | user, old_email_token, new_email_token, requested_by |
| EmailLevelSiteSetting | app\models\email_level_site_setting.rb | 0 | none |
| EmailLog | app\models\email_log.rb | 0 | user, post, smtp_group |
| EmailStyle | app\models\email_style.rb | 0 | none |
| EmailToken | app\models\email_token.rb | 0 | user |
| EmbeddableHost | app\models\embeddable_host.rb | 0 | category, user, embeddable_host_tags, tags |
| EmbeddableHostTag | app\models\embeddable_host_tag.rb | 0 | embeddable_host, tag |
| Embedding | app\models\embedding.rb | 0 | none |
| Emoji | app\models\emoji.rb | 1 | none |
| EmojiSetSiteSetting | app\models\emoji_set_site_setting.rb | 0 | none |
| ExternalUploadStub | app\models\external_upload_stub.rb | 0 | created_by |
| Flag | app\models\flag.rb | 1 | none |
| FormTemplate | app\models\form_template.rb | 0 | category_form_templates, categories |
| FullNameRequirement | app\models\full_name_requirement.rb | 0 | none |
| GivenDailyLike | app\models\given_daily_like.rb | 0 | user |
| GlobalSetting | app\models\global_setting.rb | 2 | none |
| Group | app\models\group.rb | 1 | category_groups, category_moderation_groups, group_users, group_requests, group_mentions, group_associated_groups, group_archived_messages, categories, moderation_categories, users, human_users, requesters, group_histories, group_category_notification_defaults, group_tag_notification_defaults, associated_groups, flair_upload, upload_references, smtp_updated_by, imap_updated_by |
| GroupArchivedMessage | app\models\group_archived_message.rb | 0 | group, topic |
| GroupAssociatedGroup | app\models\group_associated_group.rb | 0 | group, associated_group |
| GroupCategoryNotificationDefault | app\models\group_category_notification_default.rb | 0 | group, category |
| GroupCustomField | app\models\group_custom_field.rb | 0 | group |
| GroupHistory | app\models\group_history.rb | 0 | group, acting_user, target_user |
| GroupMention | app\models\group_mention.rb | 0 | post, group |
| GroupRequest | app\models\group_request.rb | 0 | group, user |
| GroupTagNotificationDefault | app\models\group_tag_notification_default.rb | 0 | group, tag |
| GroupUser | app\models\group_user.rb | 0 | group, user |
| IgnoredUser | app\models\ignored_user.rb | 0 | user, ignored_user |
| ImapSyncLog | app\models\imap_sync_log.rb | 0 | group |
| IncomingDomain | app\models\incoming_domain.rb | 0 | none |
| IncomingEmail | app\models\incoming_email.rb | 0 | user, topic, post, group |
| IncomingLink | app\models\incoming_link.rb | 1 | post, user, incoming_referer |
| IncomingLinksReport | app\models\incoming_links_report.rb | 1 | none |
| IncomingReferer | app\models\incoming_referer.rb | 0 | incoming_domain |
| InterfaceColorSelectorSetting | app\models\interface_color_selector_setting.rb | 0 | none |
| Invite | app\models\invite.rb | 0 | invited_by, invited_users, users, invited_groups, groups, topic_invites, topics |
| InvitedGroup | app\models\invited_group.rb | 0 | group, invite |
| InvitedUser | app\models\invited_user.rb | 0 | user, invite |
| InviteRedeemer | app\models\invite_redeemer.rb | 1 | none |
| JavascriptCache | app\models\javascript_cache.rb | 0 | theme_field, theme |
| LikeNotificationFrequencySiteSetting | app\models\like_notification_frequency_site_setting.rb | 0 | none |
| LinkedTopic | app\models\linked_topic.rb | 0 | topic |
| LocaleSiteSetting | app\models\locale_site_setting.rb | 0 | none |
| MailingListModeSiteSetting | app\models\mailing_list_mode_site_setting.rb | 0 | none |
| MobileCategoryPageStyle | app\models\mobile_category_page_style.rb | 0 | none |
| MovedPost | app\models\moved_post.rb | 0 | old_topic, old_post, new_topic, new_post, posting_user, user |
| MutedUser | app\models\muted_user.rb | 0 | user, muted_user |
| NavigationMenuSiteSetting | app\models\navigation_menu_site_setting.rb | 0 | none |
| NewTopicDurationSiteSetting | app\models\new_topic_duration_site_setting.rb | 0 | none |
| Notification | app\models\notification.rb | 3 | user, topic, shelved_notification |
| NotificationLevelWhenReplyingSiteSetting | app\models\notification_level_when_replying_site_setting.rb | 0 | none |
| Oauth2UserInfo | app\models\oauth2_user_info.rb | 0 | user |
| OnceoffLog | app\models\onceoff_log.rb | 0 | none |
| OneboxLocaleSiteSetting | app\models\onebox_locale_site_setting.rb | 0 | none |
| OptimizedImage | app\models\optimized_image.rb | 0 | upload |
| Permalink | app\models\permalink.rb | 1 | topic, post, category, tag, user |
| PluginStore | app\models\plugin_store.rb | 1 | none |
| PluginStoreRow | app\models\plugin_store_row.rb | 0 | none |
| Post | app\models\post.rb | 2 | user, topic, reply_to_user, post_replies, replies, post_actions, topic_links, group_mentions, upload_references, uploads, post_stat, bookmarks, incoming_email, post_details, post_revisions, revisions, moved_posts_as_old_post, moved_posts_as_new_post, user_actions, image_upload, post_hotlinked_media, reviewables |
| PostAction | app\models\post_action.rb | 0 | post, user, post_action_type, related_post, target_user |
| PostActionType | app\models\post_action_type.rb | 2 | none |
| PostAnalyzer | app\models\post_analyzer.rb | 0 | none |
| PostCustomField | app\models\post_custom_field.rb | 0 | post |
| PostDetail | app\models\post_detail.rb | 0 | post |
| PostHotlinkedMedia | app\models\post_hotlinked_media.rb | 0 | post, upload |
| PostMover | app\models\post_mover.rb | 1 | none |
| PostReply | app\models\post_reply.rb | 0 | post, reply |
| PostReplyKey | app\models\post_reply_key.rb | 0 | post, user |
| PostRevision | app\models\post_revision.rb | 0 | post, user |
| PostSearchData | app\models\post_search_data.rb | 0 | none |
| PostStat | app\models\post_stat.rb | 0 | post |
| PostStripper | app\models\post_stripper.rb | 0 | none |
| PostTiming | app\models\post_timing.rb | 0 | topic, user |
| PreviousRepliesSiteSetting | app\models\previous_replies_site_setting.rb | 0 | none |
| PrivateMessageTopicTrackingState | app\models\private_message_topic_tracking_state.rb | 0 | none |
| ProblemCheck | app\models\problem_check.rb | 2 | none |
| ProblemCheckTracker | app\models\problem_check_tracker.rb | 0 | none |
| PublishedPage | app\models\published_page.rb | 0 | topic |
| PushSubscription | app\models\push_subscription.rb | 0 | user |
| QuotedPost | app\models\quoted_post.rb | 0 | post, quoted_post |
| RedeliveringWebhookEvent | app\models\redelivering_webhook_event.rb | 0 | web_hook_event |
| RemoteTheme | app\models\remote_theme.rb | 0 | theme |
| RemoveMutedTagsFromLatestSiteSetting | app\models\remove_muted_tags_from_latest_site_setting.rb | 0 | none |
| Report | app\models\report.rb | 1 | none |
| Reviewable | app\models\reviewable.rb | 1 | target, created_by, target_created_by, topic, category, reviewable_histories, reviewable_scores |
| ReviewableClaimedTopic | app\models\reviewable_claimed_topic.rb | 0 | topic, user |
| ReviewableFlaggedPost | app\models\reviewable_flagged_post.rb | 0 | none |
| ReviewableHistory | app\models\reviewable_history.rb | 0 | reviewable, created_by |
| ReviewablePost | app\models\reviewable_post.rb | 0 | none |
| ReviewablePrioritySetting | app\models\reviewable_priority_setting.rb | 0 | none |
| ReviewableQueuedPost | app\models\reviewable_queued_post.rb | 0 | none |
| ReviewableScore | app\models\reviewable_score.rb | 0 | reviewable, user, reviewed_by, meta_topic |
| ReviewableSensitivitySetting | app\models\reviewable_sensitivity_setting.rb | 0 | none |
| ReviewableUser | app\models\reviewable_user.rb | 0 | none |
| S3RegionSiteSetting | app\models\s3_region_site_setting.rb | 0 | none |
| ScreenedEmail | app\models\screened_email.rb | 0 | none |
| ScreenedIpAddress | app\models\screened_ip_address.rb | 0 | none |
| ScreenedUrl | app\models\screened_url.rb | 0 | none |
| SearchExperienceSiteSetting | app\models\search_experience_site_setting.rb | 0 | none |
| SearchLog | app\models\search_log.rb | 0 | user |
| SearchSortOrderSiteSetting | app\models\search_sort_order_site_setting.rb | 0 | none |
| SharedDraft | app\models\shared_draft.rb | 0 | topic, category |
| ShelvedNotification | app\models\shelved_notification.rb | 0 | notification |
| SidebarSection | app\models\sidebar_section.rb | 0 | user, sidebar_section_links, sidebar_urls |
| SidebarSectionLink | app\models\sidebar_section_link.rb | 0 | user, linkable, sidebar_section |
| SidebarUrl | app\models\sidebar_url.rb | 0 | none |
| SingleSignOnRecord | app\models\single_sign_on_record.rb | 0 | user |
| Site | app\models\site.rb | 2 | none |
| Sitemap | app\models\sitemap.rb | 0 | none |
| SiteSetting | app\models\site_setting.rb | 0 | upload_references |
| SkippedEmailLog | app\models\skipped_email_log.rb | 0 | email_log, user, post, topic |
| SlugSetting | app\models\slug_setting.rb | 0 | none |
| Stat | app\models\stat.rb | 1 | none |
| StylesheetCache | app\models\stylesheet_cache.rb | 0 | none |
| Tag | app\models\tag.rb | 0 | tag_users, topic_tags, topics, category_tag_stats, category_tags, categories, tag_group_memberships, tag_groups, target_tag, synonyms, sidebar_section_links, embeddable_host_tags, embeddable_hosts |
| TagGroup | app\models\tag_group.rb | 1 | tag_group_memberships, tags, none_synonym_tags, category_tag_groups, category_required_tag_groups, categories, tag_group_permissions, parent_tag |
| TagGroupMembership | app\models\tag_group_membership.rb | 0 | tag, tag_group |
| TagGroupPermission | app\models\tag_group_permission.rb | 0 | tag_group, group |
| TagSearchData | app\models\tag_search_data.rb | 0 | none |
| TagUser | app\models\tag_user.rb | 0 | tag, user |
| Theme | app\models\theme.rb | 3 | user, color_scheme, theme_fields, theme_settings, theme_translation_overrides, child_theme_relation, parent_theme_relation, child_themes, parent_themes, color_schemes, theme_settings_migrations, remote_theme, theme_modifier_set, theme_svg_sprite, settings_field, javascript_cache, theme_color_scheme, owned_color_scheme, locale_fields, upload_fields, extra_scss_fields, yaml_theme_fields, var_theme_fields, builder_theme_fields, migration_fields |
| ThemeColorScheme | app\models\theme_color_scheme.rb | 0 | theme, color_scheme |
| ThemeField | app\models\theme_field.rb | 0 | upload, javascript_cache, upload_reference, theme_settings_migration, theme |
| ThemeModifierSet | app\models\theme_modifier_set.rb | 0 | theme |
| ThemeSetting | app\models\theme_setting.rb | 0 | theme, upload_references |
| ThemeSettingsMigration | app\models\theme_settings_migration.rb | 0 | theme, theme_field |
| ThemeSvgSprite | app\models\theme_svg_sprite.rb | 0 | theme |
| ThemeTranslationOverride | app\models\theme_translation_override.rb | 0 | theme |
| Topic | app\models\topic.rb | 15 | category, category_users, posts, bookmarks, ordered_posts, topic_allowed_users, topic_allowed_groups, incoming_email, group_archived_messages, user_archived_messages, topic_view_stats, allowed_groups, allowed_group_users, allowed_users, topic_tags, tags, tag_users, moved_posts_as_old_topic, moved_posts_as_new_topic, top_topic, topic_hot_score, shared_draft, published_page, user, last_poster, featured_user, featured_user, featured_user, featured_user, topic_users, dismissed_topic_users, topic_links, topic_invites, invites, topic_timers, reviewables, user_profiles, user_warning, first_post, topic_search_data, topic_embed, linked_topic, image_upload, topic_thumbnails |
| TopicAllowedGroup | app\models\topic_allowed_group.rb | 0 | topic, group |
| TopicAllowedUser | app\models\topic_allowed_user.rb | 0 | topic, user |
| TopicConverter | app\models\topic_converter.rb | 1 | none |
| TopicCustomField | app\models\topic_custom_field.rb | 0 | topic |
| TopicEmbed | app\models\topic_embed.rb | 1 | topic, post |
| TopicFeaturedUsers | app\models\topic_featured_users.rb | 1 | none |
| TopicGroup | app\models\topic_group.rb | 0 | group, topic |
| TopicHotScore | app\models\topic_hot_score.rb | 0 | topic |
| TopicInvite | app\models\topic_invite.rb | 0 | topic, invite |
| TopicLink | app\models\topic_link.rb | 0 | topic, user, post, link_topic, link_post, topic_link_clicks |
| TopicLinkClick | app\models\topic_link_click.rb | 0 | topic_link, user |
| TopicList | app\models\topic_list.rb | 1 | none |
| TopicNotifier | app\models\topic_notifier.rb | 0 | none |
| TopicParticipantsSummary | app\models\topic_participants_summary.rb | 1 | none |
| TopicParticipantGroupsSummary | app\models\topic_participant_groups_summary.rb | 1 | none |
| TopicPoster | app\models\topic_poster.rb | 1 | none |
| TopicPostersSummary | app\models\topic_posters_summary.rb | 1 | none |
| TopicSearchData | app\models\topic_search_data.rb | 0 | none |
| TopicTag | app\models\topic_tag.rb | 0 | topic, tag |
| TopicThumbnail | app\models\topic_thumbnail.rb | 0 | upload, optimized_image |
| TopicTimer | app\models\topic_timer.rb | 0 | user, topic, category |
| TopicTrackingState | app\models\topic_tracking_state.rb | 0 | none |
| TopicUser | app\models\topic_user.rb | 1 | user, topic |
| TopicViewItem | app\models\topic_view_item.rb | 0 | user, topic |
| TopicViewStat | app\models\topic_view_stat.rb | 0 | topic |
| TopLists | app\models\top_lists.rb | 1 | none |
| TopMenuItem | app\models\top_menu_item.rb | 1 | none |
| TopTopic | app\models\top_topic.rb | 0 | topic |
| TranslationOverride | app\models\translation_override.rb | 0 | none |
| TrustLevel3Requirements | app\models\trust_level3_requirements.rb | 1 | none |
| TrustLevelAndStaffAndDisabledSetting | app\models\trust_level_and_staff_and_disabled_setting.rb | 0 | none |
| TrustLevelAndStaffSetting | app\models\trust_level_and_staff_setting.rb | 0 | none |
| TrustLevelSetting | app\models\trust_level_setting.rb | 0 | none |
| UnsubscribeKey | app\models\unsubscribe_key.rb | 0 | user, post, topic |
| Upload | app\models\upload.rb | 7 | user, access_control_post, post_hotlinked_media, optimized_images, user_uploads, upload_references, posts, topic_thumbnails, badges |
| UploadReference | app\models\upload_reference.rb | 0 | upload, target |
| User | app\models\user.rb | 6 | posts, topics, uploads, category_users, tag_users, user_api_keys, topic_allowed_users, user_archived_messages, email_change_requests, email_tokens, topic_links, user_uploads, upload_references, user_emails, user_associated_accounts, oauth, user_second_factors, user_badges, user_auth_tokens, group_users, user_warnings, api_keys, push_subscriptions, acting_group_histories, targeted_group_histories, reviewable_scores, invites, user_custom_fields, user_associated_groups, pending_posts, user_option, user_avatar, primary_email, user_stat, user_profile, single_sign_on_record, anonymous_user_master, anonymous_user_shadow, invited_user, user_notification_schedule, user_password, bookmarks, notifications, topic_users, incoming_emails, user_visits, user_auth_token_logs, group_requests, muted_user_records, ignored_user_records, do_not_disturb_timings, sidebar_sections, user_status, user_actions, post_actions, post_timings, directory_items, email_logs, security_keys, all_security_keys, badges, default_featured_user_badges, topics_allowed, groups, secure_categories, associated_groups, totps, master_user, shadow_user, profile_background_upload, card_background_upload, approved_by, primary_group, flair_group, muted_users, ignored_users, uploaded_avatar, sidebar_section_links, embeddable_hosts |
| UsernameValidator | app\models\username_validator.rb | 3 | none |
| UserAction | app\models\user_action.rb | 0 | user, acting_user, target_user, target_post, target_topic |
| UserApiKey | app\models\user_api_key.rb | 0 | user, client, scopes |
| UserApiKeyClient | app\models\user_api_key_client.rb | 0 | keys, scopes |
| UserApiKeyClientScope | app\models\user_api_key_client_scope.rb | 0 | client |
| UserApiKeyScope | app\models\user_api_key_scope.rb | 0 | none |
| UserArchivedMessage | app\models\user_archived_message.rb | 0 | user, topic |
| UserAssociatedAccount | app\models\user_associated_account.rb | 0 | user |
| UserAssociatedGroup | app\models\user_associated_group.rb | 0 | user, associated_group |
| UserAuthToken | app\models\user_auth_token.rb | 1 | user |
| UserAuthTokenLog | app\models\user_auth_token_log.rb | 0 | user |
| UserAvatar | app\models\user_avatar.rb | 0 | user, gravatar_upload, custom_upload, upload_references |
| UserBadge | app\models\user_badge.rb | 0 | badge, user, granted_by, notification, post |
| UserBadges | app\models\user_badges.rb | 1 | none |
| UserBookmarkList | app\models\user_bookmark_list.rb | 2 | none |
| UserCustomField | app\models\user_custom_field.rb | 0 | user |
| UserEmail | app\models\user_email.rb | 3 | user |
| UserExport | app\models\user_export.rb | 0 | user, upload, topic, upload_references |
| UserField | app\models\user_field.rb | 0 | user_field_options, directory_column |
| UserFieldOption | app\models\user_field_option.rb | 0 | user_field |
| UserHistory | app\models\user_history.rb | 0 | acting_user, target_user, post, topic, category |
| UserIpAddressHistory | app\models\user_ip_address_history.rb | 0 | user |
| UserNotificationSchedule | app\models\user_notification_schedule.rb | 0 | user |
| UserOpenId | app\models\user_open_id.rb | 0 | user |
| UserOption | app\models\user_option.rb | 0 | user |
| UserPassword | app\models\user_password.rb | 0 | user |
| UserProfile | app\models\user_profile.rb | 1 | user, card_background_upload, profile_background_upload, granted_title_badge, featured_topic, upload_references, user_profile_views |
| UserProfileView | app\models\user_profile_view.rb | 0 | user_profile |
| UserRequiredFieldsVersion | app\models\user_required_fields_version.rb | 0 | none |
| UserSearch | app\models\user_search.rb | 0 | none |
| UserSearchData | app\models\user_search_data.rb | 0 | none |
| UserSecondFactor | app\models\user_second_factor.rb | 0 | user |
| UserSecurityKey | app\models\user_security_key.rb | 0 | user |
| UserStat | app\models\user_stat.rb | 0 | user |
| UserStatus | app\models\user_status.rb | 0 | user |
| UserSummary | app\models\user_summary.rb | 0 | none |
| UserUpload | app\models\user_upload.rb | 0 | upload, user |
| UserVisit | app\models\user_visit.rb | 0 | none |
| UserWarning | app\models\user_warning.rb | 0 | user, topic, created_by |
| WatchedWord | app\models\watched_word.rb | 0 | watched_word_group |
| WatchedWordGroup | app\models\watched_word_group.rb | 0 | watched_words |
| WebCrawlerRequest | app\models\web_crawler_request.rb | 1 | none |
| WebHook | app\models\web_hook.rb | 0 | web_hook_events, redelivering_webhook_events, web_hook_events_daily_aggregates |
| WebHookEvent | app\models\web_hook_event.rb | 0 | web_hook, redelivering_webhook_event |
| WebHookEventsDailyAggregate | app\models\web_hook_events_daily_aggregate.rb | 0 | web_hook |
| WebHookEventType | app\models\web_hook_event_type.rb | 0 | none |
| WebHookEventTypesHook | app\models\web_hook_event_types_hook.rb | 0 | web_hook_event_type, web_hook |

## Controllers

| Controller | File | Actions | Routes |
|------------|------|---------|--------|
| AboutController | app\controllers\about_controller.rb | 2 | /about/index |
| AdminController | app\controllers\admin\admin_controller.rb | 1 | /admin/index |
| AdminNoticesController | app\controllers\admin\admin_notices_controller.rb | 1 | /adminnotices/destroy |
| ApiController | app\controllers\admin\api_controller.rb | 14 | /api/index, /api/show, /api/update, /api/destroy, /api/create |
| BackupsController | app\controllers\admin\backups_controller.rb | 22 | /backups/index, /backups/create, /backups/show, /backups/destroy |
| BadgesController | app\controllers\admin\badges_controller.rb | 13 | /badges/index, /badges/new, /badges/show, /badges/create, /badges/update, /badges/destroy |
| ColorSchemesController | app\controllers\admin\color_schemes_controller.rb | 6 | /colorschemes/index, /colorschemes/create, /colorschemes/update, /colorschemes/destroy |
| AboutController | app\controllers\admin\config\about_controller.rb | 2 | /about/index, /about/update |
| BrandingController | app\controllers\admin\config\branding_controller.rb | 2 | /branding/index |
| ColorPalettesController | app\controllers\admin\config\color_palettes_controller.rb | 1 | /colorpalettes/show |
| CustomizeController | app\controllers\admin\config\customize_controller.rb | 2 | none |
| FlagsController | app\controllers\admin\config\flags_controller.rb | 8 | /flags/index, /flags/new, /flags/edit, /flags/create, /flags/update, /flags/destroy |
| SiteSettingsController | app\controllers\admin\config\site_settings_controller.rb | 1 | /sitesettings/index |
| DashboardController | app\controllers\admin\dashboard_controller.rb | 9 | /dashboard/index |
| EmailController | app\controllers\admin\email_controller.rb | 19 | /email/index |
| EmailStylesController | app\controllers\admin\email_styles_controller.rb | 2 | /emailstyles/show, /emailstyles/update |
| EmailTemplatesController | app\controllers\admin\email_templates_controller.rb | 10 | /emailtemplates/show, /emailtemplates/update, /emailtemplates/index |
| EmbeddableHostsController | app\controllers\admin\embeddable_hosts_controller.rb | 5 | /embeddablehosts/create, /embeddablehosts/update, /embeddablehosts/destroy |
| EmbeddingController | app\controllers\admin\embedding_controller.rb | 5 | /embedding/show, /embedding/update, /embedding/new, /embedding/edit |
| EmojiController | app\controllers\admin\emoji_controller.rb | 3 | /emoji/index, /emoji/create, /emoji/destroy |
| FormTemplatesController | app\controllers\admin\form_templates_controller.rb | 9 | /formtemplates/index, /formtemplates/new, /formtemplates/create, /formtemplates/show, /formtemplates/edit, /formtemplates/update, /formtemplates/destroy |
| GroupsController | app\controllers\admin\groups_controller.rb | 7 | /groups/create, /groups/destroy |
| ImpersonateController | app\controllers\admin\impersonate_controller.rb | 1 | /impersonate/create |
| PermalinksController | app\controllers\admin\permalinks_controller.rb | 10 | /permalinks/index, /permalinks/new, /permalinks/edit, /permalinks/show, /permalinks/create, /permalinks/update, /permalinks/destroy |
| PluginsController | app\controllers\admin\plugins_controller.rb | 2 | /plugins/index, /plugins/show |
| ReportsController | app\controllers\admin\reports_controller.rb | 4 | /reports/index, /reports/show |
| RobotsTxtController | app\controllers\admin\robots_txt_controller.rb | 5 | /robotstxt/show, /robotstxt/update |
| ScreenedEmailsController | app\controllers\admin\screened_emails_controller.rb | 3 | /screenedemails/index, /screenedemails/destroy |
| ScreenedIpAddressesController | app\controllers\admin\screened_ip_addresses_controller.rb | 6 | /screenedipaddresses/index, /screenedipaddresses/create, /screenedipaddresses/update, /screenedipaddresses/destroy |
| ScreenedUrlsController | app\controllers\admin\screened_urls_controller.rb | 1 | /screenedurls/index |
| SearchController | app\controllers\admin\search_controller.rb | 1 | /search/index |
| SearchLogsController | app\controllers\admin\search_logs_controller.rb | 2 | /searchlogs/index |
| SectionController | app\controllers\admin\section_controller.rb | 1 | /section/show |
| SiteSettingsController | app\controllers\admin\site_settings_controller.rb | 5 | /sitesettings/index, /sitesettings/update |
| SiteTextsController | app\controllers\admin\site_texts_controller.rb | 16 | /sitetexts/index, /sitetexts/show, /sitetexts/update |
| StaffActionLogsController | app\controllers\admin\staff_action_logs_controller.rb | 7 | /staffactionlogs/index |
| StaffController | app\controllers\admin\staff_controller.rb | 0 | none |
| ThemesController | app\controllers\admin\themes_controller.rb | 28 | /themes/index, /themes/create, /themes/update, /themes/destroy, /themes/show |
| UnknownReviewablesController | app\controllers\admin\unknown_reviewables_controller.rb | 1 | /unknownreviewables/destroy |
| UsersController | app\controllers\admin\users_controller.rb | 39 | /users/index, /users/show, /users/destroy |
| UserFieldsController | app\controllers\admin\user_fields_controller.rb | 8 | /userfields/create, /userfields/index, /userfields/show, /userfields/edit, /userfields/update, /userfields/destroy |
| VersionsController | app\controllers\admin\versions_controller.rb | 1 | /versions/show |
| WatchedWordsController | app\controllers\admin\watched_words_controller.rb | 7 | /watchedwords/index, /watchedwords/create, /watchedwords/destroy |
| WebHooksController | app\controllers\admin\web_hooks_controller.rb | 13 | /webhooks/index, /webhooks/show, /webhooks/edit, /webhooks/create, /webhooks/update, /webhooks/destroy |
| ApplicationController | app\controllers\application_controller.rb | 78 | none |
| AssociatedGroupsController | app\controllers\associated_groups_controller.rb | 1 | /associatedgroups/index |
| BadgesController | app\controllers\badges_controller.rb | 2 | /badges/index, /badges/show |
| BookmarksController | app\controllers\bookmarks_controller.rb | 5 | /bookmarks/create, /bookmarks/destroy, /bookmarks/update |
| BootstrapController | app\controllers\bootstrap_controller.rb | 3 | none |
| CategoriesController | app\controllers\categories_controller.rb | 28 | /categories/index, /categories/show, /categories/create, /categories/update, /categories/destroy |
| ClicksController | app\controllers\clicks_controller.rb | 1 | none |
| ComposerController | app\controllers\composer_controller.rb | 11 | none |
| ComposerMessagesController | app\controllers\composer_messages_controller.rb | 2 | /composermessages/index |
| CspReportsController | app\controllers\csp_reports_controller.rb | 3 | /cspreports/create |
| CustomHomepageController | app\controllers\custom_homepage_controller.rb | 1 | /customhomepage/index |
| DirectoryColumnsController | app\controllers\directory_columns_controller.rb | 1 | /directorycolumns/index |
| DirectoryItemsController | app\controllers\directory_items_controller.rb | 3 | /directoryitems/index |
| DoNotDisturbController | app\controllers\do_not_disturb_controller.rb | 4 | /donotdisturb/create, /donotdisturb/destroy |
| DraftsController | app\controllers\drafts_controller.rb | 5 | /drafts/index, /drafts/show, /drafts/create, /drafts/destroy |
| EditDirectoryColumnsController | app\controllers\edit_directory_columns_controller.rb | 3 | /editdirectorycolumns/index, /editdirectorycolumns/update |
| EmailController | app\controllers\email_controller.rb | 3 | none |
| EmbedController | app\controllers\embed_controller.rb | 6 | none |
| EmojisController | app\controllers\emojis_controller.rb | 2 | /emojis/index |
| ExceptionsController | app\controllers\exceptions_controller.rb | 2 | none |
| ExportCsvController | app\controllers\export_csv_controller.rb | 3 | none |
| ExtraLocalesController | app\controllers\extra_locales_controller.rb | 10 | /extralocales/show |
| FinishInstallationController | app\controllers\finish_installation_controller.rb | 8 | /finishinstallation/index |
| FormTemplatesController | app\controllers\form_templates_controller.rb | 3 | /formtemplates/index, /formtemplates/show |
| ForumsController | app\controllers\forums_controller.rb | 1 | none |
| GroupsController | app\controllers\groups_controller.rb | 32 | /groups/index, /groups/show, /groups/new, /groups/edit, /groups/update |
| HashtagsController | app\controllers\hashtags_controller.rb | 3 | none |
| HighlightJsController | app\controllers\highlight_js_controller.rb | 1 | /highlightjs/show |
| InlineOneboxController | app\controllers\inline_onebox_controller.rb | 1 | /inlineonebox/show |
| InvitesController | app\controllers\invites_controller.rb | 19 | /invites/show, /invites/create, /invites/update, /invites/destroy |
| ListController | app\controllers\list_controller.rb | 25 | none |
| MetadataController | app\controllers\metadata_controller.rb | 5 | none |
| NewInviteController | app\controllers\new_invite_controller.rb | 1 | /newinvite/index |
| NewTopicController | app\controllers\new_topic_controller.rb | 1 | /newtopic/index |
| NotificationsController | app\controllers\notifications_controller.rb | 9 | /notifications/index, /notifications/create, /notifications/update, /notifications/destroy |
| OfflineController | app\controllers\offline_controller.rb | 1 | /offline/index |
| OneboxController | app\controllers\onebox_controller.rb | 1 | /onebox/show |
| PageviewController | app\controllers\pageview_controller.rb | 1 | /pageview/index |
| PermalinksController | app\controllers\permalinks_controller.rb | 2 | /permalinks/show |
| PostsController | app\controllers\posts_controller.rb | 49 | /posts/create, /posts/update, /posts/show, /posts/destroy |
| PostActionsController | app\controllers\post_actions_controller.rb | 4 | /postactions/create, /postactions/destroy |
| PostActionUsersController | app\controllers\post_action_users_controller.rb | 2 | /postactionusers/index |
| PostReadersController | app\controllers\post_readers_controller.rb | 2 | /postreaders/index |
| PresenceController | app\controllers\presence_controller.rb | 3 | /presence/update |
| PublishedPagesController | app\controllers\published_pages_controller.rb | 8 | /publishedpages/show, /publishedpages/destroy |
| PushNotificationController | app\controllers\push_notification_controller.rb | 3 | none |
| QunitController | app\controllers\qunit_controller.rb | 3 | none |
| ReviewablesController | app\controllers\reviewables_controller.rb | 18 | /reviewables/index, /reviewables/show, /reviewables/destroy, /reviewables/update |
| ReviewableClaimedTopicsController | app\controllers\reviewable_claimed_topics_controller.rb | 3 | /reviewableclaimedtopics/create, /reviewableclaimedtopics/destroy |
| RobotsTxtController | app\controllers\robots_txt_controller.rb | 3 | /robotstxt/index |
| SafeModeController | app\controllers\safe_mode_controller.rb | 4 | /safemode/index |
| SearchController | app\controllers\search_controller.rb | 8 | /search/show |
| SessionController | app\controllers\session_controller.rb | 40 | /session/create, /session/destroy |
| SidebarSectionsController | app\controllers\sidebar_sections_controller.rb | 10 | /sidebarsections/index, /sidebarsections/create, /sidebarsections/update, /sidebarsections/destroy |
| SimilarTopicsController | app\controllers\similar_topics_controller.rb | 3 | /similartopics/index |
| SitemapController | app\controllers\sitemap_controller.rb | 6 | /sitemap/index |
| SiteController | app\controllers\site_controller.rb | 7 | none |
| SlugsController | app\controllers\slugs_controller.rb | 1 | none |
| StaticController | app\controllers\static_controller.rb | 6 | /static/show |
| StepsController | app\controllers\steps_controller.rb | 1 | /steps/update |
| StylesheetsController | app\controllers\stylesheets_controller.rb | 6 | /stylesheets/show |
| SvgSpriteController | app\controllers\svg_sprite_controller.rb | 5 | /svgsprite/show |
| TagsController | app\controllers\tags_controller.rb | 29 | /tags/index, /tags/show, /tags/update, /tags/destroy |
| TagGroupsController | app\controllers\tag_groups_controller.rb | 9 | /taggroups/index, /taggroups/show, /taggroups/new, /taggroups/create, /taggroups/update, /taggroups/destroy |
| TestRequestsController | app\controllers\test_requests_controller.rb | 2 | none |
| ThemeJavascriptsController | app\controllers\theme_javascripts_controller.rb | 9 | /themejavascripts/show |
| TopicsController | app\controllers\topics_controller.rb | 68 | /topics/show, /topics/update, /topics/destroy |
| TopicViewStatsController | app\controllers\topic_view_stats_controller.rb | 1 | /topicviewstats/index |
| UploadsController | app\controllers\uploads_controller.rb | 20 | /uploads/create, /uploads/show |
| AssociateAccountsController | app\controllers\users\associate_accounts_controller.rb | 5 | none |
| OmniauthCallbacksController | app\controllers\users\omniauth_callbacks_controller.rb | 11 | none |
| UsersController | app\controllers\users_controller.rb | 87 | /users/index, /users/show, /users/update, /users/create, /users/destroy |
| UsersEmailController | app\controllers\users_email_controller.rb | 8 | /usersemail/index, /usersemail/create, /usersemail/update |
| UserActionsController | app\controllers\user_actions_controller.rb | 3 | /useractions/index, /useractions/show |
| UserApiKeysController | app\controllers\user_api_keys_controller.rb | 13 | /userapikeys/new, /userapikeys/create |
| UserApiKeyClientsController | app\controllers\user_api_key_clients_controller.rb | 7 | /userapikeyclients/show, /userapikeyclients/create |
| UserAvatarsController | app\controllers\user_avatars_controller.rb | 10 | /useravatars/show |
| UserBadgesController | app\controllers\user_badges_controller.rb | 10 | /userbadges/index, /userbadges/create, /userbadges/destroy |
| UserStatusController | app\controllers\user_status_controller.rb | 4 | none |
| WebhooksController | app\controllers\webhooks_controller.rb | 20 | none |
| WizardController | app\controllers\wizard_controller.rb | 1 | /wizard/index |
