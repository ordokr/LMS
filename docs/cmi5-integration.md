# cmi5 Integration in Ordo LMS

## Overview

Ordo LMS uses cmi5 as the primary standard for integration with Learning Management Systems (LMSs). cmi5 is an xAPI Profile designed to standardize the way LMSs launch, track, and report online courses using xAPI. It serves as a modern successor to SCORM and AICC, offering improved flexibility, extensibility, and support for modern learning scenarios.

## Why cmi5?

cmi5 offers several advantages over older standards like SCORM:

1. **Modern Architecture**: Built on xAPI, supporting distributed content and modern web technologies
2. **Content Location Independence**: Content can be hosted anywhere, not just within the LMS
3. **Improved Tracking**: More detailed and flexible tracking of learning experiences
4. **Mobile Support**: Better support for mobile and offline learning scenarios
5. **Extensibility**: Can track any learning experience while maintaining interoperability
6. **Simplified Implementation**: Clearer specification with fewer ambiguities

## Implementation Details

### Core Components

The cmi5 implementation in Ordo LMS consists of the following components:

1. **Cmi5Service**: Main service for managing cmi5 courses and sessions
2. **Cmi5Client**: Client for communicating with the Learning Record Store (LRS)
3. **CourseStructure**: Parser and manager for cmi5 course packages
4. **LaunchService**: Service for launching cmi5 content
5. **Statements**: Builders and utilities for creating xAPI statements

### Course Structure

cmi5 courses are defined by a course structure, which is typically provided in a `cmi5.xml` file. The course structure defines:

- Course metadata (title, description, etc.)
- Assignable Units (AUs) - the individual learning objects
- Launch parameters for each AU
- Completion and success criteria

Example course structure:

```xml
<courseStructure xmlns="https://w3id.org/xapi/profiles/cmi5/v1/CourseStructure.xsd">
  <course id="https://example.com/courses/sample-course">
    <title>Sample Course</title>
    <description>A sample course demonstrating cmi5</description>
    <au id="https://example.com/courses/sample-course/module1">
      <title>Module 1</title>
      <url>https://example.com/content/module1/index.html</url>
      <moveOn>Completed</moveOn>
    </au>
    <au id="https://example.com/courses/sample-course/module2">
      <title>Module 2</title>
      <url>https://example.com/content/module2/index.html</url>
      <moveOn>Passed</moveOn>
      <masteryScore>0.8</masteryScore>
    </au>
  </course>
</courseStructure>
```

### Launch Process

The cmi5 launch process follows these steps:

1. The LMS initiates the launch by redirecting to the AU's launch URL with cmi5 parameters
2. The AU retrieves the auth token and establishes a session with the LRS
3. The AU sends an "initialized" statement to the LRS
4. The AU delivers content and tracks progress using xAPI statements
5. The AU sends "completed", "passed/failed", and "terminated" statements as appropriate

### Statement Types

cmi5 defines several statement types that must be used in a specific order:

1. **Initialized**: Sent when the AU is first launched
2. **Completed**: Sent when the learner completes the AU
3. **Passed/Failed**: Sent to indicate success or failure
4. **Terminated**: Sent when the AU session ends
5. **Satisfied**: Sent when a learning objective is satisfied
6. **Abandoned**: Sent when a session is abandoned
7. **Waived**: Sent when an AU is waived (skipped)

### Integration with Ordo Quiz Module

The Quiz Module in Ordo LMS integrates with cmi5 to:

1. Import and export cmi5 course packages
2. Launch quizzes as cmi5 Assignable Units
3. Track quiz attempts and results using cmi5 statements
4. Report completion and success status to the LMS

## SCORM Compatibility

While cmi5 is the primary standard, Ordo LMS maintains compatibility with SCORM for legacy content and systems. The SCORM implementation serves as a backup when cmi5 is not supported by the target LMS.

Key differences in the implementation:

| Feature | cmi5 | SCORM |
|---------|------|-------|
| Content Location | Can be hosted anywhere | Must be hosted within the LMS |
| Data Model | xAPI statements | CMI data model |
| Tracking | Flexible, extensible | Limited to predefined elements |
| Mobile Support | Full support | Limited support |
| Offline Learning | Supported | Not supported |
| Implementation | Primary | Backup |

## Usage Examples

### Importing a cmi5 Course

```rust
use crate::quiz::cmi5::Cmi5Service;
use std::path::Path;

async fn import_course(cmi5_service: &Cmi5Service, package_path: &str) {
    let path = Path::new(package_path);
    match cmi5_service.import_course(path).await {
        Ok(course_id) => println!("Course imported successfully: {}", course_id),
        Err(e) => eprintln!("Failed to import course: {}", e),
    }
}
```

### Launching a cmi5 Assignable Unit

```rust
use crate::quiz::cmi5::{Cmi5Service, LaunchMode};

async fn launch_assignable_unit(
    cmi5_service: &Cmi5Service,
    course_id: &str,
    au_id: &str,
    actor_id: &str,
) {
    match cmi5_service.launch_assignable_unit(
        course_id,
        au_id,
        actor_id,
        LaunchMode::Normal,
    ).await {
        Ok(launch_url) => println!("Launch URL: {}", launch_url),
        Err(e) => eprintln!("Failed to launch AU: {}", e),
    }
}
```

### Completing a cmi5 Session

```rust
use crate::quiz::cmi5::{Cmi5Service, Cmi5Score};

async fn complete_session(
    cmi5_service: &Cmi5Service,
    session_id: &str,
    score: f64,
    success: bool,
) {
    let cmi5_score = Cmi5Score::percentage(score);
    
    match cmi5_service.complete_session(
        session_id,
        Some(cmi5_score),
        Some(success),
    ).await {
        Ok(_) => println!("Session completed successfully"),
        Err(e) => eprintln!("Failed to complete session: {}", e),
    }
}
```

## References

- [Official cmi5 Specification](https://aicc.github.io/CMI-5_Spec_Current/)
- [cmi5 Best Practices](https://aicc.github.io/CMI-5_Spec_Current/best_practices/)
- [cmi5 Sample Implementations](https://aicc.github.io/CMI-5_Spec_Current/samples/)
- [xAPI Specification](https://github.com/adlnet/xAPI-Spec)
- [Rustici Software cmi5 Player](https://rusticisoftware.com/products/cmi5-player/)

## Conclusion

cmi5 provides a modern, flexible standard for integrating Ordo LMS with external learning systems. By adopting cmi5 as the primary standard while maintaining SCORM compatibility, Ordo LMS offers the best of both worlds: forward-looking technology with backward compatibility.
