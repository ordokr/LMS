# Canvas Source Code Analysis

_Analysis performed on 2025-04-04_

## Overview

- **Total Files**: 19872
- **Lines of Code**: 2,577,027
- **Models**: 268
- **Controllers**: 315

## File Types

| Extension | Count | Lines of Code |
|-----------|-------|---------------|
| .yml | 173 | 0 |
| .js | 2875 | 405,422 |
| (no extension) | 243 | 0 |
| .json | 1444 | 0 |
| .html | 20 | 624 |
| .rb | 5773 | 1,229,045 |
| .md | 150 | 0 |
| .erb | 872 | 47,461 |
| .txt | 34 | 0 |
| .lua | 8 | 0 |
| .docx | 4 | 0 |
| .scss | 401 | 56,626 |
| .css | 20 | 3,475 |
| .builder | 1 | 0 |
| .handlebars | 265 | 0 |
| .svg | 519 | 0 |
| .ttf | 20 | 0 |
| .prawn | 1 | 0 |
| .sh | 122 | 0 |
| .template | 2 | 0 |
| .kts | 1 | 0 |
| .groovy | 18 | 0 |
| .py | 1 | 31 |
| .zip | 92 | 0 |
| .example | 34 | 0 |
| .ignore | 1 | 0 |
| .csv | 25 | 0 |
| .sample | 1 | 0 |
| .pem | 2 | 0 |
| .ru | 1 | 0 |
| .xmind | 1 | 0 |
| .png | 621 | 0 |
| .gif | 101 | 0 |
| .plantuml | 2 | 0 |
| .yaml | 19 | 0 |
| .mustache | 1 | 0 |
| .githook | 1 | 0 |
| .jenkins | 1 | 0 |
| .jenkins-cache | 1 | 0 |
| .final | 1 | 0 |
| .linters | 1 | 0 |
| .ruby-runner | 1 | 0 |
| .webpack-assets | 1 | 0 |
| .webpack-builder | 1 | 0 |
| .webpack-cache | 1 | 0 |
| .webpack-runner | 1 | 0 |
| .yarn-runner | 1 | 0 |
| .master-bouncer | 1 | 0 |
| .package-translations | 2 | 0 |
| .production | 1 | 0 |
| .puma | 1 | 0 |
| .lock | 86 | 0 |
| .gemspec | 58 | 0 |
| .gem | 1 | 0 |
| .jpg | 19 | 0 |
| .unknown | 1 | 0 |
| .xss | 94 | 0 |
| .rdoc | 1 | 0 |
| .imscc | 9 | 0 |
| .tar | 3 | 0 |
| .gz | 4 | 0 |
| .rake | 21 | 0 |
| .html_body | 4 | 0 |
| .text_body | 4 | 0 |
| .eml | 4 | 0 |
| .xml | 143 | 0 |
| .qti | 1 | 0 |
| .jsx | 2591 | 433,142 |
| .ts | 1060 | 116,035 |
| .tsx | 1755 | 285,166 |
| .axe | 1 | 0 |
| .contract-tests | 1 | 0 |
| .coverage | 1 | 0 |
| .coverage-js | 1 | 0 |
| .crystalball | 1 | 0 |
| .dive | 1 | 0 |
| .docker-smoke | 1 | 0 |
| .docker-sync | 1 | 0 |
| .dynamodb | 1 | 0 |
| .junit-uploader | 1 | 0 |
| .master-bouncer-check-all | 1 | 0 |
| .postgres | 1 | 0 |
| .redis | 1 | 0 |
| .rspecq | 1 | 0 |
| .flakey_spec_catcher | 1 | 0 |
| .chrome | 1 | 0 |
| .test-subbuild | 1 | 0 |
| .vendored-gems | 1 | 0 |
| .xbrowser | 1 | 0 |
| .xsd | 2 | 0 |
| .snap | 57 | 0 |
| .patch | 1 | 0 |
| .ico | 6 | 0 |
| .pdf | 2 | 0 |
| .woff2 | 10 | 0 |
| .eot | 6 | 0 |
| .woff | 6 | 0 |
| .otf | 1 | 0 |
| .swf | 4 | 0 |
| .xap | 1 | 0 |
| .psd | 1 | 0 |
| .graphql | 1 | 0 |
| .mp3 | 1 | 0 |
| .doc | 1 | 0 |
| .rtf | 1 | 0 |
| .opts | 2 | 0 |
| .xpi | 1 | 0 |
| .bak | 3 | 0 |
| .puml | 2 | 0 |
| .mjs | 1 | 0 |

## Models

| Model | File | Fields | Associations |
|-------|------|--------|-------------|
| AbstractAssignment | app\models\abstract_assignment.rb | 5 | submissions, all_submissions, observer_alerts, provisional_grades, annotatable_attachment, attachments, quiz, assignment_group, discussion_topic, wiki_page, learning_outcome_alignments, rubric_association, rubric, teacher_enrollment, ignores, moderated_grading_selections, context, grading_standard, group_category, grader_section, final_grader, active_groups, group_memberships, assigned_students, enrollments_for_assigned_students, sections_for_assigned_students, duplicate_of, duplicates, assignment_configuration_tool_lookups, tool_settings_context_external_tools, line_items, external_tool_tag, score_statistic, post_policy, moderation_graders, moderation_grader_users, auditor_grade_change_records, lti_resource_links, lti_asset_processors, conditional_release_rules, conditional_release_associations, master_content_tag, parent_assignment, sub_assignments, sub_assignment_submissions, sub_assignment_overrides, estimated_duration |
| AbstractCourse | app\models\abstract_course.rb | 0 | root_account, account, enrollment_term, courses |
| AccessToken | app\models\access_token.rb | 2 | developer_key, user, real_user, account, notification_endpoints |
| Account | app\models\account.rb | 2 | root_account, parent_account, courses, custom_grade_statuses, standard_grade_statuses, favorites, all_courses, terms_of_service, terms_of_service_content, group_categories, all_group_categories, groups, all_groups, all_group_memberships, differentiation_tag_categories, all_differentiation_tag_categories, differentiation_tags, all_differentiation_tags, all_differentiation_tag_memberships, combined_groups_and_differentiation_tags, combined_group_and_differentiation_tag_categories, active_combined_group_and_differentiation_tag_categories, enrollment_terms, active_enrollment_terms, grading_period_groups, grading_periods, enrollments, all_enrollments, temporary_enrollment_pairings, sub_accounts, all_accounts, account_users, active_account_users, course_sections, sis_batches, abstract_courses, root_abstract_courses, user_account_associations, all_users, users, user_past_lti_ids, pseudonyms, pseudonym_users, role_overrides, course_account_associations, child_courses, attachments, active_assignments, folders, active_folders, developer_keys, developer_key_account_bindings, lti_registration_account_bindings, lti_overlays, lti_overlay_versions, lti_notice_handlers, authentication_providers, calendar_events, account_reports, grading_standards, assessment_question_banks, assessment_questions, roles, all_roles, progresses, content_migrations, sis_batch_errors, canvadocs_annotation_contexts, outcome_proficiency, outcome_calculation_method, rubric_imports, rubric_assessment_imports, auditor_authentication_records, auditor_course_records, auditor_grade_change_records, auditor_root_grade_change_records, auditor_feature_flag_records, auditor_pseudonym_records, lti_resource_links, lti_registrations, block_editor_templates, course_template, grading_standard, context_external_tools, error_reports, announcements, alerts, report_snapshots, external_integration_keys, shared_brand_configs, brand_config, blackout_dates |
| AccountNotification | app\models\account_notification.rb | 1 | account, user, account_notification_roles |
| AccountNotificationRole | app\models\account_notification_role.rb | 0 | account_notification, role |
| AccountReport | app\models\account_report.rb | 1 | account, user, attachment, account_report_runners, account_report_rows |
| AccountReportRow | app\models\account_report_row.rb | 0 | account_report, account_report_runner |
| AccountReportRunner | app\models\account_report_runner.rb | 1 | account_report, account_report_rows |
| AccountUser | app\models\account_user.rb | 0 | account, user, role, role_overrides |
| Alert | app\models\alert.rb | 0 | context, criteria |
| AlertCriterion | app\models\alert_criterion.rb | 0 | alert |
| Announcement | app\models\announcement.rb | 0 | context |
| AnnouncementEmbedding | app\models\announcement_embedding.rb | 0 | none |
| AnonymousOrModerationEvent | app\models\anonymous_or_moderation_event.rb | 0 | assignment, user, submission, canvadoc, quiz, context_external_tool |
| ApplicationRecord | app\models\application_record.rb | 0 | none |
| AppointmentGroup | app\models\appointment_group.rb | 3 | appointments, through, _appointments, appointments_participants, appointment_group_contexts, appointment_group_sub_contexts |
| AppointmentGroupContext | app\models\appointment_group_context.rb | 0 | appointment_group, context |
| AppointmentGroupSubContext | app\models\appointment_group_sub_context.rb | 0 | appointment_group, sub_context |
| AssessmentQuestion | app\models\assessment_question.rb | 1 | quiz_questions, attachments, assessment_question_bank |
| AssessmentQuestionBank | app\models\assessment_question_bank.rb | 0 | context, assessment_questions, assessment_question_bank_users, learning_outcome_alignments, quiz_groups |
| AssessmentQuestionBankUser | app\models\assessment_question_bank_user.rb | 0 | assessment_question_bank, user |
| AssessmentRequest | app\models\assessment_request.rb | 0 | user, asset, assessor_asset, assessor, rubric_association, submission_comments, ignores, rubric_assessment |
| AssetUserAccess | app\models\asset_user_access.rb | 0 | context, user, page_views |
| AssetUserAccessLog | app\models\asset_user_access_log.rb | 0 | none |
| Assignment | app\models\assignment.rb | 2 | none |
| AssignmentConfigurationToolLookup | app\models\assignment_configuration_tool_lookup.rb | 0 | tool, assignment |
| AssignmentEmbedding | app\models\assignment_embedding.rb | 0 | assignment |
| AssignmentGroup | app\models\assignment_group.rb | 2 | context, scores, assignments, active_assignments, published_assignments |
| AssignmentOverride | app\models\assignment_override.rb | 2 | root_account, assignment, quiz, context_module, wiki_page, discussion_topic, attachment, set, parent_override, child_overrides, assignment_override_students |
| AssignmentOverrideStudent | app\models\assignment_override_student.rb | 1 | assignment, assignment_override, user, quiz, context_module, wiki_page, discussion_topic, attachment |
| Attachment | app\models\attachment.rb | 5 | context, cloned_item, folder, user, account_report, course_report, group_and_membership_importer, media_object, media_object_by_media_id, media_tracks, submission_draft_attachments, submissions, attachment_associations, assignment_submissions, root_attachment, replacement_attachment, replaced_attachments, sis_batch, thumbnail, thumbnails, children, attachment_upload_statuses, crocodoc_document, canvadoc, usage_rights, canvadocs_annotation_contexts, discussion_entry_drafts, master_content_tag, estimated_duration, lti_assets, attachments |
| AttachmentAssociation | app\models\attachment_association.rb | 0 | attachment, context, user |
| AttachmentUploadStatus | app\models\attachment_upload_status.rb | 0 | attachment |
| Auditors | app\models\auditors.rb | 0 | none |
| AuthenticationProvider | app\models\authentication_provider.rb | 0 | account, pseudonyms |
| BigBlueButtonConference | app\models\big_blue_button_conference.rb | 1 | none |
| BlackoutDate | app\models\blackout_date.rb | 0 | context, root_account |
| BlockEditor | app\models\block_editor.rb | 0 | context |
| BlockEditorTemplate | app\models\block_editor_template.rb | 0 | context |
| Bookmarks | app\models\bookmarks.rb | 0 | none |
| BookmarkService | app\models\bookmark_service.rb | 0 | none |
| BounceNotificationProcessor | app\models\bounce_notification_processor.rb | 1 | none |
| BrandConfig | app\models\brand_config.rb | 0 | accounts, shared_brand_configs, parent |
| CalendarEvent | app\models\calendar_event.rb | 6 | context, user, parent_event, child_events, web_conference, root_account |
| Canvadoc | app\models\canvadoc.rb | 0 | attachment, canvadocs_submissions |
| CanvadocsAnnotationContext | app\models\canvadocs_annotation_context.rb | 0 | attachment, root_account, submission |
| CanvadocsSubmission | app\models\canvadocs_submission.rb | 0 | canvadoc, crocodoc_document, submission |
| CanvasMetadatum | app\models\canvas_metadatum.rb | 0 | none |
| ClonedItem | app\models\cloned_item.rb | 0 | original_item, attachments, discussion_topics, wiki_pages |
| Collaboration | app\models\collaboration.rb | 1 | context, user, collaborators, users |
| Collaborator | app\models\collaborator.rb | 0 | collaboration, group, user |
| CommentBankItem | app\models\comment_bank_item.rb | 0 | course, user |
| CommunicationChannel | app\models\communication_channel.rb | 3 | pseudonym, pseudonyms, user, notification_policies, notification_policy_overrides, delayed_messages, messages |
| ConditionalRelease | app\models\conditional_release.rb | 0 | none |
| ContentExport | app\models\content_export.rb | 2 | context, user, attachment, content_migration, attachments, sent_content_share, received_content_shares, quiz_migration_alerts, epub_export, job_progress |
| ContentMigration | app\models\content_migration.rb | 3 | context, user, attachment, overview_attachment, exported_attachment, asset_map_attachment, source_course, root_account, content_export, migration_issues, quiz_migration_alerts, job_progress |
| ContentParticipation | app\models\content_participation.rb | 1 | content, user |
| ContentParticipationCount | app\models\content_participation_count.rb | 0 | context, user |
| ContentShare | app\models\content_share.rb | 0 | user, content_export, course, group, context_user, sender, root_account |
| ContentTag | app\models\content_tag.rb | 2 | content, context, associated_asset, context_module, learning_outcome, learning_outcome_content, learning_outcome_results, root_account, estimated_duration |
| Context | app\models\context.rb | 0 | none |
| ContextExternalTool | app\models\context_external_tool.rb | 1 | content_tags, context_external_tool_placements, lti_resource_links, progresses, lti_notice_handlers, lti_asset_processors, lti_asset_processor_eula_acceptances, context, developer_key, root_account, lti_registration |
| ContextExternalToolPlacement | app\models\context_external_tool_placement.rb | 0 | context_external_tool |
| ContextModule | app\models\context_module.rb | 2 | context, root_account, context_module_progressions, content_tags, assignment_overrides, assignment_override_students, master_content_tag |
| ContextModuleItem | app\models\context_module_item.rb | 0 | context_module_tags, context_modules |
| ContextModuleProgression | app\models\context_module_progression.rb | 1 | context_module, user, root_account |
| Conversation | app\models\conversation.rb | 1 | conversation_participants, conversation_messages, conversation_message_participants, stream_item, context |
| ConversationBatch | app\models\conversation_batch.rb | 3 | user, root_conversation_message, context |
| ConversationMessage | app\models\conversation_message.rb | 1 | conversation, author, context, conversation_message_participants, attachment_associations, asset |
| ConversationMessageParticipant | app\models\conversation_message_participant.rb | 0 | conversation_message, user, conversation_participant |
| ConversationParticipant | app\models\conversation_participant.rb | 2 | conversation, user, conversation_message_participants |
| Course | app\models\course.rb | 7 | root_account, abstract_course, enrollment_term, template_course, templated_courses, templated_accounts, block_editor_templates, linked_homeroom_course, course_sections, active_course_sections, moved_sections, enrollments, all_enrollments, current_enrollments, all_current_enrollments, prior_enrollments, prior_users, prior_students, participating_enrollments, participating_students, participating_students_by_date, student_enrollments, student_enrollments_including_completed, students, self_enrolled_students, admin_visible_student_enrollments, admin_visible_students, gradable_student_enrollments, gradable_students, all_student_enrollments, all_student_enrollments_including_deleted, all_students, all_students_including_deleted, all_accepted_student_enrollments, all_accepted_students, all_real_enrollments, all_real_users, all_real_student_enrollments, all_real_students, teacher_enrollments, teachers, ta_enrollments, tas, observer_enrollments, observers, non_observer_enrollments, enrollments_excluding_linked_observers, participating_observers, participating_observers_by_date, instructors, instructor_enrollments, participating_instructors, participating_instructors_by_date, admins, admin_enrollments, participating_admins, participating_admins_by_date, student_view_enrollments, student_view_students, custom_gradebook_columns, course_account_associations, users, all_users, current_users, all_current_users, active_users, user_past_lti_ids, group_categories, all_group_categories, combined_group_and_differentiation_tag_categories, active_combined_group_and_differentiation_tag_categories, groups, active_groups, differentiation_tag_categories, all_differentiation_tag_categories, differentiation_tags, active_differentiation_tags, combined_groups_and_differentiation_tags, assignment_groups, assignments, calendar_events, submissions, submission_comments, discussion_topics, active_discussion_topics, all_discussion_topics, discussion_entries, announcements, active_announcements, attachments, active_images, active_assignments, folders, active_folders, messages, context_external_tools, tool_proxies, wiki, wiki_pages, wiki_page_lookups, quizzes, quiz_questions, active_quizzes, assessment_question_banks, assessment_questions, external_feeds, grading_standard, grading_standards, web_conferences, collaborations, context_modules, context_module_progressions, active_context_modules, context_module_tags, media_objects, page_views, asset_user_accesses, role_overrides, content_migrations, content_exports, epub_exports, course_reports, gradebook_filters, web_zip_exports, alerts, appointment_group_contexts, appointment_groups, appointment_participants, content_participation_counts, poll_sessions, grading_period_groups, grading_periods, usage_rights, custom_grade_statuses, sis_post_grades_statuses, progresses, gradebook_csvs, master_course_templates, master_course_subscriptions, late_policy, quiz_migration_alerts, notification_policy_overrides, post_policies, assignment_post_policies, default_post_policy, course_score_statistic, auditor_course_records, auditor_grade_change_records, lti_resource_links, conditional_release_rules, outcome_proficiency, outcome_calculation_method, microsoft_sync_group, microsoft_sync_partial_sync_changes, comment_bank_items, course_paces, blackout_dates, favorites, account |
| CourseAccountAssociation | app\models\course_account_association.rb | 0 | course, course_section, account, account_users |
| CourseDateRange | app\models\course_date_range.rb | 1 | none |
| CoursePace | app\models\course_pace.rb | 0 | course, course_pace_module_items, course_section, user, root_account |
| CoursePaceModuleItem | app\models\course_pace_module_item.rb | 0 | course_pace, module_item, root_account |
| CourseProfile | app\models\course_profile.rb | 0 | none |
| CourseProgress | app\models\course_progress.rb | 1 | none |
| CourseReport | app\models\course_report.rb | 0 | course, user, attachment, root_account |
| CourseScoreStatistic | app\models\course_score_statistic.rb | 0 | course |
| CourseSection | app\models\course_section.rb | 0 | course, nonxlist_course, root_account, enrollment_term, enrollments, all_enrollments, student_enrollments, students, all_student_enrollments, all_students, instructor_enrollments, admin_enrollments, users, course_account_associations, calendar_events, assignment_overrides, discussion_topic_section_visibilities, discussion_topics, course_paces, sis_post_grades_statuses |
| CrocodocDocument | app\models\crocodoc_document.rb | 0 | attachment, canvadocs_submissions |
| CustomData | app\models\custom_data.rb | 1 | user |
| CustomGradebookColumn | app\models\custom_gradebook_column.rb | 0 | course, custom_gradebook_column_data |
| CustomGradebookColumnDatum | app\models\custom_gradebook_column_datum.rb | 0 | custom_gradebook_column |
| CustomGradeStatus | app\models\custom_grade_status.rb | 0 | root_account, created_by, deleted_by, submissions, scores |
| DelayedMessage | app\models\delayed_message.rb | 0 | notification_policy, notification_policy_override, context, communication_channel |
| DelayedNotification | app\models\delayed_notification.rb | 1 | asset |
| DesignerEnrollment | app\models\designer_enrollment.rb | 0 | none |
| DeveloperKey | app\models\developer_key.rb | 2 | user, account, root_account, service_user, lti_registration, page_views, access_tokens, developer_key_account_bindings, context_external_tools, tool_consumer_profile, tool_configuration, ims_registration |
| DeveloperKeyAccountBinding | app\models\developer_key_account_binding.rb | 0 | account, developer_key, root_account, lti_registration_account_binding |
| DiscussionEntry | app\models\discussion_entry.rb | 3 | discussion_entry_drafts, discussion_entry_versions, legacy_subentries, root_discussion_replies, discussion_subentries, unordered_discussion_subentries, flattened_discussion_subentries, discussion_entry_participants, discussion_topic_insight_entries, last_discussion_subentry, discussion_topic, quoted_entry, parent_entry, root_entry, user, mentions, attachment, editor, root_account, external_feed_entry |
| DiscussionEntryDraft | app\models\discussion_entry_draft.rb | 0 | discussion_topic, discussion_entry, attachment, user |
| DiscussionEntryParticipant | app\models\discussion_entry_participant.rb | 0 | discussion_entry, user |
| DiscussionEntryVersion | app\models\discussion_entry_version.rb | 0 | discussion_entry, root_account, user, discussion_topic_insight_entries |
| DiscussionTopic | app\models\discussion_topic.rb | 5 | discussion_entries, discussion_entry_drafts, rated_discussion_entries, root_discussion_entries, external_feed_entry, root_account, external_feed, context, attachment, editor, root_topic, group_category, sub_assignments, child_topics, discussion_topic_participants, discussion_entry_participants, discussion_topic_section_visibilities, course_sections, user, master_content_tag, summaries, insights, insight_entries, estimated_duration |
| DiscussionTopicEmbedding | app\models\discussion_topic_embedding.rb | 0 | discussion_topic |
| DiscussionTopicInsight | app\models\discussion_topic_insight.rb | 0 | root_account, user, discussion_topic, entries |
| DiscussionTopicParticipant | app\models\discussion_topic_participant.rb | 0 | discussion_topic, user |
| DiscussionTopicSectionVisibility | app\models\discussion_topic_section_visibility.rb | 1 | course_section, discussion_topic |
| DiscussionTopicSummary | app\models\discussion_topic_summary.rb | 0 | root_account, user, discussion_topic, parent, feedback |
| DocumentService | app\models\document_service.rb | 0 | none |
| Enrollment | app\models\enrollment.rb | 1 | course, course_section, root_account, user, sis_pseudonym, associated_user, temporary_enrollment_pairing, role, enrollment_state, role_overrides, pseudonyms, course_account_associations, scores, through |
| EnrollmentDatesOverride | app\models\enrollment_dates_override.rb | 0 | root_account, context, enrollment_term |
| EnrollmentState | app\models\enrollment_state.rb | 1 | enrollment |
| EnrollmentTerm | app\models\enrollment_term.rb | 0 | root_account, grading_period_group, grading_periods, enrollment_dates_overrides, courses, enrollments, course_sections |
| Eportfolio | app\models\eportfolio.rb | 0 | eportfolio_categories, eportfolio_entries, attachments, user |
| EportfolioCategory | app\models\eportfolio_category.rb | 1 | eportfolio_entries, eportfolio |
| EportfolioEntry | app\models\eportfolio_entry.rb | 1 | eportfolio, eportfolio_category, page_comments |
| EpubExport | app\models\epub_export.rb | 0 | content_export, course, user, attachments, epub_attachment, zip_attachment, job_progress |
| ErrorReport | app\models\error_report.rb | 1 | user, account |
| EstimatedDuration | app\models\estimated_duration.rb | 0 | assignment, quiz, wiki_page, discussion_topic, attachment, content_tag |
| EtherpadCollaboration | app\models\etherpad_collaboration.rb | 0 | none |
| ExternalFeed | app\models\external_feed.rb | 0 | user, context, external_feed_entries, discussion_topics |
| ExternalFeedEntry | app\models\external_feed_entry.rb | 0 | user, external_feed, asset |
| ExternalIntegrationKey | app\models\external_integration_key.rb | 0 | context |
| ExternalToolCollaboration | app\models\external_tool_collaboration.rb | 0 | none |
| Favorite | app\models\favorite.rb | 0 | context, user, root_account |
| FeatureFlag | app\models\feature_flag.rb | 1 | context |
| Folder | app\models\folder.rb | 1 | context, cloned_item, parent_folder, file_attachments, active_file_attachments, visible_file_attachments, sub_folders, active_sub_folders |
| GoogleDocsCollaboration | app\models\google_docs_collaboration.rb | 0 | none |
| GradebookCsv | app\models\gradebook_csv.rb | 0 | course, user, attachment, progress |
| GradebookFilter | app\models\gradebook_filter.rb | 0 | user, course |
| GradebookUpload | app\models\gradebook_upload.rb | 0 | course, user, progress, attachments |
| GradingPeriod | app\models\grading_period.rb | 0 | grading_period_group, scores, submissions, auditor_grade_change_records |
| GradingPeriodGroup | app\models\grading_period_group.rb | 0 | root_account, course, grading_periods, enrollment_terms |
| GradingStandard | app\models\grading_standard.rb | 1 | context, user, assignments, courses, accounts |
| Group | app\models\group.rb | 2 | group_memberships, users, user_past_lti_ids, participating_group_memberships, participating_users, context, group_category, account, root_account, calendar_events, discussion_topics, active_discussion_topics, all_discussion_topics, discussion_entries, announcements, active_announcements, attachments, active_images, active_assignments, all_attachments, folders, active_folders, submissions_folders, collaborators, external_feeds, messages, wiki, wiki_pages, wiki_page_lookups, web_conferences, collaborations, media_objects, content_migrations, content_exports, usage_rights, avatar_attachment, leader, lti_resource_links, favorites |
| GroupAndMembershipImporter | app\models\group_and_membership_importer.rb | 1 | group_category, attachment |
| GroupCategory | app\models\group_category.rb | 6 | context, sis_batch, root_account, assignments, groups, progresses, group_and_membership_importers, current_progress |
| GroupLeadership | app\models\group_leadership.rb | 1 | none |
| GroupMembership | app\models\group_membership.rb | 2 | group, user |
| HorizonValidators | app\models\horizon_validators.rb | 0 | none |
| Ignore | app\models\ignore.rb | 0 | user, asset |
| Importers | app\models\importers.rb | 1 | none |
| KalturaMediaFileHandler | app\models\kaltura_media_file_handler.rb | 0 | none |
| LatePolicy | app\models\late_policy.rb | 0 | course |
| LearningOutcome | app\models\learning_outcome.rb | 0 | context, learning_outcome_results, alignments, copied_from, cloned_outcomes |
| LearningOutcomeGroup | app\models\learning_outcome_group.rb | 2 | learning_outcome_group, source_outcome_group, destination_outcome_groups, child_outcome_groups, child_outcome_links, context |
| LearningOutcomeQuestionResult | app\models\learning_outcome_question_result.rb | 0 | learning_outcome_result, learning_outcome, associated_asset, root_account |
| LearningOutcomeResult | app\models\learning_outcome_result.rb | 0 | user, learning_outcome, alignment, association_object, artifact, associated_asset, context, root_account, learning_outcome_question_results |
| LiveAssessments | app\models\live_assessments.rb | 0 | none |
| LlmConfig | app\models\llm_config.rb | 1 | none |
| Lti | app\models\lti.rb | 0 | none |
| LtiConference | app\models\lti_conference.rb | 0 | none |
| Mailer | app\models\mailer.rb | 1 | none |
| ManyRootAccounts | app\models\many_root_accounts.rb | 0 | none |
| MasterCourses | app\models\master_courses.rb | 0 | none |
| MediaObject | app\models\media_object.rb | 1 | user, context, attachment, root_account, media_tracks, attachments_by_media_id |
| MediaSourceFetcher | app\models\media_source_fetcher.rb | 0 | none |
| MediaTrack | app\models\media_track.rb | 0 | user, media_object, attachment, master_content_tags |
| Mention | app\models\mention.rb | 0 | user, discussion_entry, root_account, discussion_topic |
| Message | app\models\message.rb | 3 | communication_channel, context, user, root_account, attachments |
| MicrosoftSync | app\models\microsoft_sync.rb | 0 | none |
| MigrationIssue | app\models\migration_issue.rb | 0 | content_migration, error_report |
| ModeratedGrading | app\models\moderated_grading.rb | 0 | none |
| ModerationGrader | app\models\moderation_grader.rb | 0 | user, assignment |
| Notification | app\models\notification.rb | 0 | messages, notification_policies, notification_policy_overrides |
| NotificationEndpoint | app\models\notification_endpoint.rb | 0 | access_token |
| NotificationFailureProcessor | app\models\notification_failure_processor.rb | 1 | none |
| NotificationFinder | app\models\notification_finder.rb | 1 | none |
| NotificationPolicy | app\models\notification_policy.rb | 0 | communication_channel, delayed_messages |
| NotificationPolicyOverride | app\models\notification_policy_override.rb | 0 | communication_channel, context, notification, delayed_messages |
| NotificationPreloader | app\models\notification_preloader.rb | 0 | notification |
| Notifier | app\models\notifier.rb | 0 | none |
| OauthRequest | app\models\oauth_request.rb | 0 | user |
| ObserverAlert | app\models\observer_alert.rb | 0 | student, observer, observer_alert_threshold, context |
| ObserverAlertThreshold | app\models\observer_alert_threshold.rb | 0 | student, observer, observer_alerts |
| ObserverEnrollment | app\models\observer_enrollment.rb | 0 | none |
| ObserverPairingCode | app\models\observer_pairing_code.rb | 0 | user |
| OneTimePassword | app\models\one_time_password.rb | 0 | user |
| OriginalityReport | app\models\originality_report.rb | 0 | submission, attachment, originality_report_attachment, root_account, lti_link |
| OutcomeCalculationMethod | app\models\outcome_calculation_method.rb | 0 | context |
| OutcomeFriendlyDescription | app\models\outcome_friendly_description.rb | 0 | learning_outcome, context |
| OutcomeImport | app\models\outcome_import.rb | 0 | context, attachment, user, outcome_import_errors |
| OutcomeImportContext | app\models\outcome_import_context.rb | 0 | outcome_imports, latest_outcome_import |
| OutcomeImportError | app\models\outcome_import_error.rb | 0 | outcome_import |
| OutcomeProficiency | app\models\outcome_proficiency.rb | 0 | outcome_proficiency_ratings, context |
| OutcomeProficiencyRating | app\models\outcome_proficiency_rating.rb | 0 | outcome_proficiency |
| PageComment | app\models\page_comment.rb | 0 | page, user |
| PageView | app\models\page_view.rb | 1 | developer_key, user, account, real_user, asset_user_access, context |
| ParallelImporter | app\models\parallel_importer.rb | 0 | sis_batch, attachment, sis_batch_errors |
| PlannerNote | app\models\planner_note.rb | 0 | user, course, linked_object |
| PlannerOverride | app\models\planner_override.rb | 0 | plannable, user |
| PluginSetting | app\models\plugin_setting.rb | 2 | none |
| Polling | app\models\polling.rb | 0 | none |
| PostPolicy | app\models\post_policy.rb | 0 | course, assignment |
| Profile | app\models\profile.rb | 0 | context, root_account, profile |
| Progress | app\models\progress.rb | 1 | context, user, delayed_job |
| Pseudonym | app\models\pseudonym.rb | 3 | session_persistence_tokens, account, user, communication_channels, sis_enrollments, auditor_authentication_records, auditor_records, communication_channel, sis_communication_channel, authentication_provider |
| PseudonymSession | app\models\pseudonym_session.rb | 3 | none |
| Purgatory | app\models\purgatory.rb | 0 | attachment, deleted_by_user |
| Quizzes | app\models\quizzes.rb | 0 | none |
| QuizMigrationAlert | app\models\quiz_migration_alert.rb | 0 | user, course, migration |
| ReceivedContentShare | app\models\received_content_share.rb | 0 | sender |
| ReleaseNote | app\models\release_note.rb | 1 | none |
| ReportSnapshot | app\models\report_snapshot.rb | 0 | account |
| Role | app\models\role.rb | 0 | account, root_account, role_overrides |
| RoleOverride | app\models\role_override.rb | 1 | context, role |
| RollupScore | app\models\rollup_score.rb | 1 | none |
| RootAccountResolver | app\models\root_account_resolver.rb | 0 | root_account |
| Rubric | app\models\rubric.rb | 2 | user, rubric, context, rubric_associations, rubric_associations_with_deleted, rubric_assessments, learning_outcome_alignments, learning_outcome_results, rubric_criteria |
| RubricAssessment | app\models\rubric_assessment.rb | 0 | rubric, rubric_association, user, assessor, artifact, assessment_requests, learning_outcome_results |
| RubricAssessmentExport | app\models\rubric_assessment_export.rb | 1 | none |
| RubricAssessmentImport | app\models\rubric_assessment_import.rb | 0 | course, assignment, attachment, root_account, user |
| RubricAssociation | app\models\rubric_association.rb | 2 | rubric, association_object, context, rubric_assessments, assessment_requests |
| RubricCriterion | app\models\rubric_criterion.rb | 0 | rubric, learning_outcome, created_by, deleted_by |
| RubricImport | app\models\rubric_import.rb | 0 | account, course, attachment, root_account, user |
| ScheduledPublication | app\models\scheduled_publication.rb | 0 | none |
| ScheduledSmartAlert | app\models\scheduled_smart_alert.rb | 0 | none |
| Score | app\models\score.rb | 0 | enrollment, grading_period, assignment_group, custom_grade_status, course, score_metadata |
| ScoreMetadata | app\models\score_metadata.rb | 0 | score |
| ScoreStatistic | app\models\score_statistic.rb | 0 | assignment |
| SentContentShare | app\models\sent_content_share.rb | 0 | received_content_shares, receivers |
| SessionPersistenceToken | app\models\session_persistence_token.rb | 1 | pseudonym |
| Setting | app\models\setting.rb | 0 | none |
| ShardedBookmarkedCollection | app\models\sharded_bookmarked_collection.rb | 0 | none |
| SharedBrandConfig | app\models\shared_brand_config.rb | 0 | brand_config, account |
| SisBatch | app\models\sis_batch.rb | 2 | account, attachment, errors_attachment, parallel_importers, sis_batch_errors, roll_back_data, progresses, generated_diff, batch_mode_term, user, auditor_course_records |
| SisBatchError | app\models\sis_batch_error.rb | 0 | sis_batch, parallel_importer, root_account |
| SisBatchRollBackData | app\models\sis_batch_roll_back_data.rb | 0 | sis_batch, context |
| SisPostGradesStatus | app\models\sis_post_grades_status.rb | 0 | course, course_section, user |
| SisPseudonym | app\models\sis_pseudonym.rb | 1 | none |
| SplitUsers | app\models\split_users.rb | 1 | none |
| StandardGradeStatus | app\models\standard_grade_status.rb | 0 | root_account |
| StreamItem | app\models\stream_item.rb | 1 | stream_item_instances, users, context, asset |
| StreamItemInstance | app\models\stream_item_instance.rb | 0 | user, stream_item, context |
| StudentEnrollment | app\models\student_enrollment.rb | 0 | student, course_paces |
| StudentViewEnrollment | app\models\student_view_enrollment.rb | 0 | none |
| Submission | app\models\submission.rb | 6 | attachment, assignment, course, custom_grade_status, observer_alerts, lti_assets, user, grader, proxy_submitter, grading_period, group, media_object, root_account, quiz_submission, all_submission_comments, all_submission_comments_for_groups, group_memberships, submission_comments, visible_submission_comments, hidden_submission_comments, assessment_requests, assigned_assessments, rubric_assessments, attachment_associations, provisional_grades, originality_reports, rubric_assessment, lti_result, submission_drafts, conversation_messages, content_participations, canvadocs_annotation_contexts, canvadocs_submissions, auditor_grade_change_records |
| SubmissionComment | app\models\submission_comment.rb | 2 | root_account, submission, author, assessment_request, context, provisional_grade, messages, viewed_submission_comments |
| SubmissionCommentInteraction | app\models\submission_comment_interaction.rb | 0 | none |
| SubmissionDraft | app\models\submission_draft.rb | 0 | submission, media_object, submission_draft_attachments, attachments |
| SubmissionDraftAttachment | app\models\submission_draft_attachment.rb | 0 | submission_draft, attachment |
| SubmissionVersion | app\models\submission_version.rb | 0 | assignment, context, root_account, version |
| SubAssignment | app\models\sub_assignment.rb | 0 | discussion_topic |
| TaEnrollment | app\models\ta_enrollment.rb | 0 | none |
| TeacherEnrollment | app\models\teacher_enrollment.rb | 0 | none |
| TemporaryEnrollmentPairing | app\models\temporary_enrollment_pairing.rb | 0 | root_account, created_by, deleted_by, enrollments |
| TermsOfService | app\models\terms_of_service.rb | 1 | account, terms_of_service_content |
| TermsOfServiceContent | app\models\terms_of_service_content.rb | 0 | account |
| Thumbnail | app\models\thumbnail.rb | 0 | attachment |
| UsageRights | app\models\usage_rights.rb | 0 | context |
| User | app\models\user.rb | 4 | communication_channels, notification_policies, notification_policy_overrides, communication_channel, ignores, planner_notes, viewed_submission_comments, enrollments, course_paces, course_reports, not_ended_enrollments, not_removed_enrollments, observer_enrollments, observee_enrollments, observer_pairing_codes, as_student_observation_links, as_observer_observation_links, as_student_observer_alert_thresholds, as_student_observer_alerts, as_observer_observer_alert_thresholds, as_observer_observer_alerts, linked_observers, linked_students, all_courses, all_courses_for_active_enrollments, polls, group_memberships, current_group_memberships, groups, current_groups, differentiation_tag_memberships, current_differentiation_tag_memberships, differentiation_tags, current_differentiation_tags, user_account_associations, unordered_associated_accounts, associated_accounts, associated_root_accounts, developer_keys, access_tokens, masquerade_tokens, notification_endpoints, context_external_tools, lti_results, student_enrollments, ta_enrollments, teacher_enrollments, all_submissions, submissions, pseudonyms, active_pseudonyms, pseudonym_accounts, pseudonym, attachments, active_images, active_assignments, mentions, discussion_entries, discussion_entry_drafts, discussion_entry_versions, all_attachments, folders, submissions_folders, active_folders, calendar_events, eportfolios, quiz_submissions, dashboard_messages, user_services, rubric_associations, rubrics, context_rubrics, grading_standards, context_module_progressions, assessment_question_bank_users, assessment_question_banks, learning_outcome_results, collaborators, collaborations, assigned_submission_assessments, assigned_assessments, web_conference_participants, web_conferences, account_users, media_objects, user_generated_media_objects, content_shares, received_content_shares, sent_content_shares, account_reports, stream_item_instances, all_conversations, conversation_batches, favorites, messages, sis_batches, sis_post_grades_statuses, content_migrations, content_exports, usage_rights, gradebook_csvs, block_editor_templates, asset_user_accesses, profile, progresses, one_time_passwords, past_lti_ids, user_preference_values, auditor_authentication_records, auditor_course_records, auditor_student_grade_change_records, auditor_grader_grade_change_records, auditor_feature_flag_records, created_lti_registrations, updated_lti_registrations, created_lti_registration_account_bindings, updated_lti_registration_account_bindings, lti_overlays, lti_overlay_versions, lti_asset_processor_eula_acceptances, comment_bank_items, microsoft_sync_partial_sync_changes, gradebook_filters, quiz_migration_alerts, custom_data, otp_communication_channel, merged_into_user |
| UserAccountAssociation | app\models\user_account_association.rb | 0 | user, account |
| UserLearningObjectScopes | app\models\user_learning_object_scopes.rb | 0 | none |
| UserLmgbOutcomeOrderings | app\models\user_lmgb_outcome_orderings.rb | 0 | user, course, learning_outcome |
| UserMergeData | app\models\user_merge_data.rb | 0 | user, from_user, records, items |
| UserMergeDataItem | app\models\user_merge_data_item.rb | 0 | user, merge_data |
| UserMergeDataRecord | app\models\user_merge_data_record.rb | 0 | previous_user, merge_data, context |
| UserObservationLink | app\models\user_observation_link.rb | 1 | student, observer, root_account |
| UserObserver | app\models\user_observer.rb | 0 | none |
| UserPastLtiId | app\models\user_past_lti_id.rb | 0 | user, context |
| UserPreferenceValue | app\models\user_preference_value.rb | 0 | user |
| UserProfile | app\models\user_profile.rb | 0 | user, links |
| UserProfileLink | app\models\user_profile_link.rb | 0 | user_profile |
| UserService | app\models\user_service.rb | 1 | user |
| Version | app\models\version.rb | 0 | none |
| ViewedSubmissionComment | app\models\viewed_submission_comment.rb | 0 | submission_comment, user |
| WebConference | app\models\web_conference.rb | 3 | context, calendar_event, web_conference_participants, users, invitees, attendees, user |
| WebConferenceParticipant | app\models\web_conference_participant.rb | 0 | web_conference, user |
| WebZipExport | app\models\web_zip_export.rb | 0 | none |
| Wiki | app\models\wiki.rb | 0 | wiki_pages, course, group, root_account |
| WikiPage | app\models\wiki_page.rb | 4 | wiki, user, context, root_account, current_lookup, wiki_page_lookups, master_content_tag, block_editor, estimated_duration |
| WikiPageEmbedding | app\models\wiki_page_embedding.rb | 0 | wiki_page |
| WikiPageLookup | app\models\wiki_page_lookup.rb | 0 | wiki_page, context |
| WimbaConference | app\models\wimba_conference.rb | 0 | none |

## Controllers

| Controller | File | Actions | Routes |
|------------|------|---------|--------|
| AccountsController | app\controllers\accounts_controller.rb | 49 | /accounts/index, /accounts/show, /accounts/update |
| AccountCalendarsApiController | app\controllers\account_calendars_api_controller.rb | 6 | /accountcalendarsapi/index, /accountcalendarsapi/show, /accountcalendarsapi/update |
| AccountGradingSettingsController | app\controllers\account_grading_settings_controller.rb | 1 | /accountgradingsettings/index |
| AccountNotificationsController | app\controllers\account_notifications_controller.rb | 18 | /accountnotifications/show, /accountnotifications/create, /accountnotifications/update, /accountnotifications/destroy |
| AccountReportsController | app\controllers\account_reports_controller.rb | 8 | /accountreports/create, /accountreports/index, /accountreports/show, /accountreports/destroy |
| AdminsController | app\controllers\admins_controller.rb | 5 | /admins/create, /admins/destroy, /admins/index |
| AlertsController | app\controllers\alerts_controller.rb | 5 | /alerts/create, /alerts/update, /alerts/destroy |
| AnalyticsHubController | app\controllers\analytics_hub_controller.rb | 2 | /analyticshub/show |
| AnnouncementsApiController | app\controllers\announcements_api_controller.rb | 3 | /announcementsapi/index |
| AnnouncementsController | app\controllers\announcements_controller.rb | 5 | /announcements/index, /announcements/show |
| AnonymousProvisionalGradesController | app\controllers\anonymous_provisional_grades_controller.rb | 1 | none |
| AnonymousSubmissionsController | app\controllers\anonymous_submissions_controller.rb | 6 | /anonymoussubmissions/show, /anonymoussubmissions/update |
| ApplicationController | app\controllers\application_controller.rb | 211 | none |
| AppointmentGroupsController | app\controllers\appointment_groups_controller.rb | 16 | /appointmentgroups/index, /appointmentgroups/create, /appointmentgroups/show, /appointmentgroups/edit, /appointmentgroups/update, /appointmentgroups/destroy |
| AppCenterController | app\controllers\app_center_controller.rb | 4 | /appcenter/index |
| AssessmentQuestionsController | app\controllers\assessment_questions_controller.rb | 5 | /assessmentquestions/create, /assessmentquestions/update, /assessmentquestions/destroy |
| AssignmentsApiController | app\controllers\assignments_api_controller.rb | 24 | /assignmentsapi/index, /assignmentsapi/show, /assignmentsapi/create, /assignmentsapi/update |
| AssignmentsController | app\controllers\assignments_controller.rb | 43 | /assignments/index, /assignments/show, /assignments/create, /assignments/new, /assignments/edit, /assignments/destroy |
| AssignmentExtensionsController | app\controllers\assignment_extensions_controller.rb | 2 | /assignmentextensions/create |
| AssignmentGroupsApiController | app\controllers\assignment_groups_api_controller.rb | 9 | /assignmentgroupsapi/show, /assignmentgroupsapi/create, /assignmentgroupsapi/update, /assignmentgroupsapi/destroy |
| AssignmentGroupsController | app\controllers\assignment_groups_controller.rb | 23 | /assignmentgroups/index, /assignmentgroups/show, /assignmentgroups/create, /assignmentgroups/update, /assignmentgroups/destroy |
| AssignmentOverridesController | app\controllers\assignment_overrides_controller.rb | 19 | /assignmentoverrides/index, /assignmentoverrides/show, /assignmentoverrides/create, /assignmentoverrides/update, /assignmentoverrides/destroy |
| AuditorApiController | app\controllers\auditor_api_controller.rb | 1 | none |
| AuthenticationAuditApiController | app\controllers\authentication_audit_api_controller.rb | 5 | none |
| AuthenticationProvidersController | app\controllers\authentication_providers_controller.rb | 23 | /authenticationproviders/index, /authenticationproviders/show, /authenticationproviders/create, /authenticationproviders/update, /authenticationproviders/destroy |
| BlackoutDatesController | app\controllers\blackout_dates_controller.rb | 9 | /blackoutdates/index, /blackoutdates/show, /blackoutdates/new, /blackoutdates/create, /blackoutdates/update, /blackoutdates/destroy |
| BlockEditorsController | app\controllers\block_editors_controller.rb | 2 | /blockeditors/show |
| BlockEditorTemplatesApiController | app\controllers\block_editor_templates_api_controller.rb | 9 | /blockeditortemplatesapi/index, /blockeditortemplatesapi/create, /blockeditortemplatesapi/update, /blockeditortemplatesapi/destroy |
| BookmarksController | app\controllers\bookmarks\bookmarks_controller.rb | 11 | /bookmarks/index, /bookmarks/create, /bookmarks/show, /bookmarks/update, /bookmarks/destroy |
| BrandConfigsApiController | app\controllers\brand_configs_api_controller.rb | 1 | /brandconfigsapi/show |
| BrandConfigsController | app\controllers\brand_configs_controller.rb | 16 | /brandconfigs/index, /brandconfigs/new, /brandconfigs/show, /brandconfigs/create, /brandconfigs/destroy |
| CalendarsController | app\controllers\calendars_controller.rb | 1 | /calendars/show |
| CalendarEventsApiController | app\controllers\calendar_events_api_controller.rb | 51 | /calendareventsapi/index, /calendareventsapi/create, /calendareventsapi/show, /calendareventsapi/update, /calendareventsapi/destroy |
| CalendarEventsController | app\controllers\calendar_events_controller.rb | 9 | /calendarevents/show, /calendarevents/new, /calendarevents/create, /calendarevents/edit, /calendarevents/update, /calendarevents/destroy |
| CanvadocSessionsController | app\controllers\canvadoc_sessions_controller.rb | 3 | /canvadocsessions/create, /canvadocsessions/show |
| CollaborationsController | app\controllers\collaborations_controller.rb | 14 | /collaborations/index, /collaborations/show, /collaborations/create, /collaborations/update, /collaborations/destroy |
| CommunicationChannelsController | app\controllers\communication_channels_controller.rb | 21 | /communicationchannels/index, /communicationchannels/create, /communicationchannels/destroy |
| CommMessagesApiController | app\controllers\comm_messages_api_controller.rb | 1 | /commmessagesapi/index |
| GradingSchemeSerializer.RbController | app\controllers\concerns\grading_scheme_serializer.rb | 5 | none |
| GranularPermissionEnforcement.RbController | app\controllers\concerns\granular_permission_enforcement.rb | 1 | none |
| HorizonMode.RbController | app\controllers\concerns\horizon_mode.rb | 1 | none |
| K5Mode.RbController | app\controllers\concerns\k5_mode.rb | 2 | none |
| ApiToNestedAttributes.RbController | app\controllers\conditional_release\concerns\api_to_nested_attributes.rb | 1 | none |
| PermittedApiParameters.RbController | app\controllers\conditional_release\concerns\permitted_api_parameters.rb | 18 | none |
| RulesController | app\controllers\conditional_release\rules_controller.rb | 14 | /rules/index, /rules/show, /rules/create, /rules/update, /rules/destroy |
| StatsController | app\controllers\conditional_release\stats_controller.rb | 5 | none |
| ConferencesController | app\controllers\conferences_controller.rb | 23 | /conferences/index, /conferences/show, /conferences/create, /conferences/update, /conferences/destroy |
| ContentExportsApiController | app\controllers\content_exports_api_controller.rb | 8 | /contentexportsapi/index, /contentexportsapi/show, /contentexportsapi/create, /contentexportsapi/update |
| ContentExportsController | app\controllers\content_exports_controller.rb | 7 | /contentexports/index, /contentexports/show, /contentexports/create, /contentexports/destroy |
| ContentImportsController | app\controllers\content_imports_controller.rb | 7 | /contentimports/index |
| ContentMigrationsController | app\controllers\content_migrations_controller.rb | 14 | /contentmigrations/index, /contentmigrations/show, /contentmigrations/create, /contentmigrations/update |
| ContentSharesController | app\controllers\content_shares_controller.rb | 11 | /contentshares/create, /contentshares/index, /contentshares/show, /contentshares/destroy, /contentshares/update |
| ContextController | app\controllers\context_controller.rb | 10 | none |
| ContextModulesApiController | app\controllers\context_modules_api_controller.rb | 10 | /contextmodulesapi/index, /contextmodulesapi/show, /contextmodulesapi/create, /contextmodulesapi/update, /contextmodulesapi/destroy |
| ContextModulesController | app\controllers\context_modules_controller.rb | 39 | /contextmodules/index, /contextmodules/create, /contextmodules/show, /contextmodules/update, /contextmodules/destroy |
| ContextModuleItemsApiController | app\controllers\context_module_items_api_controller.rb | 18 | /contextmoduleitemsapi/index, /contextmoduleitemsapi/show, /contextmoduleitemsapi/create, /contextmoduleitemsapi/update, /contextmoduleitemsapi/destroy |
| ConversationsController | app\controllers\conversations_controller.rb | 24 | /conversations/index, /conversations/create, /conversations/show, /conversations/update, /conversations/destroy |
| CoursesController | app\controllers\courses_controller.rb | 99 | /courses/index, /courses/create, /courses/destroy, /courses/show, /courses/update |
| CourseAuditApiController | app\controllers\course_audit_api_controller.rb | 4 | none |
| CourseNicknamesController | app\controllers\course_nicknames_controller.rb | 6 | /coursenicknames/index, /coursenicknames/show, /coursenicknames/update |
| CoursePacesController | app\controllers\course_paces_controller.rb | 27 | /coursepaces/index, /coursepaces/new, /coursepaces/create, /coursepaces/update, /coursepaces/destroy |
| BulkStudentEnrollmentPacesApiController | app\controllers\course_pacing\bulk_student_enrollment_paces_api_controller.rb | 12 | none |
| PacesApiController | app\controllers\course_pacing\paces_api_controller.rb | 11 | /pacesapi/show, /pacesapi/create, /pacesapi/update |
| PaceContextsApiController | app\controllers\course_pacing\pace_contexts_api_controller.rb | 5 | /pacecontextsapi/index |
| SectionPacesApiController | app\controllers\course_pacing\section_paces_api_controller.rb | 4 | none |
| StudentEnrollmentPacesApiController | app\controllers\course_pacing\student_enrollment_paces_api_controller.rb | 4 | none |
| CourseReportsController | app\controllers\course_reports_controller.rb | 4 | /coursereports/show, /coursereports/create |
| CrocodocSessionsController | app\controllers\crocodoc_sessions_controller.rb | 1 | /crocodocsessions/show |
| CspSettingsController | app\controllers\csp_settings_controller.rb | 11 | none |
| CustomDataController | app\controllers\custom_data_controller.rb | 6 | none |
| CustomGradebookColumnsApiController | app\controllers\custom_gradebook_columns_api_controller.rb | 7 | /customgradebookcolumnsapi/index, /customgradebookcolumnsapi/create, /customgradebookcolumnsapi/update, /customgradebookcolumnsapi/destroy |
| CustomGradebookColumnDataApiController | app\controllers\custom_gradebook_column_data_api_controller.rb | 5 | /customgradebookcolumndataapi/index, /customgradebookcolumndataapi/update |
| DeveloperKeysController | app\controllers\developer_keys_controller.rb | 13 | /developerkeys/index, /developerkeys/create, /developerkeys/update, /developerkeys/destroy |
| DeveloperKeyAccountBindingsController | app\controllers\developer_key_account_bindings_controller.rb | 11 | none |
| DisablePostToSisApiController | app\controllers\disable_post_to_sis_api_controller.rb | 7 | none |
| DiscussionEntriesController | app\controllers\discussion_entries_controller.rb | 9 | /discussionentries/show, /discussionentries/create, /discussionentries/update, /discussionentries/destroy |
| DiscussionTopicsApiController | app\controllers\discussion_topics_api_controller.rb | 51 | /discussiontopicsapi/show |
| DiscussionTopicsController | app\controllers\discussion_topics_controller.rb | 39 | /discussiontopics/index, /discussiontopics/new, /discussiontopics/edit, /discussiontopics/show, /discussiontopics/create, /discussiontopics/update, /discussiontopics/destroy |
| DiscussionTopicUsersController | app\controllers\discussion_topic_users_controller.rb | 3 | none |
| DocviewerAuditEventsController | app\controllers\docviewer_audit_events_controller.rb | 5 | /docviewerauditevents/create |
| EnrollmentsApiController | app\controllers\enrollments_api_controller.rb | 18 | /enrollmentsapi/index, /enrollmentsapi/show, /enrollmentsapi/create, /enrollmentsapi/destroy |
| EportfoliosApiController | app\controllers\eportfolios_api_controller.rb | 7 | /eportfoliosapi/index, /eportfoliosapi/show |
| EportfoliosController | app\controllers\eportfolios_controller.rb | 15 | /eportfolios/index, /eportfolios/show, /eportfolios/create, /eportfolios/update, /eportfolios/destroy |
| EportfolioCategoriesController | app\controllers\eportfolio_categories_controller.rb | 8 | /eportfoliocategories/index, /eportfoliocategories/create, /eportfoliocategories/update, /eportfoliocategories/show, /eportfoliocategories/destroy |
| EportfolioEntriesController | app\controllers\eportfolio_entries_controller.rb | 8 | /eportfolioentries/create, /eportfolioentries/show, /eportfolioentries/update, /eportfolioentries/destroy |
| EpubExportsController | app\controllers\epub_exports_controller.rb | 4 | /epubexports/index, /epubexports/create, /epubexports/show |
| EquationImagesController | app\controllers\equation_images_controller.rb | 2 | /equationimages/show |
| ErrorsController | app\controllers\errors_controller.rb | 5 | /errors/index, /errors/show, /errors/create |
| ExternalContentController | app\controllers\external_content_controller.rb | 12 | none |
| ExternalFeedsController | app\controllers\external_feeds_controller.rb | 3 | /externalfeeds/index, /externalfeeds/create, /externalfeeds/destroy |
| ExternalToolsController | app\controllers\external_tools_controller.rb | 48 | /externaltools/index, /externaltools/show, /externaltools/create, /externaltools/update, /externaltools/destroy |
| FavoritesController | app\controllers\favorites_controller.rb | 10 | none |
| FeatureFlagsController | app\controllers\feature_flags_controller.rb | 8 | /featureflags/index, /featureflags/show, /featureflags/update |
| FilesController | app\controllers\files_controller.rb | 55 | /files/index, /files/show, /files/update, /files/destroy |
| FilePreviewsController | app\controllers\file_previews_controller.rb | 4 | /filepreviews/show |
| LiveAssessments.RbController | app\controllers\filters\live_assessments.rb | 1 | none |
| Polling.RbController | app\controllers\filters\polling.rb | 3 | none |
| Quizzes.RbController | app\controllers\filters\quizzes.rb | 2 | none |
| QuizSubmissions.RbController | app\controllers\filters\quiz_submissions.rb | 5 | none |
| FoldersController | app\controllers\folders_controller.rb | 18 | /folders/index, /folders/show, /folders/update, /folders/create, /folders/destroy |
| GradebooksController | app\controllers\gradebooks_controller.rb | 82 | /gradebooks/show |
| GradebookCsvsController | app\controllers\gradebook_csvs_controller.rb | 1 | /gradebookcsvs/create |
| GradebookFiltersApiController | app\controllers\gradebook_filters_api_controller.rb | 7 | /gradebookfiltersapi/index, /gradebookfiltersapi/show, /gradebookfiltersapi/create, /gradebookfiltersapi/update, /gradebookfiltersapi/destroy |
| GradebookHistoryApiController | app\controllers\gradebook_history_api_controller.rb | 6 | none |
| GradebookSettingsController | app\controllers\gradebook_settings_controller.rb | 7 | /gradebooksettings/update |
| GradebookUploadsController | app\controllers\gradebook_uploads_controller.rb | 8 | /gradebookuploads/index, /gradebookuploads/new, /gradebookuploads/show, /gradebookuploads/create |
| GradeChangeAuditApiController | app\controllers\grade_change_audit_api_controller.rb | 18 | none |
| GradingPeriodsController | app\controllers\grading_periods_controller.rb | 20 | /gradingperiods/index, /gradingperiods/show, /gradingperiods/update, /gradingperiods/destroy |
| GradingPeriodSetsController | app\controllers\grading_period_sets_controller.rb | 10 | /gradingperiodsets/index, /gradingperiodsets/create, /gradingperiodsets/update, /gradingperiodsets/destroy |
| GradingSchemesJsonController | app\controllers\grading_schemes_json_controller.rb | 25 | /gradingschemesjson/show, /gradingschemesjson/create, /gradingschemesjson/update, /gradingschemesjson/destroy |
| GradingStandardsApiController | app\controllers\grading_standards_api_controller.rb | 4 | /gradingstandardsapi/create |
| GradingStandardsController | app\controllers\grading_standards_controller.rb | 7 | /gradingstandards/index, /gradingstandards/create, /gradingstandards/update, /gradingstandards/destroy |
| GraphqlController | app\controllers\graphql_controller.rb | 9 | none |
| GroupsController | app\controllers\groups_controller.rb | 21 | /groups/index, /groups/show, /groups/new, /groups/create, /groups/update, /groups/destroy |
| GroupCategoriesController | app\controllers\group_categories_controller.rb | 22 | /groupcategories/index, /groupcategories/show, /groupcategories/create, /groupcategories/update, /groupcategories/destroy |
| GroupMembershipsController | app\controllers\group_memberships_controller.rb | 8 | /groupmemberships/index, /groupmemberships/show, /groupmemberships/create, /groupmemberships/update, /groupmemberships/destroy |
| HistoryController | app\controllers\history_controller.rb | 2 | /history/index |
| HorizonController | app\controllers\horizon_controller.rb | 3 | none |
| ImmersiveReaderController | app\controllers\immersive_reader_controller.rb | 7 | none |
| InfoController | app\controllers\info_controller.rb | 14 | none |
| InstAccessTokensController | app\controllers\inst_access_tokens_controller.rb | 2 | /instaccesstokens/create |
| JobsController | app\controllers\jobs_controller.rb | 7 | /jobs/index, /jobs/show |
| JobsV2Controller | app\controllers\jobs_v2_controller.rb | 36 | /jobsv2/index |
| JwtsController | app\controllers\jwts_controller.rb | 20 | /jwts/create |
| LatePolicyController | app\controllers\late_policy_controller.rb | 9 | /latepolicy/show, /latepolicy/create, /latepolicy/update |
| LearningObjectDatesController | app\controllers\learning_object_dates_controller.rb | 15 | /learningobjectdates/show, /learningobjectdates/update |
| LearnPlatformController | app\controllers\learn_platform_controller.rb | 7 | /learnplatform/index, /learnplatform/show |
| LegalInformationController | app\controllers\legal_information_controller.rb | 2 | none |
| AssessmentsController | app\controllers\live_assessments\assessments_controller.rb | 3 | /assessments/create, /assessments/index |
| ResultsController | app\controllers\live_assessments\results_controller.rb | 3 | /results/create, /results/index |
| AppleController | app\controllers\login\apple_controller.rb | 0 | none |
| CanvasController | app\controllers\login\canvas_controller.rb | 9 | /canvas/new, /canvas/create |
| CasController | app\controllers\login\cas_controller.rb | 7 | /cas/new, /cas/create, /cas/destroy |
| CleverController | app\controllers\login\clever_controller.rb | 2 | /clever/create |
| EmailVerifyController | app\controllers\login\email_verify_controller.rb | 2 | /emailverify/show |
| ExternalAuthObserversController | app\controllers\login\external_auth_observers_controller.rb | 3 | none |
| FacebookController | app\controllers\login\facebook_controller.rb | 0 | none |
| GithubController | app\controllers\login\github_controller.rb | 0 | none |
| GoogleController | app\controllers\login\google_controller.rb | 0 | none |
| LdapController | app\controllers\login\ldap_controller.rb | 0 | none |
| LinkedinController | app\controllers\login\linkedin_controller.rb | 0 | none |
| MicrosoftController | app\controllers\login\microsoft_controller.rb | 0 | none |
| Oauth2Controller | app\controllers\login\oauth2_controller.rb | 12 | /oauth2/new, /oauth2/create |
| OauthBaseController | app\controllers\login\oauth_base_controller.rb | 3 | none |
| OauthController | app\controllers\login\oauth_controller.rb | 4 | /oauth/new, /oauth/create |
| OpenidConnectController | app\controllers\login\openid_connect_controller.rb | 5 | /openidconnect/destroy |
| OtpController | app\controllers\login\otp_controller.rb | 9 | /otp/new, /otp/create, /otp/destroy |
| SamlController | app\controllers\login\saml_controller.rb | 13 | /saml/new, /saml/create, /saml/destroy |
| SamlIdpDiscoveryController | app\controllers\login\saml_idp_discovery_controller.rb | 2 | /samlidpdiscovery/new |
| Shared.RbController | app\controllers\login\shared.rb | 11 | none |
| LoginController | app\controllers\login_controller.rb | 8 | /login/new, /login/destroy |
| AccountExternalToolsController | app\controllers\lti\account_external_tools_controller.rb | 11 | /accountexternaltools/create, /accountexternaltools/show, /accountexternaltools/index, /accountexternaltools/destroy |
| AccountLookupController | app\controllers\lti\account_lookup_controller.rb | 4 | /accountlookup/show |
| AssetProcessorLaunchController | app\controllers\lti\asset_processor_launch_controller.rb | 21 | none |
| Oembed.RbController | app\controllers\lti\concerns\oembed.rb | 12 | none |
| ParentFrame.RbController | app\controllers\lti\concerns\parent_frame.rb | 6 | none |
| SessionlessLaunches.RbController | app\controllers\lti\concerns\sessionless_launches.rb | 9 | none |
| DataServicesController | app\controllers\lti\data_services_controller.rb | 14 | /dataservices/create, /dataservices/update, /dataservices/show, /dataservices/index, /dataservices/destroy |
| FeatureFlagsController | app\controllers\lti\feature_flags_controller.rb | 6 | /featureflags/show |
| AccessTokenHelper.RbController | app\controllers\lti\ims\access_token_helper.rb | 9 | none |
| AssetProcessorController | app\controllers\lti\ims\asset_processor_controller.rb | 19 | none |
| AssetProcessorEulaController | app\controllers\lti\ims\asset_processor_eula_controller.rb | 14 | none |
| AuthenticationController | app\controllers\lti\ims\authentication_controller.rb | 21 | none |
| AuthorizationController | app\controllers\lti\ims\authorization_controller.rb | 2 | none |
| AdvantageServices.RbController | app\controllers\lti\ims\concerns\advantage_services.rb | 7 | none |
| DeepLinkingModules.RbController | app\controllers\lti\ims\concerns\deep_linking_modules.rb | 9 | none |
| DeepLinkingServices.RbController | app\controllers\lti\ims\concerns\deep_linking_services.rb | 26 | none |
| GradebookServices.RbController | app\controllers\lti\ims\concerns\gradebook_services.rb | 11 | none |
| LtiServices.RbController | app\controllers\lti\ims\concerns\lti_services.rb | 15 | none |
| DeepLinkingController | app\controllers\lti\ims\deep_linking_controller.rb | 6 | none |
| DynamicRegistrationController | app\controllers\lti\ims\dynamic_registration_controller.rb | 14 | /dynamicregistration/show, /dynamicregistration/create |
| LineItemsController | app\controllers\lti\ims\line_items_controller.rb | 16 | /lineitems/create, /lineitems/update, /lineitems/show, /lineitems/index, /lineitems/destroy |
| NamesAndRolesController | app\controllers\lti\ims\names_and_roles_controller.rb | 8 | none |
| NoticeHandlersController | app\controllers\lti\ims\notice_handlers_controller.rb | 7 | /noticehandlers/index, /noticehandlers/update |
| ProgressController | app\controllers\lti\ims\progress_controller.rb | 5 | /progress/show |
| CourseMembershipsProvider.RbController | app\controllers\lti\ims\providers\course_memberships_provider.rb | 20 | none |
| GroupMembershipsProvider.RbController | app\controllers\lti\ims\providers\group_memberships_provider.rb | 18 | none |
| MembershipsProvider.RbController | app\controllers\lti\ims\providers\memberships_provider.rb | 30 | none |
| ResultsController | app\controllers\lti\ims\results_controller.rb | 7 | /results/index, /results/show |
| ScoresController | app\controllers\lti\ims\scores_controller.rb | 35 | /scores/create |
| ToolConsumerProfileController | app\controllers\lti\ims\tool_consumer_profile_controller.rb | 2 | /toolconsumerprofile/show |
| ToolProxyController | app\controllers\lti\ims\tool_proxy_controller.rb | 7 | /toolproxy/show, /toolproxy/create |
| ToolSettingController | app\controllers\lti\ims\tool_setting_controller.rb | 12 | /toolsetting/show, /toolsetting/update |
| LtiAppsController | app\controllers\lti\lti_apps_controller.rb | 6 | /ltiapps/index |
| MembershipServiceController | app\controllers\lti\membership_service_controller.rb | 6 | none |
| MessageController | app\controllers\lti\message_controller.rb | 20 | none |
| OriginalityReportsApiController | app\controllers\lti\originality_reports_api_controller.rb | 27 | /originalityreportsapi/create, /originalityreportsapi/update, /originalityreportsapi/show |
| PlagiarismAssignmentsApiController | app\controllers\lti\plagiarism_assignments_api_controller.rb | 7 | /plagiarismassignmentsapi/show |
| PlatformStorageController | app\controllers\lti\platform_storage_controller.rb | 12 | none |
| PublicJwkController | app\controllers\lti\public_jwk_controller.rb | 3 | /publicjwk/update |
| RegistrationsController | app\controllers\lti\registrations_controller.rb | 28 | /registrations/index, /registrations/show, /registrations/create, /registrations/update, /registrations/destroy |
| ResourceLinksController | app\controllers\lti\resource_links_controller.rb | 28 | /resourcelinks/index, /resourcelinks/show, /resourcelinks/create, /resourcelinks/update, /resourcelinks/destroy |
| SubmissionsApiController | app\controllers\lti\submissions_api_controller.rb | 12 | /submissionsapi/show |
| SubscriptionsApiController | app\controllers\lti\subscriptions_api_controller.rb | 8 | /subscriptionsapi/create, /subscriptionsapi/destroy, /subscriptionsapi/show, /subscriptionsapi/update, /subscriptionsapi/index |
| SubscriptionsValidator.RbController | app\controllers\lti\subscriptions_validator.rb | 7 | none |
| TokenController | app\controllers\lti\token_controller.rb | 5 | none |
| ToolConfigurationsApiController | app\controllers\lti\tool_configurations_api_controller.rb | 16 | /toolconfigurationsapi/create, /toolconfigurationsapi/update, /toolconfigurationsapi/show, /toolconfigurationsapi/destroy |
| ToolDefaultIconController | app\controllers\lti\tool_default_icon_controller.rb | 2 | /tooldefaulticon/show |
| ToolProxyController | app\controllers\lti\tool_proxy_controller.rb | 7 | /toolproxy/destroy, /toolproxy/update |
| UsersApiController | app\controllers\lti\users_api_controller.rb | 7 | /usersapi/show |
| LtiApiController | app\controllers\lti_api_controller.rb | 9 | none |
| MasterTemplatesController | app\controllers\master_courses\master_templates_controller.rb | 24 | /mastertemplates/show |
| MediaObjectsController | app\controllers\media_objects_controller.rb | 10 | /mediaobjects/show, /mediaobjects/index |
| MediaTracksController | app\controllers\media_tracks_controller.rb | 9 | /mediatracks/index, /mediatracks/create, /mediatracks/show, /mediatracks/destroy, /mediatracks/update |
| MessagesController | app\controllers\messages_controller.rb | 5 | /messages/index, /messages/show, /messages/create |
| GroupsController | app\controllers\microsoft_sync\groups_controller.rb | 15 | /groups/create, /groups/show, /groups/destroy |
| MigrationIssuesController | app\controllers\migration_issues_controller.rb | 4 | /migrationissues/index, /migrationissues/show, /migrationissues/update |
| ModerationSetController | app\controllers\moderation_set_controller.rb | 5 | /moderationset/index, /moderationset/create |
| ModuleAssignmentOverridesController | app\controllers\module_assignment_overrides_controller.rb | 9 | /moduleassignmentoverrides/index |
| NotificationPreferencesController | app\controllers\notification_preferences_controller.rb | 10 | /notificationpreferences/index, /notificationpreferences/show, /notificationpreferences/update |
| Oauth2ProviderController | app\controllers\oauth2_provider_controller.rb | 10 | /oauth2provider/destroy |
| OauthProxyController | app\controllers\oauth_proxy_controller.rb | 1 | none |
| ObserverAlertsApiController | app\controllers\observer_alerts_api_controller.rb | 3 | /observeralertsapi/update |
| ObserverAlertThresholdsApiController | app\controllers\observer_alert_thresholds_api_controller.rb | 6 | /observeralertthresholdsapi/index, /observeralertthresholdsapi/show, /observeralertthresholdsapi/create, /observeralertthresholdsapi/update, /observeralertthresholdsapi/destroy |
| ObserverPairingCodesApiController | app\controllers\observer_pairing_codes_api_controller.rb | 2 | /observerpairingcodesapi/create |
| OneTimePasswordsController | app\controllers\one_time_passwords_controller.rb | 4 | /onetimepasswords/index |
| OutcomesAcademicBenchmarkImportApiController | app\controllers\outcomes_academic_benchmark_import_api_controller.rb | 11 | /outcomesacademicbenchmarkimportapi/create |
| OutcomesApiController | app\controllers\outcomes_api_controller.rb | 7 | /outcomesapi/show, /outcomesapi/update |
| OutcomesController | app\controllers\outcomes_controller.rb | 16 | /outcomes/index, /outcomes/show, /outcomes/create, /outcomes/update, /outcomes/destroy |
| OutcomeGroupsApiController | app\controllers\outcome_groups_api_controller.rb | 19 | /outcomegroupsapi/index, /outcomegroupsapi/show, /outcomegroupsapi/update, /outcomegroupsapi/destroy, /outcomegroupsapi/create |
| OutcomeGroupsController | app\controllers\outcome_groups_controller.rb | 4 | /outcomegroups/create, /outcomegroups/update, /outcomegroups/destroy |
| OutcomeImportsApiController | app\controllers\outcome_imports_api_controller.rb | 7 | /outcomeimportsapi/create, /outcomeimportsapi/show |
| OutcomeProficiencyApiController | app\controllers\outcome_proficiency_api_controller.rb | 4 | /outcomeproficiencyapi/create, /outcomeproficiencyapi/show |
| OutcomeResultsController | app\controllers\outcome_results_controller.rb | 40 | /outcomeresults/index |
| PageCommentsController | app\controllers\page_comments_controller.rb | 2 | /pagecomments/create, /pagecomments/destroy |
| PageViewsController | app\controllers\page_views_controller.rb | 2 | /pageviews/index, /pageviews/update |
| PeerReviewsApiController | app\controllers\peer_reviews_api_controller.rb | 5 | /peerreviewsapi/index, /peerreviewsapi/create, /peerreviewsapi/destroy |
| PlannerController | app\controllers\planner_controller.rb | 27 | /planner/index |
| PlannerNotesController | app\controllers\planner_notes_controller.rb | 6 | /plannernotes/index, /plannernotes/show, /plannernotes/update, /plannernotes/create, /plannernotes/destroy |
| PlannerOverridesController | app\controllers\planner_overrides_controller.rb | 5 | /planneroverrides/index, /planneroverrides/show, /planneroverrides/update, /planneroverrides/create, /planneroverrides/destroy |
| PluginsController | app\controllers\plugins_controller.rb | 7 | /plugins/index, /plugins/show, /plugins/update |
| PollsController | app\controllers\polling\polls_controller.rb | 8 | /polls/index, /polls/show, /polls/create, /polls/update, /polls/destroy |
| PollChoicesController | app\controllers\polling\poll_choices_controller.rb | 8 | /pollchoices/index, /pollchoices/show, /pollchoices/create, /pollchoices/update, /pollchoices/destroy |
| PollSessionsController | app\controllers\polling\poll_sessions_controller.rb | 12 | /pollsessions/index, /pollsessions/show, /pollsessions/create, /pollsessions/update, /pollsessions/destroy |
| PollSubmissionsController | app\controllers\polling\poll_submissions_controller.rb | 3 | /pollsubmissions/show, /pollsubmissions/create |
| ProfileController | app\controllers\profile_controller.rb | 16 | /profile/show, /profile/update |
| ProgressController | app\controllers\progress_controller.rb | 2 | /progress/show |
| ProvisionalGradesBaseController | app\controllers\provisional_grades_base_controller.rb | 3 | none |
| ProvisionalGradesController | app\controllers\provisional_grades_controller.rb | 4 | none |
| PseudonymsController | app\controllers\pseudonyms_controller.rb | 18 | /pseudonyms/index, /pseudonyms/show, /pseudonyms/new, /pseudonyms/create, /pseudonyms/edit, /pseudonyms/update, /pseudonyms/destroy |
| PseudonymSessionsController | app\controllers\pseudonym_sessions_controller.rb | 1 | none |
| QuestionBanksController | app\controllers\question_banks_controller.rb | 10 | /questionbanks/index, /questionbanks/show, /questionbanks/create, /questionbanks/update, /questionbanks/destroy |
| CourseQuizExtensionsController | app\controllers\quizzes\course_quiz_extensions_controller.rb | 5 | /coursequizextensions/create |
| OutstandingQuizSubmissionsController | app\controllers\quizzes\outstanding_quiz_submissions_controller.rb | 2 | /outstandingquizsubmissions/index |
| QuizzesApiController | app\controllers\quizzes\quizzes_api_controller.rb | 12 | /quizzesapi/index, /quizzesapi/show, /quizzesapi/create, /quizzesapi/update, /quizzesapi/destroy |
| QuizzesController | app\controllers\quizzes\quizzes_controller.rb | 50 | /quizzes/index, /quizzes/show, /quizzes/new, /quizzes/edit, /quizzes/create, /quizzes/update, /quizzes/destroy |
| QuizAssignmentOverridesController | app\controllers\quizzes\quiz_assignment_overrides_controller.rb | 4 | /quizassignmentoverrides/index |
| QuizExtensionsController | app\controllers\quizzes\quiz_extensions_controller.rb | 4 | /quizextensions/create |
| QuizGroupsController | app\controllers\quizzes\quiz_groups_controller.rb | 6 | /quizgroups/show, /quizgroups/create, /quizgroups/update, /quizgroups/destroy |
| QuizIpFiltersController | app\controllers\quizzes\quiz_ip_filters_controller.rb | 1 | /quizipfilters/index |
| QuizQuestionsController | app\controllers\quizzes\quiz_questions_controller.rb | 13 | /quizquestions/index, /quizquestions/show, /quizquestions/create, /quizquestions/update, /quizquestions/destroy |
| QuizReportsController | app\controllers\quizzes\quiz_reports_controller.rb | 9 | /quizreports/index, /quizreports/create, /quizreports/show |
| QuizStatisticsController | app\controllers\quizzes\quiz_statistics_controller.rb | 4 | /quizstatistics/index |
| QuizSubmissionsApiController | app\controllers\quizzes\quiz_submissions_api_controller.rb | 12 | /quizsubmissionsapi/index, /quizsubmissionsapi/show, /quizsubmissionsapi/create, /quizsubmissionsapi/update |
| QuizSubmissionsController | app\controllers\quizzes\quiz_submissions_controller.rb | 11 | /quizsubmissions/index, /quizsubmissions/create, /quizsubmissions/update, /quizsubmissions/show |
| QuizSubmissionEventsApiController | app\controllers\quizzes\quiz_submission_events_api_controller.rb | 2 | /quizsubmissioneventsapi/create, /quizsubmissioneventsapi/index |
| QuizSubmissionEventsController | app\controllers\quizzes\quiz_submission_events_controller.rb | 1 | /quizsubmissionevents/index |
| QuizSubmissionFilesController | app\controllers\quizzes\quiz_submission_files_controller.rb | 1 | /quizsubmissionfiles/create |
| QuizSubmissionQuestionsController | app\controllers\quizzes\quiz_submission_questions_controller.rb | 13 | /quizsubmissionquestions/index |
| QuizSubmissionUsersController | app\controllers\quizzes\quiz_submission_users_controller.rb | 11 | /quizsubmissionusers/index |
| QuizzesApiController | app\controllers\quizzes_next\quizzes_api_controller.rb | 3 | /quizzesapi/index |
| ReleaseNotesController | app\controllers\release_notes_controller.rb | 20 | /releasenotes/index, /releasenotes/create, /releasenotes/update, /releasenotes/destroy |
| RichContentApiController | app\controllers\rich_content_api_controller.rb | 2 | none |
| RoleOverridesController | app\controllers\role_overrides_controller.rb | 13 | /roleoverrides/index, /roleoverrides/show, /roleoverrides/update, /roleoverrides/create |
| RubricsApiController | app\controllers\rubrics_api_controller.rb | 16 | /rubricsapi/index, /rubricsapi/show |
| RubricsController | app\controllers\rubrics_controller.rb | 13 | /rubrics/index, /rubrics/show, /rubrics/create, /rubrics/update, /rubrics/destroy |
| RubricAssessmentsController | app\controllers\rubric_assessments_controller.rb | 8 | /rubricassessments/create, /rubricassessments/update, /rubricassessments/destroy |
| RubricAssessmentImportsController | app\controllers\rubric_assessment_imports_controller.rb | 3 | /rubricassessmentimports/show, /rubricassessmentimports/create |
| RubricAssociationsController | app\controllers\rubric_associations_controller.rb | 5 | /rubricassociations/create, /rubricassociations/update, /rubricassociations/destroy |
| ScopesApiController | app\controllers\scopes_api_controller.rb | 1 | /scopesapi/index |
| SearchController | app\controllers\search_controller.rb | 3 | none |
| SectionsController | app\controllers\sections_controller.rb | 11 | /sections/index, /sections/create, /sections/update, /sections/show, /sections/destroy |
| SecurityController | app\controllers\security_controller.rb | 8 | none |
| SelfEnrollmentsController | app\controllers\self_enrollments_controller.rb | 2 | /selfenrollments/new |
| ServicesApiController | app\controllers\services_api_controller.rb | 3 | none |
| SharedBrandConfigsController | app\controllers\shared_brand_configs_controller.rb | 5 | /sharedbrandconfigs/create, /sharedbrandconfigs/update, /sharedbrandconfigs/destroy |
| SisApiController | app\controllers\sis_api_controller.rb | 10 | none |
| SisImportsApiController | app\controllers\sis_imports_api_controller.rb | 11 | /sisimportsapi/index, /sisimportsapi/create, /sisimportsapi/show |
| SisImportErrorsApiController | app\controllers\sis_import_errors_api_controller.rb | 2 | /sisimporterrorsapi/index |
| SmartSearchController | app\controllers\smart_search_controller.rb | 5 | /smartsearch/show |
| AbstractSubmissionForShow.RbController | app\controllers\submissions\abstract_submission_for_show.rb | 5 | none |
| AnonymousDownloadsController | app\controllers\submissions\anonymous_downloads_controller.rb | 1 | /anonymousdownloads/show |
| AnonymousPreviewsController | app\controllers\submissions\anonymous_previews_controller.rb | 1 | /anonymouspreviews/show |
| AnonymousSubmissionForShow.RbController | app\controllers\submissions\anonymous_submission_for_show.rb | 3 | none |
| AttachmentForSubmissionDownload.RbController | app\controllers\submissions\attachment_for_submission_download.rb | 10 | none |
| DownloadsBaseController | app\controllers\submissions\downloads_base_controller.rb | 3 | /downloadsbase/show |
| DownloadsController | app\controllers\submissions\downloads_controller.rb | 1 | /downloads/show |
| PreviewsBaseController | app\controllers\submissions\previews_base_controller.rb | 10 | /previewsbase/show |
| PreviewsController | app\controllers\submissions\previews_controller.rb | 1 | /previews/show |
| ShowHelper.RbController | app\controllers\submissions\show_helper.rb | 2 | none |
| SubmissionForShow.RbController | app\controllers\submissions\submission_for_show.rb | 3 | none |
| SubmissionsApiController | app\controllers\submissions_api_controller.rb | 28 | /submissionsapi/index, /submissionsapi/show, /submissionsapi/update |
| SubmissionsBaseController | app\controllers\submissions_base_controller.rb | 14 | /submissionsbase/show, /submissionsbase/update |
| SubmissionsController | app\controllers\submissions_controller.rb | 23 | /submissions/index, /submissions/show, /submissions/create, /submissions/update |
| SubmissionCommentsApiController | app\controllers\submission_comments_api_controller.rb | 8 | /submissioncommentsapi/update, /submissioncommentsapi/destroy |
| SubmissionCommentsController | app\controllers\submission_comments_controller.rb | 6 | /submissioncomments/index, /submissioncomments/update, /submissioncomments/destroy |
| SubAccountsController | app\controllers\sub_accounts_controller.rb | 8 | /subaccounts/index, /subaccounts/show, /subaccounts/create, /subaccounts/update, /subaccounts/destroy |
| CrocodocController | app\controllers\support_helpers\crocodoc_controller.rb | 2 | none |
| PlagiarismPlatformController | app\controllers\support_helpers\plagiarism_platform_controller.rb | 6 | none |
| SubmissionLifecycleManageController | app\controllers\support_helpers\submission_lifecycle_manage_controller.rb | 1 | none |
| TurnitinController | app\controllers\support_helpers\turnitin_controller.rb | 7 | none |
| TabsController | app\controllers\tabs_controller.rb | 2 | /tabs/index, /tabs/update |
| TemporaryEnrollmentPairingsApiController | app\controllers\temporary_enrollment_pairings_api_controller.rb | 9 | /temporaryenrollmentpairingsapi/index, /temporaryenrollmentpairingsapi/show, /temporaryenrollmentpairingsapi/new, /temporaryenrollmentpairingsapi/create, /temporaryenrollmentpairingsapi/destroy |
| TermsApiController | app\controllers\terms_api_controller.rb | 4 | /termsapi/index, /termsapi/show |
| TermsController | app\controllers\terms_controller.rb | 8 | /terms/create, /terms/update, /terms/destroy |
| TokensController | app\controllers\tokens_controller.rb | 7 | /tokens/show, /tokens/create, /tokens/update, /tokens/destroy |
| TranslationController | app\controllers\translation_controller.rb | 8 | none |
| UsageRightsController | app\controllers\usage_rights_controller.rb | 5 | none |
| UsersController | app\controllers\users_controller.rb | 83 | /users/show, /users/new, /users/create, /users/update, /users/destroy |
| UserListsController | app\controllers\user_lists_controller.rb | 2 | /userlists/create |
| UserObserveesController | app\controllers\user_observees_controller.rb | 21 | /userobservees/index, /userobservees/create, /userobservees/show, /userobservees/update, /userobservees/destroy |
| WebZipExportsController | app\controllers\web_zip_exports_controller.rb | 3 | /webzipexports/index, /webzipexports/show |
| WhatIfGradesApiController | app\controllers\what_if_grades_api_controller.rb | 3 | /whatifgradesapi/update |
| WikiPagesApiController | app\controllers\wiki_pages_api_controller.rb | 21 | /wikipagesapi/index, /wikipagesapi/create, /wikipagesapi/show, /wikipagesapi/update, /wikipagesapi/destroy |
| WikiPagesController | app\controllers\wiki_pages_controller.rb | 11 | /wikipages/index, /wikipages/show, /wikipages/edit |
