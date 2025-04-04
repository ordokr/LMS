# Canvas Core Models Reference

## Course Model
```ruby
# From Canvas LMS: app/models/course.rb
class Course < ActiveRecord::Base
  include Workflow
  include Content
  # ... key attributes and relations
end