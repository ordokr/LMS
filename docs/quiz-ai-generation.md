# Quiz Module AI-assisted Quiz Generation

This document describes the AI-assisted quiz generation feature implemented for the Quiz Module, allowing users to generate quizzes using various AI models.

## 1. Overview

The Quiz Module now includes a comprehensive AI-assisted quiz generation system that:

- Supports multiple AI model providers (OpenAI, Anthropic, Gemini, etc.)
- Allows generating quizzes from various source types (text, URLs, PDFs, etc.)
- Provides customization options for question types, difficulty, and more
- Tracks generation requests and results
- Is model-agnostic and platform-agnostic with a pluggable architecture

This system enables educators to save time by automatically generating quizzes from their content, with the flexibility to use different AI models based on their needs and availability.

## 2. Source and Model Types

The system supports various source types for generating quizzes:

```rust
pub enum AISourceType {
    Text,
    URL,
    PDF,
    Image,
    Video,
    Audio,
    Custom,
}
```

And multiple AI model providers:

```rust
pub enum AIModelType {
    OpenAI,
    Anthropic,
    Gemini,
    LocalLLM,
    Custom,
}
```

## 3. Data Models

### AI Generation Request

The `AIGenerationRequest` represents a request to generate a quiz using AI:

```rust
pub struct AIGenerationRequest {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub source_type: AISourceType,
    pub source_content: String,
    pub model_type: AIModelType,
    pub model_parameters: Option<serde_json::Value>,
    pub num_questions: i32,
    pub question_types: Vec<AnswerType>,
    pub difficulty_level: i32, // 1-5 scale
    pub topic_focus: Option<String>,
    pub language: String,
    pub study_mode: StudyMode,
    pub visibility: QuizVisibility,
    pub status: AIGenerationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
```

### AI Generation Result

The `AIGenerationResult` represents the result of an AI generation request:

```rust
pub struct AIGenerationResult {
    pub id: Uuid,
    pub request_id: Uuid,
    pub quiz_id: Option<Uuid>,
    pub raw_response: serde_json::Value,
    pub error_message: Option<String>,
    pub processing_time_ms: i64,
    pub token_usage: Option<i32>,
    pub created_at: DateTime<Utc>,
}
```

## 4. Database Schema

The AI generation system uses several tables:

### Quiz AI Generation Requests Table

```sql
CREATE TABLE IF NOT EXISTS quiz_ai_generation_requests (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    source_type TEXT NOT NULL,
    source_content TEXT NOT NULL,
    model_type TEXT NOT NULL,
    model_parameters TEXT,
    num_questions INTEGER NOT NULL,
    question_types TEXT NOT NULL,
    difficulty_level INTEGER NOT NULL,
    topic_focus TEXT,
    language TEXT NOT NULL,
    study_mode TEXT NOT NULL,
    visibility TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT
);
```

### Quiz AI Generation Results Table

```sql
CREATE TABLE IF NOT EXISTS quiz_ai_generation_results (
    id TEXT PRIMARY KEY,
    request_id TEXT NOT NULL,
    quiz_id TEXT,
    raw_response TEXT NOT NULL,
    error_message TEXT,
    processing_time_ms INTEGER NOT NULL,
    token_usage INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (request_id) REFERENCES quiz_ai_generation_requests (id) ON DELETE CASCADE,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE SET NULL
);
```

## 5. Core Functionality

### AI Model Provider Interface

The system uses a trait-based approach to support multiple AI model providers:

```rust
pub trait AIModelProvider: Send + Sync {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> AIModelType;
}
```

This allows for easy addition of new AI model providers without changing the core system.

### Creating and Processing Requests

Requests can be created and processed:

```rust
// Create a new request
let request = ai_generation_service.create_request(
    title,
    description,
    user_id,
    source_type,
    source_content,
    model_type,
    model_parameters,
    num_questions,
    question_types,
    difficulty_level,
    topic_focus,
    language,
    study_mode,
    visibility,
).await?;

// Process a request
let result = ai_generation_service.process_request(request_id).await?;
```

### Creating Quizzes from AI Responses

The system can create quizzes from AI responses:

```rust
// Create a quiz from an AI response
let quiz_id = ai_generation_service.create_quiz_from_response(
    &request,
    &response,
).await?;
```

## 6. Tauri Commands

The following Tauri commands are available for the AI generation system:

### AI Generation Commands

```typescript
// Create a new AI generation request
const request = await invoke('create_ai_generation_request', {
  title: 'My AI-generated Quiz',
  description: 'A quiz about history',
  userId: '550e8400-e29b-41d4-a716-446655440000',
  sourceType: 'Text',
  sourceContent: 'World War II was a global war that lasted from 1939 to 1945...',
  modelType: 'OpenAI',
  modelParameters: { temperature: 0.7 },
  numQuestions: 10,
  questionTypes: ['multiple_choice', 'true_false'],
  difficultyLevel: 3,
  topicFocus: 'World War II',
  language: 'en',
  studyMode: 'multiple_choice',
  visibility: 'private',
});

// Process an AI generation request
const result = await invoke('process_ai_generation_request', {
  requestId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get an AI generation request
const request = await invoke('get_ai_generation_request', {
  requestId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get an AI generation result
const result = await invoke('get_ai_generation_result', {
  requestId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get all AI generation requests for a user
const requests = await invoke('get_ai_generation_requests_by_user', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 10,
  offset: 0,
});

// Cancel an AI generation request
await invoke('cancel_ai_generation_request', {
  requestId: '550e8400-e29b-41d4-a716-446655440001',
});

// Get available AI model providers
const providers = await invoke('get_ai_model_providers');
```

## 7. Frontend Integration

The AI generation system can be integrated into the frontend to provide a quiz generation experience:

### AI Quiz Generator

```tsx
// In an AI quiz generator component
const [title, setTitle] = useState('');
const [description, setDescription] = useState('');
const [sourceType, setSourceType] = useState('Text');
const [sourceContent, setSourceContent] = useState('');
const [modelType, setModelType] = useState('OpenAI');
const [numQuestions, setNumQuestions] = useState(10);
const [questionTypes, setQuestionTypes] = useState(['multiple_choice']);
const [difficultyLevel, setDifficultyLevel] = useState(3);
const [topicFocus, setTopicFocus] = useState('');
const [language, setLanguage] = useState('en');
const [studyMode, setStudyMode] = useState('multiple_choice');
const [visibility, setVisibility] = useState('private');
const [providers, setProviders] = useState([]);
const [isGenerating, setIsGenerating] = useState(false);
const [generatedQuizId, setGeneratedQuizId] = useState(null);

useEffect(() => {
  const fetchProviders = async () => {
    const providers = await invoke('get_ai_model_providers');
    setProviders(providers);
    
    if (providers.length > 0) {
      setModelType(providers[0][1]);
    }
  };
  
  fetchProviders();
}, []);

const handleGenerate = async () => {
  try {
    setIsGenerating(true);
    
    // Create the request
    const request = await invoke('create_ai_generation_request', {
      title,
      description: description || undefined,
      userId: currentUser.id,
      sourceType,
      sourceContent,
      modelType,
      numQuestions,
      questionTypes,
      difficultyLevel,
      topicFocus: topicFocus || undefined,
      language,
      studyMode,
      visibility,
    });
    
    // Process the request
    const result = await invoke('process_ai_generation_request', {
      requestId: request.id,
    });
    
    // Check if a quiz was created
    if (result.quiz_id) {
      setGeneratedQuizId(result.quiz_id);
      toast.success('Quiz generated successfully!');
    } else {
      toast.error('Failed to generate quiz');
    }
  } catch (error) {
    console.error('Error generating quiz:', error);
    toast.error(`Error: ${error}`);
  } finally {
    setIsGenerating(false);
  }
};

return (
  <div className="ai-quiz-generator">
    <h2>AI Quiz Generator</h2>
    
    <div className="form-group">
      <label htmlFor="title">Quiz Title</label>
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
      <label htmlFor="sourceType">Source Type</label>
      <select
        id="sourceType"
        value={sourceType}
        onChange={e => setSourceType(e.target.value)}
      >
        <option value="Text">Text</option>
        <option value="URL">URL</option>
        <option value="PDF">PDF</option>
        <option value="Image">Image</option>
        <option value="Video">Video</option>
        <option value="Audio">Audio</option>
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="sourceContent">Source Content</label>
      <textarea
        id="sourceContent"
        value={sourceContent}
        onChange={e => setSourceContent(e.target.value)}
        rows={10}
        required
      />
    </div>
    
    <div className="form-group">
      <label htmlFor="modelType">AI Model</label>
      <select
        id="modelType"
        value={modelType}
        onChange={e => setModelType(e.target.value)}
      >
        {providers.map(([name, type]) => (
          <option key={type} value={type}>{name}</option>
        ))}
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="numQuestions">Number of Questions</label>
      <input
        type="number"
        id="numQuestions"
        value={numQuestions}
        onChange={e => setNumQuestions(parseInt(e.target.value))}
        min={1}
        max={50}
        required
      />
    </div>
    
    <div className="form-group">
      <label>Question Types</label>
      <div className="checkbox-group">
        <label>
          <input
            type="checkbox"
            checked={questionTypes.includes('multiple_choice')}
            onChange={e => {
              if (e.target.checked) {
                setQuestionTypes([...questionTypes, 'multiple_choice']);
              } else {
                setQuestionTypes(questionTypes.filter(qt => qt !== 'multiple_choice'));
              }
            }}
          />
          Multiple Choice
        </label>
        
        <label>
          <input
            type="checkbox"
            checked={questionTypes.includes('true_false')}
            onChange={e => {
              if (e.target.checked) {
                setQuestionTypes([...questionTypes, 'true_false']);
              } else {
                setQuestionTypes(questionTypes.filter(qt => qt !== 'true_false'));
              }
            }}
          />
          True/False
        </label>
        
        <label>
          <input
            type="checkbox"
            checked={questionTypes.includes('short_answer')}
            onChange={e => {
              if (e.target.checked) {
                setQuestionTypes([...questionTypes, 'short_answer']);
              } else {
                setQuestionTypes(questionTypes.filter(qt => qt !== 'short_answer'));
              }
            }}
          />
          Short Answer
        </label>
      </div>
    </div>
    
    <div className="form-group">
      <label htmlFor="difficultyLevel">Difficulty Level</label>
      <select
        id="difficultyLevel"
        value={difficultyLevel}
        onChange={e => setDifficultyLevel(parseInt(e.target.value))}
      >
        <option value="1">1 - Very Easy</option>
        <option value="2">2 - Easy</option>
        <option value="3">3 - Medium</option>
        <option value="4">4 - Hard</option>
        <option value="5">5 - Very Hard</option>
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="topicFocus">Topic Focus (Optional)</label>
      <input
        type="text"
        id="topicFocus"
        value={topicFocus}
        onChange={e => setTopicFocus(e.target.value)}
        placeholder="E.g., World War II, Photosynthesis, etc."
      />
    </div>
    
    <div className="form-group">
      <label htmlFor="language">Language</label>
      <select
        id="language"
        value={language}
        onChange={e => setLanguage(e.target.value)}
      >
        <option value="en">English</option>
        <option value="es">Spanish</option>
        <option value="fr">French</option>
        <option value="de">German</option>
        <option value="it">Italian</option>
        <option value="pt">Portuguese</option>
        <option value="ru">Russian</option>
        <option value="zh">Chinese</option>
        <option value="ja">Japanese</option>
        <option value="ko">Korean</option>
      </select>
    </div>
    
    <div className="form-group">
      <label htmlFor="studyMode">Study Mode</label>
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
      <label htmlFor="visibility">Visibility</label>
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
    
    <div className="actions">
      <button
        onClick={handleGenerate}
        disabled={isGenerating || !title || !sourceContent || questionTypes.length === 0}
      >
        {isGenerating ? 'Generating...' : 'Generate Quiz'}
      </button>
    </div>
    
    {generatedQuizId && (
      <div className="success-message">
        <p>Quiz generated successfully!</p>
        <button onClick={() => navigate(`/quizzes/${generatedQuizId}`)}>
          View Quiz
        </button>
        <button onClick={() => navigate(`/quizzes/${generatedQuizId}/edit`)}>
          Edit Quiz
        </button>
      </div>
    )}
  </div>
);
```

### AI Generation History

```tsx
// In an AI generation history component
const [requests, setRequests] = useState([]);
const [isLoading, setIsLoading] = useState(true);

useEffect(() => {
  const fetchRequests = async () => {
    try {
      setIsLoading(true);
      const requests = await invoke('get_ai_generation_requests_by_user', {
        userId: currentUser.id,
        limit: 20,
        offset: 0,
      });
      setRequests(requests);
    } catch (error) {
      console.error('Error fetching requests:', error);
      toast.error(`Error: ${error}`);
    } finally {
      setIsLoading(false);
    }
  };
  
  fetchRequests();
}, [currentUser.id]);

const handleCancel = async (requestId) => {
  try {
    await invoke('cancel_ai_generation_request', {
      requestId,
    });
    
    // Refresh the requests
    const updatedRequests = await invoke('get_ai_generation_requests_by_user', {
      userId: currentUser.id,
      limit: 20,
      offset: 0,
    });
    setRequests(updatedRequests);
    
    toast.success('Request cancelled');
  } catch (error) {
    console.error('Error cancelling request:', error);
    toast.error(`Error: ${error}`);
  }
};

if (isLoading) {
  return <div>Loading...</div>;
}

return (
  <div className="ai-generation-history">
    <h2>AI Generation History</h2>
    
    {requests.length === 0 ? (
      <p>No generation requests found.</p>
    ) : (
      <div className="request-list">
        {requests.map(request => (
          <div key={request.id} className="request-item">
            <div className="request-header">
              <h3>{request.title}</h3>
              <span className={`status status-${request.status.toLowerCase()}`}>
                {request.status}
              </span>
            </div>
            
            <div className="request-meta">
              <span className="source-type">{request.source_type}</span>
              <span className="model-type">{request.model_type}</span>
              <span className="questions">{request.num_questions} questions</span>
              <span className="difficulty">Difficulty: {request.difficulty_level}/5</span>
              <span className="date">{new Date(request.created_at).toLocaleString()}</span>
            </div>
            
            {request.description && (
              <p className="description">{request.description}</p>
            )}
            
            <div className="actions">
              {request.status === 'Completed' && (
                <button onClick={() => navigate(`/quizzes/${request.quiz_id}`)}>
                  View Quiz
                </button>
              )}
              
              {request.status === 'Pending' && (
                <button onClick={() => handleCancel(request.id)}>
                  Cancel
                </button>
              )}
              
              <button onClick={() => navigate(`/ai-generation/${request.id}`)}>
                View Details
              </button>
            </div>
          </div>
        ))}
      </div>
    )}
  </div>
);
```

## 8. Model-Agnostic Architecture

The system is designed to be model-agnostic with a pluggable architecture:

### AI Model Provider Interface

```rust
pub trait AIModelProvider: Send + Sync {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> AIModelType;
}
```

### Registering Model Providers

```rust
// Create the AI generation service
let mut ai_service = AIGenerationService::new(
    db_pool.clone(),
    store.clone()
);

// Register model providers
ai_service.register_model_provider(Box::new(MockAIModelProvider));

// Register OpenAI provider if API key is available
if let Some(openai_api_key) = std::env::var("OPENAI_API_KEY").ok() {
    ai_service.register_model_provider(Box::new(OpenAIModelProvider::new(
        openai_api_key,
        "gpt-4".to_string(),
    )));
}

// Register Anthropic provider if API key is available
if let Some(anthropic_api_key) = std::env::var("ANTHROPIC_API_KEY").ok() {
    ai_service.register_model_provider(Box::new(AnthropicModelProvider::new(
        anthropic_api_key,
        "claude-3-opus".to_string(),
    )));
}
```

### Sample Model Provider Implementation

```rust
/// OpenAI model provider
pub struct OpenAIModelProvider {
    api_key: String,
    model: String,
}

impl OpenAIModelProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
        }
    }
}

impl AIModelProvider for OpenAIModelProvider {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        // Implementation details...
    }
    
    fn get_name(&self) -> String {
        format!("OpenAI ({})", self.model)
    }
    
    fn get_type(&self) -> AIModelType {
        AIModelType::OpenAI
    }
}
```

## 9. Security Considerations

- **API Keys**: API keys for AI providers are stored as environment variables, not in the database.
- **User Authentication**: Only authenticated users can create and process AI generation requests.
- **Rate Limiting**: The system can be extended to include rate limiting to prevent abuse.
- **Content Filtering**: AI providers often include content filtering to prevent inappropriate content.

## 10. Performance Considerations

- **Asynchronous Processing**: Requests are processed asynchronously to avoid blocking the UI.
- **Caching**: Results are stored in the database to avoid regenerating the same content.
- **Pagination**: When retrieving large sets of data, pagination is implemented to improve performance.

## 11. Future Enhancements

- **Streaming Responses**: Implement streaming responses for real-time feedback during generation.
- **Custom Prompts**: Allow users to customize the prompts sent to AI models.
- **Fine-tuning**: Support for fine-tuned models for better quiz generation.
- **Batch Processing**: Support for generating multiple quizzes in a batch.
- **Advanced Filtering**: More advanced filtering options for source content.
- **Multi-modal Support**: Better support for generating quizzes from images, audio, and video.
- **Feedback Loop**: Allow users to provide feedback on generated quizzes to improve future generations.
