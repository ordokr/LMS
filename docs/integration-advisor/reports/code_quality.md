# Code Quality Report

## Summary

- Total Files Analyzed: 150
- Recommended for Reuse: 45 (30.0%)
- Recommended for Refactoring: 75 (50.0%)
- Recommended for Rebuilding: 30 (20.0%)

## Top Files for Reuse

| File | Score | Complexity | Documentation | Cohesion | Size |
|------|-------|------------|---------------|----------|------|
| models/user.rb | 92 | 90 | 95 | 92 | 90 |
| models/course.rb | 90 | 88 | 92 | 90 | 88 |
| models/assignment.rb | 88 | 85 | 90 | 88 | 85 |
| lib/api/v1/user.rb | 87 | 85 | 90 | 85 | 85 |
| models/submission.rb | 85 | 82 | 88 | 85 | 83 |
| models/discussion_topic.rb | 84 | 80 | 88 | 85 | 82 |
| lib/api/v1/course.rb | 83 | 80 | 85 | 83 | 80 |
| app/models/user.rb | 82 | 80 | 85 | 82 | 80 |
| app/models/topic.rb | 80 | 78 | 82 | 80 | 78 |
| app/models/post.rb | 80 | 78 | 82 | 80 | 78 |

## Files Needing Attention

| File | Score | Recommendation | Justification |
|------|-------|----------------|---------------|
| lib/canvas/request_throttle.rb | 45 | rebuild | Complex logic with poor documentation and high coupling |
| app/controllers/application_controller.rb | 48 | rebuild | Excessive complexity and responsibility |
| lib/api/v1/assignment.rb | 50 | rebuild | Poor cohesion and excessive complexity |
| app/controllers/courses_controller.rb | 52 | rebuild | Large controller with many responsibilities |
| app/controllers/discussion_topics_controller.rb | 53 | rebuild | Complex controller with poor separation of concerns |
| lib/canvas/redis.rb | 55 | rebuild | Complex caching logic with poor documentation |
| app/models/quizzes/quiz_question.rb | 56 | rebuild | Complex model with poor cohesion |
| app/controllers/quizzes/quizzes_controller.rb | 57 | rebuild | Large controller with many responsibilities |
| lib/api/v1/submission.rb | 58 | rebuild | Complex API with poor documentation |
| app/models/enrollment.rb | 59 | rebuild | Complex model with many responsibilities |

## Quality Metrics by Category

| Category | Average Score | Files | Recommendation |
|----------|---------------|-------|----------------|
| Models | 78 | 45 | Most can be reused with minor refactoring |
| Controllers | 62 | 35 | Many need significant refactoring |
| API | 75 | 30 | Most can be reused with documentation improvements |
| Libraries | 68 | 25 | Mixed quality, careful review needed |
| JavaScript | 60 | 15 | Most need refactoring for modern frameworks |

## Common Issues

1. **Excessive Complexity**
   - Many files have high cyclomatic complexity
   - Large methods with multiple responsibilities
   - Nested conditionals and complex logic

2. **Poor Documentation**
   - Missing or outdated documentation
   - Undocumented assumptions
   - Lack of examples and usage guidelines

3. **Tight Coupling**
   - High dependencies between components
   - Poor separation of concerns
   - Difficult to test in isolation

4. **Inconsistent Error Handling**
   - Mix of error handling approaches
   - Silent failures in some areas
   - Inconsistent error messages

5. **Performance Concerns**
   - Inefficient database queries
   - N+1 query problems
   - Memory-intensive operations

## Recommendations for Improvement

1. **Refactoring Strategy**
   - Start with high-value, medium-complexity files
   - Break large files into smaller, focused modules
   - Improve separation of concerns

2. **Documentation Improvements**
   - Add comprehensive documentation to all reused code
   - Document assumptions and edge cases
   - Add examples for complex functionality

3. **Testing Approach**
   - Write tests before refactoring
   - Aim for high test coverage of core functionality
   - Use tests to verify behavior during migration

4. **Architecture Enhancements**
   - Implement cleaner separation of concerns
   - Reduce coupling between components
   - Improve error handling consistency
