# cmi5 Implementation Guide for Content Developers

## Introduction

This guide provides practical information for content developers who want to create cmi5-compliant content for use with Ordo LMS. cmi5 is an xAPI Profile that standardizes the communication between learning content and Learning Management Systems (LMSs).

## Getting Started

### Prerequisites

To develop cmi5-compliant content, you'll need:

1. Basic understanding of HTML, JavaScript, and web development
2. Familiarity with xAPI concepts (statements, actors, verbs, objects)
3. A text editor or IDE for web development
4. Access to a cmi5-compliant LMS for testing (Ordo LMS provides this)

### Basic Structure of cmi5 Content

A cmi5 course consists of one or more Assignable Units (AUs). Each AU is a standalone learning object that can be launched by the LMS. The structure of a cmi5 course is defined in a `cmi5.xml` file.

## Creating a Simple cmi5 Course

### Step 1: Create the Course Structure

Create a `cmi5.xml` file that defines your course structure:

```xml
<?xml version="1.0" encoding="utf-8"?>
<courseStructure xmlns="https://w3id.org/xapi/profiles/cmi5/v1/CourseStructure.xsd">
  <course id="https://example.com/courses/intro-to-cmi5">
    <title>Introduction to cmi5</title>
    <description>A simple introduction to cmi5 for content developers</description>
    <au id="https://example.com/courses/intro-to-cmi5/module1">
      <title>Module 1: Getting Started</title>
      <description>Introduction to cmi5 concepts</description>
      <url>content/module1/index.html</url>
      <moveOn>Completed</moveOn>
    </au>
    <au id="https://example.com/courses/intro-to-cmi5/module2">
      <title>Module 2: Assessment</title>
      <description>Test your knowledge of cmi5</description>
      <url>content/module2/index.html</url>
      <moveOn>Passed</moveOn>
      <masteryScore>0.8</masteryScore>
    </au>
  </course>
</courseStructure>
```

### Step 2: Create the Content Files

For each AU, create the necessary HTML, CSS, and JavaScript files. Here's a simple example for the first module:

**content/module1/index.html**:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Module 1: Getting Started</title>
    <link rel="stylesheet" href="styles.css">
    <script src="cmi5.js"></script>
</head>
<body>
    <div class="container">
        <h1>Module 1: Getting Started</h1>
        <div class="content">
            <p>Welcome to the introduction to cmi5!</p>
            <!-- Your content here -->
        </div>
        <button id="complete-button">Mark Complete</button>
    </div>
    <script src="script.js"></script>
</body>
</html>
```

**content/module1/script.js**:

```javascript
// Initialize cmi5
const cmi5 = new CMI5();

// Wait for the AU to be fully initialized
cmi5.initialize().then(() => {
    console.log('AU initialized successfully');
    
    // Set up the complete button
    document.getElementById('complete-button').addEventListener('click', () => {
        // Send the completed statement
        cmi5.completed().then(() => {
            // Send the terminated statement and close the session
            cmi5.terminate();
            alert('Module completed successfully!');
        }).catch(error => {
            console.error('Error marking completion:', error);
        });
    });
}).catch(error => {
    console.error('Error initializing cmi5:', error);
});
```

### Step 3: Create the cmi5 Client Library

Create a `cmi5.js` file that handles the communication with the LMS:

```javascript
class CMI5 {
    constructor() {
        // Parse the launch parameters from the URL
        this.params = this.parseParams();
        this.initialized = false;
        this.endpoint = this.params.endpoint;
        this.fetchUrl = this.params.fetch;
        this.actor = JSON.parse(this.params.actor);
        this.activityId = this.params.activityId;
        this.registration = this.params.registration;
    }
    
    // Parse URL parameters
    parseParams() {
        const params = {};
        const urlParams = new URLSearchParams(window.location.search);
        
        params.endpoint = urlParams.get('endpoint');
        params.actor = urlParams.get('actor');
        params.activityId = urlParams.get('activityId');
        params.registration = urlParams.get('registration');
        params.authToken = urlParams.get('auth-token');
        
        return params;
    }
    
    // Initialize the AU
    async initialize() {
        if (this.initialized) {
            return Promise.resolve();
        }
        
        // Create the initialized statement
        const statement = {
            actor: this.actor,
            verb: {
                id: 'http://adlnet.gov/expapi/verbs/initialized',
                display: {
                    'en-US': 'initialized'
                }
            },
            object: {
                id: this.activityId,
                objectType: 'Activity'
            },
            context: this.getContext()
        };
        
        // Send the statement
        try {
            await this.sendStatement(statement);
            this.initialized = true;
            return Promise.resolve();
        } catch (error) {
            return Promise.reject(error);
        }
    }
    
    // Mark the AU as completed
    async completed() {
        if (!this.initialized) {
            return Promise.reject(new Error('AU not initialized'));
        }
        
        // Create the completed statement
        const statement = {
            actor: this.actor,
            verb: {
                id: 'http://adlnet.gov/expapi/verbs/completed',
                display: {
                    'en-US': 'completed'
                }
            },
            object: {
                id: this.activityId,
                objectType: 'Activity'
            },
            result: {
                completion: true
            },
            context: this.getContext()
        };
        
        // Send the statement
        try {
            await this.sendStatement(statement);
            return Promise.resolve();
        } catch (error) {
            return Promise.reject(error);
        }
    }
    
    // Mark the AU as passed
    async passed(score) {
        if (!this.initialized) {
            return Promise.reject(new Error('AU not initialized'));
        }
        
        // Create the passed statement
        const statement = {
            actor: this.actor,
            verb: {
                id: 'http://adlnet.gov/expapi/verbs/passed',
                display: {
                    'en-US': 'passed'
                }
            },
            object: {
                id: this.activityId,
                objectType: 'Activity'
            },
            result: {
                success: true,
                score: score
            },
            context: this.getContext()
        };
        
        // Send the statement
        try {
            await this.sendStatement(statement);
            return Promise.resolve();
        } catch (error) {
            return Promise.reject(error);
        }
    }
    
    // Mark the AU as failed
    async failed(score) {
        if (!this.initialized) {
            return Promise.reject(new Error('AU not initialized'));
        }
        
        // Create the failed statement
        const statement = {
            actor: this.actor,
            verb: {
                id: 'http://adlnet.gov/expapi/verbs/failed',
                display: {
                    'en-US': 'failed'
                }
            },
            object: {
                id: this.activityId,
                objectType: 'Activity'
            },
            result: {
                success: false,
                score: score
            },
            context: this.getContext()
        };
        
        // Send the statement
        try {
            await this.sendStatement(statement);
            return Promise.resolve();
        } catch (error) {
            return Promise.reject(error);
        }
    }
    
    // Terminate the AU
    async terminate() {
        if (!this.initialized) {
            return Promise.reject(new Error('AU not initialized'));
        }
        
        // Create the terminated statement
        const statement = {
            actor: this.actor,
            verb: {
                id: 'http://adlnet.gov/expapi/verbs/terminated',
                display: {
                    'en-US': 'terminated'
                }
            },
            object: {
                id: this.activityId,
                objectType: 'Activity'
            },
            context: this.getContext()
        };
        
        // Send the statement
        try {
            await this.sendStatement(statement);
            return Promise.resolve();
        } catch (error) {
            return Promise.reject(error);
        }
    }
    
    // Get the context for statements
    getContext() {
        return {
            registration: this.registration,
            contextActivities: {
                category: [{
                    id: 'https://w3id.org/xapi/cmi5/context/categories/cmi5'
                }]
            }
        };
    }
    
    // Send a statement to the LRS
    async sendStatement(statement) {
        // Add timestamp if not present
        if (!statement.timestamp) {
            statement.timestamp = new Date().toISOString();
        }
        
        // Add statement ID if not present
        if (!statement.id) {
            statement.id = this.generateUUID();
        }
        
        // Send the statement to the LRS
        const response = await fetch(`${this.endpoint}/statements`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.params.authToken}`,
                'X-Experience-API-Version': '1.0.3'
            },
            body: JSON.stringify(statement)
        });
        
        if (!response.ok) {
            throw new Error(`Failed to send statement: ${response.statusText}`);
        }
        
        return response.json();
    }
    
    // Generate a UUID
    generateUUID() {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
            const r = Math.random() * 16 | 0;
            const v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
}
```

### Step 4: Package the Course

1. Create a ZIP file containing:
   - `cmi5.xml` (at the root)
   - All content files in their respective directories

2. The ZIP file structure should look like:
   ```
   my-cmi5-course.zip
   ├── cmi5.xml
   └── content/
       ├── module1/
       │   ├── index.html
       │   ├── script.js
       │   ├── cmi5.js
       │   └── styles.css
       └── module2/
           ├── index.html
           ├── script.js
           ├── cmi5.js
           └── styles.css
   ```

## Advanced Topics

### Move On Criteria

cmi5 defines several "moveOn" criteria that determine when an AU is considered complete:

- **Completed**: The AU must send a "completed" statement
- **Passed**: The AU must send a "passed" statement
- **CompletedAndPassed**: The AU must send both "completed" and "passed" statements
- **CompletedOrPassed**: The AU must send either a "completed" or "passed" statement
- **NotApplicable**: No criteria; the AU is considered complete when launched

### Mastery Score

For AUs with a "Passed" moveOn criterion, you can specify a mastery score:

```xml
<au id="https://example.com/courses/intro-to-cmi5/quiz">
  <title>Final Quiz</title>
  <url>content/quiz/index.html</url>
  <moveOn>Passed</moveOn>
  <masteryScore>0.8</masteryScore>
</au>
```

This indicates that the learner must achieve a score of at least 80% to pass the AU.

### cmi5-Allowed Statements

In addition to the required cmi5-defined statements (initialized, completed, passed, failed, terminated), you can send additional "cmi5-allowed" statements to track more detailed learning experiences:

```javascript
// Send a cmi5-allowed statement
async function sendCustomStatement(verb, result) {
    const statement = {
        actor: cmi5.actor,
        verb: verb,
        object: {
            id: cmi5.activityId,
            objectType: 'Activity'
        },
        result: result,
        context: cmi5.getContext()
    };
    
    return cmi5.sendStatement(statement);
}

// Example: Track a question response
sendCustomStatement(
    {
        id: 'http://adlnet.gov/expapi/verbs/answered',
        display: {
            'en-US': 'answered'
        }
    },
    {
        response: 'Paris',
        success: true
    }
);
```

## Testing and Debugging

### Testing with Ordo LMS

1. Import your cmi5 package into Ordo LMS
2. Launch the course
3. Check the browser console for any errors
4. Verify that statements are being sent correctly

### Common Issues and Solutions

1. **AU not initializing**:
   - Check that you're correctly parsing the launch parameters
   - Verify that the auth token is being used correctly

2. **Statements not being sent**:
   - Check for JavaScript errors in the console
   - Verify that the endpoint URL is correct
   - Check that the auth token is valid

3. **AU not marked as complete**:
   - Ensure you're sending the correct statements based on the moveOn criteria
   - Check that the statements have the correct format

## Best Practices

1. **Initialize First**: Always initialize the AU before sending any other statements

2. **Terminate Last**: Always terminate the AU when the learner exits

3. **Handle Errors**: Implement proper error handling for all API calls

4. **Test Thoroughly**: Test your content in multiple browsers and devices

5. **Provide Feedback**: Give learners clear feedback about their progress

6. **Respect Privacy**: Only track information that is necessary for learning

## Resources

- [Official cmi5 Specification](https://aicc.github.io/CMI-5_Spec_Current/)
- [cmi5 Best Practices](https://aicc.github.io/CMI-5_Spec_Current/best_practices/)
- [cmi5 Sample Implementations](https://aicc.github.io/CMI-5_Spec_Current/samples/)
- [xAPI Specification](https://github.com/adlnet/xAPI-Spec)
- [Ordo LMS Documentation](https://docs.ordolms.com)

## Conclusion

By following this guide, you can create cmi5-compliant content that works seamlessly with Ordo LMS. cmi5 provides a modern, flexible approach to e-learning interoperability, allowing you to create more engaging and effective learning experiences.
