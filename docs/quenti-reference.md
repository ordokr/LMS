# Quenti Reference

## Overview

This document provides information about the Quenti source code that was used as a reference for implementing certain features in the Ordo LMS Quiz Module.

## Source Code Location

The Quenti source code is located at:

```
C:\Users\Tim\Desktop\quenti
```

This location should be referenced when implementing features that need to maintain compatibility with Quenti's functionality.

## Key Components

When porting features from Quenti to the Ordo LMS Quiz Module, the following key components should be considered:

1. **Quiz Data Model**: Ensure compatibility with Quenti's quiz data structure
2. **Question Types**: Maintain support for all question types implemented in Quenti
3. **UI/UX**: Preserve the user experience where appropriate
4. **API Endpoints**: Ensure API compatibility for external integrations

## Standards Implementation

For external system integration, the Ordo LMS Quiz Module uses:

1. **cmi5**: Primary standard for LMS integration
   - Modern xAPI-based tracking
   - Support for distributed content
   - Mobile and offline learning support

2. **SCORM**: Backup standard for compatibility
   - Support for legacy content
   - Compatibility with older LMS platforms

This differs from Quenti's implementation, which primarily uses SCORM. The decision to use cmi5 as the primary standard was made to leverage its modern architecture and improved capabilities while maintaining backward compatibility through SCORM support.

## Implementation Notes

When implementing features from Quenti, consider the following:

1. **Modernize**: Update implementations to use current best practices
2. **Optimize**: Improve performance where possible
3. **Extend**: Add new capabilities while maintaining compatibility
4. **Document**: Clearly document any deviations from Quenti's implementation

## References

- [Quenti Documentation](https://docs.quenti.io)
- [cmi5 Specification](https://aicc.github.io/CMI-5_Spec_Current/)
- [SCORM Documentation](https://scorm.com/scorm-explained/technical-scorm/)
- [xAPI Specification](https://github.com/adlnet/xAPI-Spec)
