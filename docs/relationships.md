# Project Relationship Map
_Generated on 2025-04-04_

## Model Relationships

```mermaid
classDiagram
```

## UI Component Dependencies

```mermaid
flowchart TD
```

## Project Architecture

```mermaid
flowchart TD
    UI[UI Components] --> API[API Layer]
    API --> Models[Data Models]
    API --> Services[Services]
    Services --> Database[(Database)]
    Services --> ExternalAPI[External APIs]
```

