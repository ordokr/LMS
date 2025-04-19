# Migration Plan: Discourse and Canvas to New System

> **Important Disclaimer:** This plan describes the process of porting and transforming source code, models, and features from Canvas LMS and Discourse to Ordo. It does **not** cover data migration, user import, or live system integration. All references to “migration,” “integration,” or “import” refer solely to source code, schema, or feature porting, not to data or live system interoperability.

## Overview

This document outlines the strategy for porting the core functionalities of Discourse and Canvas LMS to a new, unified system built on Rust, Tauri, and Leptos. The porting process will be phased, focusing on the most critical features first to ensure a smooth transition.

For a detailed list of models and controllers to be ported, see [MODEL_CONTROLLER_MIGRATION.md](MODEL_CONTROLLER_MIGRATION.md).

## Guiding Principles

1. **Incremental Porting:** We will avoid a "big bang" rewrite. Instead, we will port features incrementally, testing each thoroughly before moving to the next.
2. **Feature Parity:** The goal is to replicate the core functionality of both platforms in the new system. We will prioritize the most used features.
3. **Code Integrity:** Ensuring the accuracy and consistency of code and schema during the porting process is paramount. We will validate functionality after each phase.
4. **User Experience:** We will strive to minimize disruption for users during the porting process and ensure that the new system provides a seamless user experience.
5. **Offline first:** We will make sure the new system provides full offline capabilities.

## Porting Phases

### Phase 1: Core User and Authentication System

- **Objective:** Establish the fundamental user management and authentication framework in the new system.
- **Tasks:**
  - Port user models (users, roles, permissions) from both platforms to the unified user model in the new system.
  - Implement authentication mechanisms (login, logout, session management, password management).
  - Implement authorization checks and role-based access control.
  - Set up user profile management (profile viewing, editing).
  - Test user creation, authentication, and authorization thoroughly.
  - **Offline Support**: Ensure full offline support for user login and data synchronization.

### Phase 2: Core Course and Category Management

- **Objective:** Replicate the basic course and category structure of Canvas and Discourse.
- **Tasks:**
  - Port course models and schema from Canvas.
  - Port category models and schema from Discourse.
  - Implement course creation, editing, and deletion.
  - Implement category creation, editing, and deletion.
  - Establish relationships between courses and categories.
  - Test course and category management features.
  - **Offline Support**: Provide course and category creation and edition offline.

### Phase 3: Forum and Discussion Functionality

- **Objective:** Recreate the core forum and discussion features of Discourse.
- **Tasks:**
  - Port topic models and schema.
  - Port post models and schema.
  - Implement topic creation, editing, and deletion.
  - Implement post creation, editing, and deletion.
  - Implement topic viewing and participation features.
  - Test the forum and discussion functionality thoroughly.
  - **Offline Support:** Allow users to create posts and topics offline.

### Phase 4: Canvas Module and Assignment Porting

- **Objective:** Port Canvas module and assignment structures into the unified system.
- **Tasks:**
  - Port module models and schema.
  - Port assignment models and schema.
  - Implement module management (creation, editing, deletion).
  - Implement assignment management (creation, editing, submission, grading).
  - Establish relationships between modules, assignments, and courses.
  - Test the Canvas-specific module and assignment features.
  - **Offline Support:** Create and edit modules and assignments offline.

### Phase 5: Advanced Features and Integrations

*   **Objective:** Implement advanced features and port integrations with external systems as needed.
*   **Tasks:**
    *   Implement search functionality.
    *   Implement notification systems.
    *   Implement advanced reporting and analytics.
    *   Port integrations with external systems if needed (code only, not data).
    *   Thoroughly test all advanced features.
    *   **Offline Support**: Provide support for the most important advanced functionalities offline.

### Phase 6: Testing and Deployment

* **Objective:** Prepare the new platform for release.
* **Tasks**:
    * Perform extensive integration and E2E tests.
    * Perform user acceptance tests with a focus group.
    * Deploy the new platform.
    * **Offline Support:** Check all offline capabilities.


## Related Documents

- [MODEL_CONTROLLER_MIGRATION.md](MODEL_CONTROLLER_MIGRATION.md): Detailed list of models and controllers to be ported
- [INTEGRATION_PLAN.md](INTEGRATION_PLAN.md): Plan for porting Canvas and Discourse functionality
- [migration_guide.md](../migration_guide.md): Guide for porting from Canvas and Discourse to Ordo
- [haskell_integration.md](../architecture/haskell_integration.md): Guidelines for when to use Haskell in the codebase

This porting plan will be updated and adapted as we make progress through each phase.