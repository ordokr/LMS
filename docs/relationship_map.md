# Code Relationship Map
_Generated on 2025-04-04_

## Model Relationships

```mermaid
classDiagram
    class Forum
    class Category
    class Course
    class Module
    class Assignment
    class Submission
    class Post
    class Tag
    class Topic
    class User
    User --> Course
    Course --> Module
    Module --> Assignment
    User --> Post
    Topic --> Post
    Category --> Topic
    Course --> Category
```

## API-Model Dependencies

```mermaid
flowchart LR
    Forum["Forum"]
    get_get_forum_stats("get(get_forum_stats")
    get_get_forum_stats --> Forum
    get_search_forum("get(search_forum")
    get_search_forum --> Forum
    get_forum_categories__get_categories("get(forum_categories::get_categories")
    get_forum_categories__get_categories --> Forum
    Category["Category"]
    get_get_categories("get(get_categories")
    get_get_categories --> Category
    post_create_category("post(create_category")
    post_create_category --> Category
    get_get_category("get(get_category")
    get_get_category --> Category
    Course["Course"]
    get_get_categories_by_course("get(get_categories_by_course")
    get_get_categories_by_course --> Course
    get_integration__get_course_category("get(integration::get_course_category")
    get_integration__get_course_category --> Course
    get_integration__get_course_forum_activity("get(integration::get_course_forum_activity")
    get_integration__get_course_forum_activity --> Course
    Module["Module"]
    get_integration__get_module_topic("get(integration::get_module_topic")
    get_integration__get_module_topic --> Module
    Assignment["Assignment"]
    get_integration__get_assignment_topic("get(integration::get_assignment_topic")
    get_integration__get_assignment_topic --> Assignment
    Post["Post"]
    post_create_category("post(create_category")
    post_create_category --> Post
    post_create_topic("post(create_topic")
    post_create_topic --> Post
    get_get_posts_by_topic("get(get_posts_by_topic")
    get_get_posts_by_topic --> Post
    Tag["Tag"]
    get_get_tags("get(get_tags")
    get_get_tags --> Tag
    get_get_topics_by_tag("get(get_topics_by_tag")
    get_get_topics_by_tag --> Tag
    Topic["Topic"]
    get_get_topics_by_category("get(get_topics_by_category")
    get_get_topics_by_category --> Topic
    get_get_topics("get(get_topics")
    get_get_topics --> Topic
    post_create_topic("post(create_topic")
    post_create_topic --> Topic
```

## Module Structure

```mermaid
flowchart TD
    FE[Frontend]
    API[API Layer]
    Models[Data Models]
    Sync[Sync Engine]
    DB[(Database)]
    FE --> API
    API --> Models
    Models --> DB
    API --> Sync
    Sync --> DB
```

