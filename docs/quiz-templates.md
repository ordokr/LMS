# Quiz Module Templates

This document describes the quiz templates feature implemented for the Quiz Module, allowing users to create reusable templates for quizzes.

## 1. Overview

The Quiz Module now includes a comprehensive template system that:

- Allows users to create reusable quiz templates
- Supports creating templates from existing quizzes
- Enables users to browse and search public templates
- Provides a rating system for templates
- Makes it easy to create new quizzes from templates

This system enables educators to save time by reusing quiz structures and sharing effective quiz designs with others.

## 2. Template Categories

Templates are organized into categories for easier browsing:

```rust
pub enum TemplateCategory {
    Education,
    Business,
    Science,
    Technology,
    Language,
    Arts,
    Health,
    Custom,
}
```

## 3. Data Models

### Quiz Template

The `QuizTemplate` represents a reusable quiz structure:

```rust
pub struct QuizTemplate {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub question_templates: Vec<QuestionTemplate>,
    pub default_study_mode: StudyMode,
    pub default_visibility: QuizVisibility,
    pub is_public: bool,
    pub usage_count: i32,
    pub rating: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Question Template

The `QuestionTemplate` represents a template for a quiz question:

```rust
pub struct QuestionTemplate {
    pub id: Uuid,
    pub template_id: Uuid,
    pub text: String,
    pub description: Option<String>,
    pub answer_type: AnswerType,
    pub placeholder_text: Option<String>,
    pub example_answers: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Template Rating

The `TemplateRating` represents a user's rating of a template:

```rust
pub struct TemplateRating {
    pub id: Uuid,
    pub template_id: Uuid,
    pub user_id: Uuid,
    pub rating: f32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 4. Database Schema

The template system uses several tables:

### Quiz Templates Table

```sql
CREATE TABLE IF NOT EXISTS quiz_templates (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author_id TEXT,
    category TEXT NOT NULL,
    tags TEXT,
    default_study_mode TEXT NOT NULL,
    default_visibility TEXT NOT NULL,
    is_public INTEGER NOT NULL DEFAULT 0,
    usage_count INTEGER NOT NULL DEFAULT 0,
    rating REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

### Quiz Question Templates Table

```sql
CREATE TABLE IF NOT EXISTS quiz_question_templates (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    text TEXT NOT NULL,
    description TEXT,
    answer_type TEXT NOT NULL,
    placeholder_text TEXT,
    example_answers TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES quiz_templates (id) ON DELETE CASCADE
);
```

### Quiz Template Ratings Table

```sql
CREATE TABLE IF NOT EXISTS quiz_template_ratings (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    rating REAL NOT NULL,
    comment TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES quiz_templates (id) ON DELETE CASCADE
);
```

## 5. Core Functionality

### Creating Templates

Templates can be created from scratch or from existing quizzes:

```rust
// Create a new template
let template = template_service.create_template(
    title,
    description,
    author_id,
    category,
    tags,
    default_study_mode,
    default_visibility,
    is_public,
).await?;

// Create a template from an existing quiz
let template = template_service.create_template_from_quiz(
    quiz_id,
    title,
    description,
    author_id,
    category,
    tags,
    is_public,
).await?;
```

### Managing Templates

Templates can be updated, deleted, and retrieved:

```rust
// Update a template
let template = template_service.update_template(
    template_id,
    title,
    description,
    category,
    tags,
    default_study_mode,
    default_visibility,
    is_public,
).await?;

// Delete a template
template_service.delete_template(template_id).await?;

// Get a template
let template = template_service.get_template(template_id).await?;
```

### Browsing Templates

Users can browse and search for templates:

```rust
// Get public templates
let templates = template_service.get_public_templates(limit, offset).await?;

// Search templates
let templates = template_service.search_templates(query, category, limit, offset).await?;

// Get templates by author
let templates = template_service.get_templates_by_author(author_id, limit, offset).await?;
```

### Creating Quizzes from Templates

Templates can be used to create new quizzes:

```rust
// Create a quiz from a template
let quiz = template_service.create_quiz_from_template(
    template_id,
    title,
    description,
    author_id,
).await?;
```

### Rating Templates

Users can rate templates and view ratings:

```rust
// Rate a template
let rating = template_service.rate_template(
    template_id,
    user_id,
    rating,
    comment,
).await?;

// Get a user's rating for a template
let rating = template_service.get_user_rating(template_id, user_id).await?;

// Get all ratings for a template
let ratings = template_service.get_template_ratings(template_id, limit, offset).await?;

// Delete a rating
template_service.delete_rating(rating_id, user_id).await?;
```

## 6. Tauri Commands

The following Tauri commands are available for the template system:

### Template Management Commands

```typescript
// Create a new template
const template = await invoke('create_template', {
  title: 'My Template',
  description: 'A template for quizzes',
  authorId: '550e8400-e29b-41d4-a716-446655440000',
  category: 'Education',
  tags: ['math', 'algebra'],
  defaultStudyMode: 'multiple_choice',
  defaultVisibility: 'private',
  isPublic: true,
});

// Create a template from an existing quiz
const template = await invoke('create_template_from_quiz', {
  quizId: '550e8400-e29b-41d4-a716-446655440001',
  title: 'My Template',
  description: 'A template for quizzes',
  authorId: '550e8400-e29b-41d4-a716-446655440000',
  category: 'Education',
  tags: ['math', 'algebra'],
  isPublic: true,
});

// Add a question template
const questionTemplate = await invoke('add_question_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  text: 'What is 2 + 2?',
  description: 'A simple addition question',
  answerType: 'multiple_choice',
  placeholderText: 'Enter your answer',
  exampleAnswers: ['4', '3', '5', '6'],
});

// Update a template
const template = await invoke('update_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  title: 'Updated Template',
  description: 'An updated template for quizzes',
  category: 'Science',
  tags: ['physics', 'mechanics'],
  defaultStudyMode: 'flashcards',
  defaultVisibility: 'public',
  isPublic: true,
});

// Delete a template
await invoke('delete_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
});

// Get a template
const template = await invoke('get_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
});
```

### Template Browsing Commands

```typescript
// Get public templates
const templates = await invoke('get_public_templates', {
  limit: 10,
  offset: 0,
});

// Search templates
const templates = await invoke('search_templates', {
  query: 'math',
  category: 'Education',
  limit: 10,
  offset: 0,
});

// Get templates by author
const templates = await invoke('get_templates_by_author', {
  authorId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 10,
  offset: 0,
});

// Create a quiz from a template
const quiz = await invoke('create_quiz_from_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  title: 'My Quiz',
  description: 'A quiz created from a template',
  authorId: '550e8400-e29b-41d4-a716-446655440000',
});
```

### Template Rating Commands

```typescript
// Rate a template
const rating = await invoke('rate_template', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  userId: '550e8400-e29b-41d4-a716-446655440000',
  rating: 4.5,
  comment: 'Great template!',
});

// Get a user's rating for a template
const rating = await invoke('get_user_template_rating', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  userId: '550e8400-e29b-41d4-a716-446655440000',
});

// Get all ratings for a template
const ratings = await invoke('get_template_ratings', {
  templateId: '550e8400-e29b-41d4-a716-446655440002',
  limit: 10,
  offset: 0,
});

// Delete a rating
await invoke('delete_template_rating', {
  ratingId: '550e8400-e29b-41d4-a716-446655440003',
  userId: '550e8400-e29b-41d4-a716-446655440000',
});
```

## 7. Frontend Integration

The template system can be integrated into the frontend to provide a template browsing and creation experience:

### Template Browser

```tsx
// In a template browser component
const [templates, setTemplates] = useState([]);
const [searchQuery, setSearchQuery] = useState('');
const [selectedCategory, setSelectedCategory] = useState(null);

useEffect(() => {
  const fetchTemplates = async () => {
    if (searchQuery) {
      const templates = await invoke('search_templates', {
        query: searchQuery,
        category: selectedCategory,
        limit: 20,
        offset: 0,
      });
      setTemplates(templates);
    } else {
      const templates = await invoke('get_public_templates', {
        limit: 20,
        offset: 0,
      });
      setTemplates(templates);
    }
  };
  
  fetchTemplates();
}, [searchQuery, selectedCategory]);

const handleSearch = (e) => {
  e.preventDefault();
  setSearchQuery(e.target.elements.query.value);
};

const handleCategoryChange = (category) => {
  setSelectedCategory(category);
};

return (
  <div className="template-browser">
    <h2>Browse Templates</h2>
    
    <div className="search-bar">
      <form onSubmit={handleSearch}>
        <input
          type="text"
          name="query"
          placeholder="Search templates..."
        />
        
        <button type="submit">Search</button>
      </form>
    </div>
    
    <div className="category-filter">
      <button
        className={selectedCategory === null ? 'active' : ''}
        onClick={() => handleCategoryChange(null)}
      >
        All
      </button>
      
      <button
        className={selectedCategory === 'Education' ? 'active' : ''}
        onClick={() => handleCategoryChange('Education')}
      >
        Education
      </button>
      
      <button
        className={selectedCategory === 'Business' ? 'active' : ''}
        onClick={() => handleCategoryChange('Business')}
      >
        Business
      </button>
      
      {/* Add more category buttons */}
    </div>
    
    <div className="template-grid">
      {templates.map(template => (
        <div key={template.id} className="template-card">
          <h3>{template.title}</h3>
          
          <div className="template-meta">
            <span className="category">{template.category}</span>
            <span className="rating">★ {template.rating || 'No ratings'}</span>
            <span className="usage">{template.usage_count} uses</span>
          </div>
          
          <p className="description">{template.description}</p>
          
          <div className="tags">
            {template.tags.map(tag => (
              <span key={tag} className="tag">{tag}</span>
            ))}
          </div>
          
          <div className="actions">
            <button onClick={() => navigate(`/templates/${template.id}`)}>
              View
            </button>
            
            <button onClick={() => handleUseTemplate(template.id)}>
              Use Template
            </button>
          </div>
        </div>
      ))}
    </div>
  </div>
);
```

### Template Creator

```tsx
// In a template creator component
const [title, setTitle] = useState('');
const [description, setDescription] = useState('');
const [category, setCategory] = useState('Education');
const [tags, setTags] = useState([]);
const [studyMode, setStudyMode] = useState('multiple_choice');
const [visibility, setVisibility] = useState('private');
const [isPublic, setIsPublic] = useState(false);
const [questionTemplates, setQuestionTemplates] = useState([]);

const handleCreateTemplate = async () => {
  try {
    const template = await invoke('create_template', {
      title,
      description: description || undefined,
      authorId: currentUser.id,
      category,
      tags,
      defaultStudyMode: studyMode,
      defaultVisibility: visibility,
      isPublic,
    });
    
    navigate(`/templates/${template.id}`);
  } catch (error) {
    console.error('Error creating template:', error);
  }
};

const handleCreateFromQuiz = async (quizId) => {
  try {
    const template = await invoke('create_template_from_quiz', {
      quizId,
      title,
      description: description || undefined,
      authorId: currentUser.id,
      category,
      tags,
      isPublic,
    });
    
    navigate(`/templates/${template.id}`);
  } catch (error) {
    console.error('Error creating template from quiz:', error);
  }
};

const handleAddQuestionTemplate = async () => {
  // Open a modal to add a question template
  setShowAddQuestionModal(true);
};

const handleSaveQuestionTemplate = async (questionTemplate) => {
  try {
    const savedTemplate = await invoke('add_question_template', {
      templateId: template.id,
      text: questionTemplate.text,
      description: questionTemplate.description || undefined,
      answerType: questionTemplate.answerType,
      placeholderText: questionTemplate.placeholderText || undefined,
      exampleAnswers: questionTemplate.exampleAnswers,
    });
    
    setQuestionTemplates([...questionTemplates, savedTemplate]);
    setShowAddQuestionModal(false);
  } catch (error) {
    console.error('Error adding question template:', error);
  }
};

return (
  <div className="template-creator">
    <h2>Create Template</h2>
    
    <div className="form-group">
      <label htmlFor="title">Title</label>
      <input
        type="text"
        id="title"
        value={title}
        onChange={e => setTitle(e.target.value)}
        required
      />
    </div>
    
    <div className="form-group">
      <label htmlFor="description">Description</label>
      <textarea
        id="description"
        value={description}
        onChange={e => setDescription(e.target.value)}
      />
    </div>
    
    <div className="form-group">
      <label htmlFor="category">Category</label>
      <select
        id="category"
        value={category}
        onChange={e => setCategory(e.target.value)}
      >
        <option value="Education">Education</option>
        <option value="Business">Business</option>
        <option value="Science">Science</option>
        <option value="Technology">Technology</option>
        <option value="Language">Language</option>
        <option value="Arts">Arts</option>
        <option value="Health">Health</option>
        <option value="Custom">Custom</option>
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="tags">Tags</label>
      <TagInput
        tags={tags}
        onChange={setTags}
      />
    </div>
    
    <div className="form-group">
      <label htmlFor="studyMode">Default Study Mode</label>
      <select
        id="studyMode"
        value={studyMode}
        onChange={e => setStudyMode(e.target.value)}
      >
        <option value="multiple_choice">Multiple Choice</option>
        <option value="flashcards">Flashcards</option>
        <option value="written">Written</option>
        <option value="mixed">Mixed</option>
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="visibility">Default Visibility</label>
      <select
        id="visibility"
        value={visibility}
        onChange={e => setVisibility(e.target.value)}
      >
        <option value="private">Private</option>
        <option value="public">Public</option>
        <option value="unlisted">Unlisted</option>
      </select>
    </div>
    
    <div className="form-group">
      <label>
        <input
          type="checkbox"
          checked={isPublic}
          onChange={e => setIsPublic(e.target.checked)}
        />
        Make template public
      </label>
    </div>
    
    <div className="question-templates">
      <h3>Question Templates</h3>
      
      <button onClick={handleAddQuestionTemplate}>
        Add Question Template
      </button>
      
      <div className="question-template-list">
        {questionTemplates.map(questionTemplate => (
          <div key={questionTemplate.id} className="question-template-item">
            <h4>{questionTemplate.text}</h4>
            <p>{questionTemplate.description}</p>
            <p>Type: {questionTemplate.answer_type}</p>
            <p>Example Answers: {questionTemplate.example_answers.join(', ')}</p>
          </div>
        ))}
      </div>
    </div>
    
    <div className="actions">
      <button onClick={handleCreateTemplate}>
        Create Template
      </button>
      
      <button onClick={() => setShowQuizSelector(true)}>
        Create from Quiz
      </button>
    </div>
    
    {showAddQuestionModal && (
      <QuestionTemplateModal
        onSave={handleSaveQuestionTemplate}
        onCancel={() => setShowAddQuestionModal(false)}
      />
    )}
    
    {showQuizSelector && (
      <QuizSelectorModal
        onSelect={handleCreateFromQuiz}
        onCancel={() => setShowQuizSelector(false)}
      />
    )}
  </div>
);
```

### Template Details

```tsx
// In a template details component
const [template, setTemplate] = useState(null);
const [userRating, setUserRating] = useState(null);
const [ratings, setRatings] = useState([]);
const [ratingValue, setRatingValue] = useState(0);
const [ratingComment, setRatingComment] = useState('');

useEffect(() => {
  const fetchTemplate = async () => {
    const template = await invoke('get_template', {
      templateId: templateId,
    });
    setTemplate(template);
  };
  
  const fetchUserRating = async () => {
    try {
      const rating = await invoke('get_user_template_rating', {
        templateId: templateId,
        userId: currentUser.id,
      });
      setUserRating(rating);
      setRatingValue(rating.rating);
      setRatingComment(rating.comment || '');
    } catch (error) {
      // User hasn't rated this template yet
      setUserRating(null);
    }
  };
  
  const fetchRatings = async () => {
    const ratings = await invoke('get_template_ratings', {
      templateId: templateId,
      limit: 10,
      offset: 0,
    });
    setRatings(ratings);
  };
  
  fetchTemplate();
  fetchUserRating();
  fetchRatings();
}, [templateId]);

const handleRateTemplate = async () => {
  try {
    const rating = await invoke('rate_template', {
      templateId: templateId,
      userId: currentUser.id,
      rating: ratingValue,
      comment: ratingComment || undefined,
    });
    
    setUserRating(rating);
    
    // Refresh ratings
    const ratings = await invoke('get_template_ratings', {
      templateId: templateId,
      limit: 10,
      offset: 0,
    });
    setRatings(ratings);
    
    // Refresh template to get updated rating
    const template = await invoke('get_template', {
      templateId: templateId,
    });
    setTemplate(template);
  } catch (error) {
    console.error('Error rating template:', error);
  }
};

const handleDeleteRating = async () => {
  try {
    await invoke('delete_template_rating', {
      ratingId: userRating.id,
      userId: currentUser.id,
    });
    
    setUserRating(null);
    setRatingValue(0);
    setRatingComment('');
    
    // Refresh ratings
    const ratings = await invoke('get_template_ratings', {
      templateId: templateId,
      limit: 10,
      offset: 0,
    });
    setRatings(ratings);
    
    // Refresh template to get updated rating
    const template = await invoke('get_template', {
      templateId: templateId,
    });
    setTemplate(template);
  } catch (error) {
    console.error('Error deleting rating:', error);
  }
};

const handleUseTemplate = async () => {
  try {
    const quiz = await invoke('create_quiz_from_template', {
      templateId: templateId,
      title: `${template.title} (from template)`,
      description: template.description,
      authorId: currentUser.id,
    });
    
    navigate(`/quizzes/${quiz.id}/edit`);
  } catch (error) {
    console.error('Error creating quiz from template:', error);
  }
};

if (!template) {
  return <div>Loading...</div>;
}

return (
  <div className="template-details">
    <h2>{template.title}</h2>
    
    <div className="template-meta">
      <span className="category">{template.category}</span>
      <span className="rating">★ {template.rating || 'No ratings'}</span>
      <span className="usage">{template.usage_count} uses</span>
    </div>
    
    <p className="description">{template.description}</p>
    
    <div className="tags">
      {template.tags.map(tag => (
        <span key={tag} className="tag">{tag}</span>
      ))}
    </div>
    
    <div className="question-templates">
      <h3>Question Templates ({template.question_templates.length})</h3>
      
      <div className="question-template-list">
        {template.question_templates.map(questionTemplate => (
          <div key={questionTemplate.id} className="question-template-item">
            <h4>{questionTemplate.text}</h4>
            <p>{questionTemplate.description}</p>
            <p>Type: {questionTemplate.answer_type}</p>
            <p>Example Answers: {questionTemplate.example_answers.join(', ')}</p>
          </div>
        ))}
      </div>
    </div>
    
    <div className="actions">
      <button onClick={handleUseTemplate}>
        Use Template
      </button>
      
      {template.author_id === currentUser.id && (
        <button onClick={() => navigate(`/templates/${template.id}/edit`)}>
          Edit Template
        </button>
      )}
    </div>
    
    <div className="ratings">
      <h3>Ratings</h3>
      
      <div className="rating-form">
        <h4>{userRating ? 'Your Rating' : 'Rate this Template'}</h4>
        
        <div className="rating-stars">
          {[1, 2, 3, 4, 5].map(star => (
            <button
              key={star}
              className={star <= ratingValue ? 'active' : ''}
              onClick={() => setRatingValue(star)}
            >
              ★
            </button>
          ))}
        </div>
        
        <textarea
          placeholder="Add a comment (optional)"
          value={ratingComment}
          onChange={e => setRatingComment(e.target.value)}
        />
        
        <div className="rating-actions">
          <button onClick={handleRateTemplate}>
            {userRating ? 'Update Rating' : 'Submit Rating'}
          </button>
          
          {userRating && (
            <button onClick={handleDeleteRating}>
              Delete Rating
            </button>
          )}
        </div>
      </div>
      
      <div className="rating-list">
        {ratings.map(rating => (
          <div key={rating.id} className="rating-item">
            <div className="rating-header">
              <span className="rating-stars">
                {'★'.repeat(Math.floor(rating.rating))}
                {rating.rating % 1 >= 0.5 ? '½' : ''}
                {'☆'.repeat(5 - Math.ceil(rating.rating))}
              </span>
              
              <span className="rating-date">
                {new Date(rating.created_at).toLocaleDateString()}
              </span>
            </div>
            
            {rating.comment && (
              <p className="rating-comment">{rating.comment}</p>
            )}
          </div>
        ))}
      </div>
    </div>
  </div>
);
```

## 8. Security Considerations

- **Access Control**: Only authorized users can create and edit templates.
- **Public vs. Private**: Templates can be marked as public or private.
- **Rating Moderation**: Users can only delete their own ratings.
- **Usage Tracking**: Template usage is tracked for analytics.

## 9. Performance Considerations

- **Indexing**: The database schema includes indexes on foreign keys to improve query performance.
- **Pagination**: When retrieving large sets of data, pagination is implemented to improve performance.
- **Lazy Loading**: Question templates are only loaded when viewing a template's details.

## 10. Future Enhancements

- **Featured Templates**: Highlight high-quality templates on the template browser.
- **Template Categories**: Add more categories and subcategories.
- **Template Versioning**: Allow users to create new versions of templates.
- **Template Forking**: Allow users to create their own version of a public template.
- **Template Sharing**: Add the ability to share templates with specific users or groups.
- **Template Analytics**: Provide more detailed analytics on template usage.
- **Template Recommendations**: Recommend templates based on user preferences and history.
