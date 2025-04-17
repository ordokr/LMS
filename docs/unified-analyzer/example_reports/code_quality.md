# Code Quality Analysis Report

## Summary

- Total Files Analyzed: 245
- High Quality Files: 142 (58.0%)
- Medium Quality Files: 78 (31.8%)
- Low Quality Files: 25 (10.2%)
- Average Usefulness Score: 72.5

## Quality Distribution by Source

| Source | High Quality | Medium Quality | Low Quality | Average Score |
|--------|--------------|----------------|-------------|---------------|
| canvas | 65 (54.2%) | 42 (35.0%) | 13 (10.8%) | 68.3 |
| discourse | 77 (61.6%) | 36 (28.8%) | 12 (9.6%) | 76.7 |

## Top 10 Files Needing Improvement

| File | LOC | Complexity | Comment Coverage | Usefulness Score | Recommendation |
|------|-----|------------|------------------|------------------|----------------|
| canvas/app/controllers/quizzes_controller.rb | 1245 | 42 | 5.2% | 32 | rebuild |
| canvas/app/models/assignment.rb | 982 | 38 | 8.7% | 35 | rebuild |
| discourse/app/controllers/topics_controller.rb | 876 | 35 | 10.2% | 38 | rebuild |
| canvas/app/controllers/gradebook_controller.rb | 754 | 32 | 12.5% | 42 | refactor |
| discourse/app/models/post.rb | 685 | 30 | 15.3% | 45 | refactor |
| canvas/app/controllers/courses_controller.rb | 625 | 28 | 18.2% | 48 | refactor |
| discourse/app/controllers/users_controller.rb | 598 | 25 | 20.1% | 52 | refactor |
| canvas/app/models/user.rb | 542 | 22 | 22.5% | 55 | refactor |
| discourse/app/models/topic.rb | 512 | 20 | 25.0% | 58 | refactor |
| canvas/app/controllers/submissions_controller.rb | 485 | 18 | 28.3% | 62 | improve |

## Complexity Analysis

| Complexity Range | File Count | Average LOC | Average Comment Coverage |
|------------------|------------|-------------|--------------------------|
| Very High (30+) | 8 | 865 | 9.1% |
| High (20-29) | 22 | 568 | 21.5% |
| Medium (10-19) | 87 | 325 | 32.8% |
| Low (5-9) | 95 | 185 | 45.2% |
| Very Low (1-4) | 33 | 85 | 58.7% |

## Recommendations

1. **Rebuild** files with usefulness scores below 40
2. **Refactor** files with usefulness scores between 40 and 60
3. **Improve** files with usefulness scores between 60 and 70
4. Focus on increasing comment coverage in complex files
5. Break down large controller files into smaller, more focused components
6. Extract common functionality into shared modules
7. Add comprehensive tests for files being refactored
