# Feature Mapping Report

## Summary

- Canvas Features: 128
- Discourse Features: 95
- Ordo Features: 86
- Total Mappings: 223

## Implementation Status

- Implemented: 72 (32.3%)
- Partial: 48 (21.5%)
- Missing: 103 (46.2%)

## Feature Mappings by Category

### course_mgmt

| Source Feature | Target Feature | Status | Confidence | Priority |
|---------------|----------------|--------|------------|----------|
| canvas.course_index_route | ordo.course_list_function | implemented | 0.85 | 5 |
| canvas.course_show_route | ordo.course_detail_function | implemented | 0.90 | 5 |
| canvas.course_create_route | ordo.course_create_function | implemented | 0.95 | 5 |
| canvas.course_update_route | ordo.course_update_function | implemented | 0.92 | 5 |
| canvas.course_delete_route | ordo.course_delete_function | implemented | 0.88 | 5 |
| canvas.course_modules_route | ordo.course_modules_function | partial | 0.65 | 5 |
| canvas.course_settings_route | ordo.course_settings_function | partial | 0.70 | 4 |
| canvas.course_users_route | Not implemented | missing | 0.00 | 4 |
| canvas.course_files_route | Not implemented | missing | 0.00 | 3 |

### assignment_mgmt

| Source Feature | Target Feature | Status | Confidence | Priority |
|---------------|----------------|--------|------------|----------|
| canvas.assignment_index_route | ordo.assignment_list_function | implemented | 0.88 | 5 |
| canvas.assignment_show_route | ordo.assignment_detail_function | implemented | 0.92 | 5 |
| canvas.assignment_create_route | ordo.assignment_create_function | implemented | 0.95 | 5 |
| canvas.assignment_update_route | ordo.assignment_update_function | implemented | 0.90 | 5 |
| canvas.assignment_delete_route | ordo.assignment_delete_function | implemented | 0.85 | 5 |
| canvas.assignment_submit_route | ordo.assignment_submit_function | partial | 0.75 | 5 |
| canvas.assignment_grade_route | ordo.assignment_grade_function | partial | 0.68 | 5 |
| canvas.assignment_rubric_route | Not implemented | missing | 0.00 | 4 |
| canvas.assignment_peer_review_route | Not implemented | missing | 0.00 | 3 |

### discussions

| Source Feature | Target Feature | Status | Confidence | Priority |
|---------------|----------------|--------|------------|----------|
| discourse.topic_index_route | ordo.discussion_list_function | implemented | 0.82 | 4 |
| discourse.topic_show_route | ordo.discussion_detail_function | implemented | 0.88 | 4 |
| discourse.topic_create_route | ordo.discussion_create_function | implemented | 0.90 | 4 |
| discourse.topic_update_route | ordo.discussion_update_function | implemented | 0.85 | 4 |
| discourse.topic_delete_route | ordo.discussion_delete_function | implemented | 0.80 | 4 |
| discourse.post_create_route | ordo.post_create_function | implemented | 0.92 | 4 |
| discourse.post_update_route | ordo.post_update_function | implemented | 0.88 | 4 |
| discourse.post_delete_route | ordo.post_delete_function | implemented | 0.85 | 4 |
| discourse.post_like_route | ordo.post_like_function | partial | 0.72 | 3 |
| discourse.post_flag_route | Not implemented | missing | 0.00 | 2 |

## Missing Features by Priority

### Priority 5

| Feature | Category |
|---------|----------|
| canvas.course_users_route | course_mgmt |
| canvas.assignment_rubric_route | assignment_mgmt |
| canvas.quiz_create_route | assignment_mgmt |
| canvas.quiz_take_route | assignment_mgmt |
| canvas.grade_override_route | grading |

### Priority 4

| Feature | Category |
|---------|----------|
| canvas.course_files_route | course_mgmt |
| canvas.assignment_peer_review_route | assignment_mgmt |
| canvas.gradebook_route | grading |
| discourse.topic_pin_route | discussions |
| discourse.user_preferences_route | auth |

### Priority 3

| Feature | Category |
|---------|----------|
| canvas.calendar_route | other |
| canvas.announcement_route | other |
| discourse.post_bookmark_route | discussions |
| discourse.user_activity_route | auth |
| discourse.category_create_route | tagging |
