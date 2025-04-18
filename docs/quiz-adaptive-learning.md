# Quiz Module Adaptive Learning Paths

This document describes the adaptive learning paths feature implemented for the Quiz Module, allowing users to create personalized learning experiences.

## 1. Overview

The Quiz Module now includes a comprehensive adaptive learning system that:

- Allows creating learning paths with different types of nodes and conditional edges
- Supports personalized learning experiences based on user performance
- Tracks user progress through learning paths
- Provides recommendations for learning paths based on user history
- Is model-agnostic and platform-agnostic with a flexible architecture

This system enables educators to create adaptive learning experiences that adjust to each student's needs and performance.

## 2. Node and Edge Types

The system supports various node types for creating learning paths:

```rust
pub enum LearningPathNodeType {
    Quiz,
    Assessment,
    Content,
    Checkpoint,
    Start,
    End,
    Custom,
}
```

And multiple edge condition types for controlling the flow between nodes:

```rust
pub enum EdgeConditionType {
    Score,
    Completion,
    Time,
    Custom,
}
```

## 3. Data Models

### Adaptive Learning Path

The `AdaptiveLearningPath` represents a complete learning path with nodes and edges:

```rust
pub struct AdaptiveLearningPath {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub subject: String,
    pub tags: Vec<String>,
    pub nodes: Vec<LearningPathNode>,
    pub edges: Vec<LearningPathEdge>,
    pub default_study_mode: StudyMode,
    pub default_visibility: QuizVisibility,
    pub is_public: bool,
    pub usage_count: i32,
    pub rating: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Learning Path Node

The `LearningPathNode` represents a node in a learning path:

```rust
pub struct LearningPathNode {
    pub id: Uuid,
    pub path_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub node_type: LearningPathNodeType,
    pub content_id: Option<Uuid>,
    pub position_x: f32,
    pub position_y: f32,
    pub required_score: Option<f32>,
    pub custom_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Learning Path Edge

The `LearningPathEdge` represents an edge connecting two nodes in a learning path:

```rust
pub struct LearningPathEdge {
    pub id: Uuid,
    pub path_id: Uuid,
    pub source_node_id: Uuid,
    pub target_node_id: Uuid,
    pub condition_type: EdgeConditionType,
    pub condition_value: Option<serde_json::Value>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### User Learning Path Progress

The `UserLearningPathProgress` tracks a user's progress through a learning path:

```rust
pub struct UserLearningPathProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub path_id: Uuid,
    pub current_node_id: Uuid,
    pub completed_nodes: Vec<Uuid>,
    pub scores: std::collections::HashMap<String, f32>,
    pub started_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub custom_data: Option<serde_json::Value>,
}
```

## 4. Database Schema

The adaptive learning system uses several tables:

### Quiz Adaptive Learning Paths Table

```sql
CREATE TABLE IF NOT EXISTS quiz_adaptive_learning_paths (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author_id TEXT,
    subject TEXT NOT NULL,
    tags TEXT,
    default_study_mode TEXT NOT NULL,
    default_visibility TEXT NOT NULL,
    is_public INTEGER NOT NULL DEFAULT 0,
    usage_count INTEGER NOT NULL DEFAULT 0,
    rating REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    nodes TEXT,
    edges TEXT
);
```

### Quiz User Learning Path Progress Table

```sql
CREATE TABLE IF NOT EXISTS quiz_user_learning_path_progress (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    path_id TEXT NOT NULL,
    current_node_id TEXT NOT NULL,
    completed_nodes TEXT NOT NULL,
    scores TEXT NOT NULL,
    started_at TEXT NOT NULL,
    last_activity_at TEXT NOT NULL,
    completed_at TEXT,
    custom_data TEXT,
    FOREIGN KEY (path_id) REFERENCES quiz_adaptive_learning_paths (id) ON DELETE CASCADE
);
```

### Quiz Learning Path Recommendations Table

```sql
CREATE TABLE IF NOT EXISTS quiz_learning_path_recommendations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    path_id TEXT NOT NULL,
    score REAL NOT NULL,
    reason TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (path_id) REFERENCES quiz_adaptive_learning_paths (id) ON DELETE CASCADE
);
```

## 5. Core Functionality

### Creating and Managing Learning Paths

Learning paths can be created and managed:

```rust
// Create a new learning path
let path = adaptive_learning_service.create_path(
    title,
    description,
    author_id,
    subject,
    tags,
    default_study_mode,
    default_visibility,
    is_public,
).await?;

// Add a node to a learning path
let node = adaptive_learning_service.add_node(
    path_id,
    title,
    description,
    node_type,
    content_id,
    position_x,
    position_y,
    required_score,
    custom_data,
).await?;

// Add an edge to a learning path
let edge = adaptive_learning_service.add_edge(
    path_id,
    source_node_id,
    target_node_id,
    condition_type,
    condition_value,
    label,
).await?;
```

### User Progress and Navigation

Users can navigate through learning paths based on their performance:

```rust
// Start a learning path
let progress = adaptive_learning_service.start_path(
    user_id,
    path_id,
).await?;

// Complete a node in a learning path
let progress = adaptive_learning_service.complete_node(
    user_id,
    path_id,
    node_id,
    score,
).await?;

// Move to the next node in a learning path
let (progress, next_node) = adaptive_learning_service.move_to_next_node(
    user_id,
    path_id,
).await?;
```

### Recommendations

The system can generate recommendations for learning paths:

```rust
// Generate recommendations for a user
let recommendations = adaptive_learning_service.generate_recommendations(
    user_id,
    limit,
).await?;

// Get recommendations for a user
let recommendations = adaptive_learning_service.get_recommendations(
    user_id,
    limit,
).await?;
```

## 6. Tauri Commands

The following Tauri commands are available for the adaptive learning system:

### Learning Path Management Commands

```typescript
// Create a new learning path
const path = await invoke('create_learning_path', {
  title: 'Introduction to Programming',
  description: 'A beginner-friendly path to learn programming',
  authorId: '550e8400-e29b-41d4-a716-446655440000',
  subject: 'Computer Science',
  tags: ['programming', 'beginner'],
  defaultStudyMode: 'multiple_choice',
  defaultVisibility: 'private',
  isPublic: true,
});

// Add a node to a learning path
const node = await invoke('add_learning_path_node', {
  pathId: '550e8400-e29b-41d4-a716-446655440001',
  title: 'Variables and Data Types',
  description: 'Learn about variables and data types in programming',
  nodeType: 'Content',
  contentId: '550e8400-e29b-41d4-a716-446655440002',
  positionX: 100,
  positionY: 200,
  requiredScore: 0.7,
  customData: { difficulty: 'beginner' },
});

// Add an edge to a learning path
const edge = await invoke('add_learning_path_edge', {
  pathId: '550e8400-e29b-41d4-a716-446655440001',
  sourceNodeId: '550e8400-e29b-41d4-a716-446655440003',
  targetNodeId: '550e8400-e29b-41d4-a716-446655440004',
  conditionType: 'Score',
  conditionValue: { min_score: 0.7 },
  label: 'Score >= 70%',
});
```

### User Progress Commands

```typescript
// Start a learning path
const progress = await invoke('start_learning_path', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  pathId: '550e8400-e29b-41d4-a716-446655440001',
});

// Complete a node in a learning path
const progress = await invoke('complete_learning_path_node', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  pathId: '550e8400-e29b-41d4-a716-446655440001',
  nodeId: '550e8400-e29b-41d4-a716-446655440003',
  score: 0.85,
});

// Move to the next node in a learning path
const result = await invoke('move_to_next_learning_path_node', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  pathId: '550e8400-e29b-41d4-a716-446655440001',
});
```

### Recommendation Commands

```typescript
// Get recommendations for a user
const recommendations = await invoke('get_learning_path_recommendations', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 5,
});

// Generate new recommendations for a user
const recommendations = await invoke('generate_learning_path_recommendations', {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  limit: 5,
});
```

## 7. Frontend Integration

The adaptive learning system can be integrated into the frontend to provide a learning path experience:

### Learning Path Editor

```tsx
// In a learning path editor component
const [path, setPath] = useState(null);
const [nodes, setNodes] = useState([]);
const [edges, setEdges] = useState([]);
const [selectedNode, setSelectedNode] = useState(null);
const [selectedEdge, setSelectedEdge] = useState(null);

useEffect(() => {
  const fetchPath = async () => {
    if (pathId) {
      const path = await invoke('get_learning_path', {
        pathId,
      });
      setPath(path);
      setNodes(path.nodes);
      setEdges(path.edges);
    }
  };

  fetchPath();
}, [pathId]);

const handleAddNode = async (nodeData) => {
  try {
    const node = await invoke('add_learning_path_node', {
      pathId: path.id,
      title: nodeData.title,
      description: nodeData.description,
      nodeType: nodeData.nodeType,
      contentId: nodeData.contentId,
      positionX: nodeData.positionX,
      positionY: nodeData.positionY,
      requiredScore: nodeData.requiredScore,
      customData: nodeData.customData,
    });

    setNodes([...nodes, node]);
  } catch (error) {
    console.error('Error adding node:', error);
  }
};

const handleAddEdge = async (edgeData) => {
  try {
    const edge = await invoke('add_learning_path_edge', {
      pathId: path.id,
      sourceNodeId: edgeData.sourceNodeId,
      targetNodeId: edgeData.targetNodeId,
      conditionType: edgeData.conditionType,
      conditionValue: edgeData.conditionValue,
      label: edgeData.label,
    });

    setEdges([...edges, edge]);
  } catch (error) {
    console.error('Error adding edge:', error);
  }
};

return (
  <div className="learning-path-editor">
    <h2>{path?.title || 'New Learning Path'}</h2>

    <div className="editor-toolbar">
      <button onClick={() => setShowAddNodeModal(true)}>Add Node</button>
      <button onClick={() => setShowAddEdgeModal(true)}>Add Edge</button>
      <button onClick={handleSavePath}>Save Path</button>
    </div>

    <div className="editor-canvas">
      {/* Render nodes and edges using a graph visualization library */}
      <GraphCanvas
        nodes={nodes}
        edges={edges}
        onNodeClick={setSelectedNode}
        onEdgeClick={setSelectedEdge}
        onNodeDrag={handleNodeDrag}
      />
    </div>

    {selectedNode && (
      <div className="node-properties">
        <h3>Node Properties</h3>
        <p><strong>Title:</strong> {selectedNode.title}</p>
        <p><strong>Type:</strong> {selectedNode.node_type}</p>
        <button onClick={() => setShowEditNodeModal(true)}>Edit Node</button>
        <button onClick={() => handleDeleteNode(selectedNode.id)}>Delete Node</button>
      </div>
    )}

    {selectedEdge && (
      <div className="edge-properties">
        <h3>Edge Properties</h3>
        <p><strong>Condition:</strong> {selectedEdge.condition_type}</p>
        <p><strong>Label:</strong> {selectedEdge.label || 'No Label'}</p>
        <button onClick={() => setShowEditEdgeModal(true)}>Edit Edge</button>
        <button onClick={() => handleDeleteEdge(selectedEdge.id)}>Delete Edge</button>
      </div>
    )}

    {showAddNodeModal && (
      <NodeModal
        onSave={handleAddNode}
        onCancel={() => setShowAddNodeModal(false)}
      />
    )}

    {showAddEdgeModal && (
      <EdgeModal
        nodes={nodes}
        onSave={handleAddEdge}
        onCancel={() => setShowAddEdgeModal(false)}
      />
    )}
  </div>
);
```

### Learning Path Player

```tsx
// In a learning path player component
const [path, setPath] = useState(null);
const [progress, setProgress] = useState(null);
const [currentNode, setCurrentNode] = useState(null);
const [loading, setLoading] = useState(true);

useEffect(() => {
  const fetchPathAndProgress = async () => {
    try {
      setLoading(true);

      // Get the path
      const path = await invoke('get_learning_path', {
        pathId,
      });
      setPath(path);

      // Get or start progress
      try {
        const progress = await invoke('get_user_learning_path_progress', {
          userId: currentUser.id,
          pathId,
        });
        setProgress(progress);
      } catch (error) {
        // No progress found, start the path
        const progress = await invoke('start_learning_path', {
          userId: currentUser.id,
          pathId,
        });
        setProgress(progress);
      }
    } catch (error) {
      console.error('Error fetching path:', error);
    } finally {
      setLoading(false);
    }
  };

  fetchPathAndProgress();
}, [pathId, currentUser.id]);

useEffect(() => {
  // Set the current node based on progress
  if (path && progress) {
    const node = path.nodes.find(n => n.id === progress.current_node_id);
    setCurrentNode(node);
  }
}, [path, progress]);

const handleCompleteNode = async (score) => {
  try {
    const updatedProgress = await invoke('complete_learning_path_node', {
      userId: currentUser.id,
      pathId: path.id,
      nodeId: currentNode.id,
      score,
    });

    setProgress(updatedProgress);

    // Move to the next node
    const result = await invoke('move_to_next_learning_path_node', {
      userId: currentUser.id,
      pathId: path.id,
    });

    setProgress(result.progress);
    setCurrentNode(result.node);
  } catch (error) {
    console.error('Error completing node:', error);
  }
};

if (loading) {
  return <div>Loading...</div>;
}

return (
  <div className="learning-path-player">
    <h2>{path.title}</h2>

    <div className="path-progress">
      <div className="progress-bar">
        <div
          className="progress-fill"
          style={{ width: `${(progress.completed_nodes.length / path.nodes.length) * 100}%` }}
        />
      </div>
      <span className="progress-text">
        {progress.completed_nodes.length} / {path.nodes.length} completed
      </span>
    </div>

    {currentNode && (
      <div className="current-node">
        <h3>{currentNode.title}</h3>
        {currentNode.description && <p>{currentNode.description}</p>}

        {currentNode.node_type === 'Quiz' && currentNode.content_id && (
          <QuizPlayer
            quizId={currentNode.content_id}
            onComplete={(score) => handleCompleteNode(score)}
          />
        )}

        {currentNode.node_type === 'Content' && currentNode.content_id && (
          <ContentViewer
            contentId={currentNode.content_id}
            onComplete={() => handleCompleteNode(1.0)}
          />
        )}

        {currentNode.node_type === 'Checkpoint' && (
          <CheckpointComponent
            node={currentNode}
            onComplete={(score) => handleCompleteNode(score)}
          />
        )}

        {currentNode.node_type === 'End' && (
          <div className="path-completed">
            <h3>Congratulations!</h3>
            <p>You have completed this learning path.</p>
            <button onClick={() => navigate('/learning-paths')}>Browse More Paths</button>
          </div>
        )}
      </div>
    )}

    <div className="path-map">
      <PathMapVisualization
        path={path}
        progress={progress}
        currentNodeId={currentNode?.id}
      />
    </div>
  </div>
);
```

## 8. Model-Agnostic Architecture

The system is designed to be model-agnostic with a flexible architecture:

- **Node Types**: The system supports various node types (Quiz, Content, Assessment, etc.) that can be extended with custom types.
- **Edge Conditions**: Edge conditions determine how users progress through the learning path, with support for score-based, completion-based, time-based, and custom conditions.
- **Custom Data**: Both nodes and user progress can store custom data in a flexible JSON format, allowing for extension without schema changes.
- **Pluggable Content**: Nodes can reference any type of content through the `content_id` field, which can point to quizzes, videos, articles, or any other content type.

## 9. Future Enhancements

- **Advanced Analytics**: Implement more advanced analytics for learning paths, including heatmaps of user progress, bottleneck identification, and success rate analysis.
- **AI-driven Path Generation**: Integrate with the AI generation system to automatically create learning paths based on content and learning objectives.
- **Dynamic Path Adjustment**: Automatically adjust learning paths based on user performance, adding remedial content or skipping ahead as appropriate.
- **Collaborative Path Creation**: Allow multiple educators to collaborate on creating and refining learning paths.
- **Path Templates**: Create templates for common learning path structures that can be quickly customized.
- **Social Features**: Add social features like path sharing, comments, and ratings.
- **Gamification**: Add gamification elements like badges, points, and leaderboards to increase engagement.
- **Mobile Support**: Optimize the learning path experience for mobile devices.
- **Offline Mode**: Support offline progress through learning paths with synchronization when back online.
- **Accessibility Improvements**: Ensure learning paths are accessible to all users, including those with disabilities.