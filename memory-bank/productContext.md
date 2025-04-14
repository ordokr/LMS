# Product Context

## Project Overview
The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.

## Goals
- **Canvas Course Management**: Workflow states and API endpoints finalized.
- **Discourse Forums**: Integration planned.
- **Blockchain Certification**: Certificate creation and verification integrated with the LMS.
- **Batch Model Generation**: Rust models updated to align with the latest configurations.

## Features
- **Frontend**: Leptos, Tauri
- **Backend**: Rust, Haskell
- **Database**: SQLite, sqlx
- **Search**: MeiliSearch
- **AI Integration**: Local AI Model implementation via LM Studio or the like
- **Blockchain**: Custom Rust implementation
- **Authentication**: JWT

## Architecture Principles
- Clean Architecture
- SOLID
- Offline-first

## Design Patterns
- CQRS
- Event Sourcing
- Repository Pattern