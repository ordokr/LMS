# Decision Log

## Decisions
1. **Integration of Canvas LMS**
   - **Context**: Initial integration of Canvas LMS.
   - **Decision**: Use RESTful API endpoints for communication.
   - **Rationale**: RESTful APIs are widely supported and provide a flexible way to integrate external systems.
   - **Implementation Details**: Implemented in `src/services/canvas_service.rs`.

2. **Database Choice**
   - **Context**: Selection of database for the project.
   - **Decision**: Use SQLite with sqlx for local data storage.
   - **Rationale**: SQLite is lightweight and easy to set up, suitable for offline-first applications.
   - **Implementation Details**: Configured in `src/config/database.rs`.