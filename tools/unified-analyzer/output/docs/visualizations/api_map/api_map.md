# API Map

This document provides a comprehensive map of all API endpoints in the application.

## Table of Contents

- [CoursesController](#coursescontroller)
- [UsersController](#userscontroller)

## CoursesController

### GET Endpoints

| Path | Description | Auth Required | Parameters |
|------|-------------|--------------|------------|
| `/api/v1/courses` | Get all courses | Yes |  |

## UsersController

### GET Endpoints

| Path | Description | Auth Required | Parameters |
|------|-------------|--------------|------------|
| `/api/v1/users` | Get all users | Yes |  |
| `/api/v1/users/{id}` | Get a specific user | Yes | id |

## API Flow Diagram

```mermaid
graph LR
    Client[Client]
    _api_v1_users["GET /api/v1/users"]
    Client --> _api_v1_users
    UsersController["UsersController"]
    _api_v1_users --> UsersController
    _api_v1_users_id["GET /api/v1/users/{id}"]
    Client --> _api_v1_users_id
    UsersController["UsersController"]
    _api_v1_users_id --> UsersController
    _api_v1_courses["GET /api/v1/courses"]
    Client --> _api_v1_courses
    CoursesController["CoursesController"]
    _api_v1_courses --> CoursesController
```
