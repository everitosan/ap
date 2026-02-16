# Backend Architecture

## Overview

Rust HTTP server using **Clean Architecture** with **Vertical Slicing**. Each feature is a self-contained module with its own domain, application, and infrastructure layers.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | Actix-web 4 |
| Runtime | Tokio |
| Database | PostgreSQL 15 + SQLx |
| Serialization | Serde |

## Project Structure

```
back/
├── src/
│   ├── main.rs                 # Entry point, bootstrap, graceful shutdown
│   ├── lib.rs                  # Public re-exports
│   │
│   ├── config/                 # Configuration
│   │   ├── settings.rs         # Env vars loading
│   │   └── database.rs         # PgPool creation
│   │
│   ├── server/                 # HTTP server
│   │   ├── graceful_shutdown.rs  # SIGTERM/SIGINT handling
│   │   └── routes.rs           # Route aggregation from modules
│   │
│   ├── shared/                 # Shared utilities
│   │   ├── app_state.rs        # AppState with db_pool
│   │   ├── error.rs            # AppError enum
│   │   └── response.rs         # ApiResponse<T>
│   │
│   └── modules/                # Feature modules (vertical slices)
│       └── <feature>/
│           ├── domain/         # Entities + Repository traits
│           ├── application/    # Use cases
│           └── infrastructure/ # DB repos + HTTP handlers
│
├── specs/                      # Documentation
│   ├── schema.md               # DB schema spec
│   ├── endpoints.md            # API endpoints spec
│   └── arq.md                  # This file
│
└── utils/dev/                  # Development tools
    ├── .env                    # Docker env vars
    ├── compose.yml             # PostgreSQL + pgAdmin
    └── schema.sql              # DB initialization
```

## Clean Architecture Layers

```
┌─────────────────────────────────────────┐
│           INFRASTRUCTURE                │
│  ┌─────────────────┐ ┌────────────────┐ │
│  │ postgres_repo   │ │   handlers     │ │
│  │ (DB adapter)    │ │ (HTTP adapter) │ │
│  └────────┬────────┘ └───────┬────────┘ │
└───────────┼──────────────────┼──────────┘
            │                  │
            ▼                  ▼
┌─────────────────────────────────────────┐
│            APPLICATION                  │
│  ┌─────────────────────────────────────┐│
│  │           Use Cases                 ││
│  │  (orchestrate domain + ports)       ││
│  └─────────────────────────────────────┘│
└────────────────────┬────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│              DOMAIN                     │
│  ┌──────────────┐  ┌─────────────────┐  │
│  │  Entities    │  │ Repository Trait│  │
│  │ (pure data)  │  │    (ports)      │  │
│  └──────────────┘  └─────────────────┘  │
└─────────────────────────────────────────┘
```

**Dependency Rule**: Inner layers know nothing about outer layers.

## Module Pattern

Each feature module follows this structure:

```
modules/<feature>/
├── mod.rs
├── domain/
│   ├── mod.rs
│   ├── <entity>.rs       # Domain entity (pure struct)
│   └── repository.rs     # Trait defining persistence contract
├── application/
│   ├── mod.rs
│   └── <use_case>.rs     # Business logic orchestration
└── infrastructure/
    ├── mod.rs
    ├── postgres_repository.rs  # Implements domain::Repository
    └── handlers.rs             # HTTP handlers (actix-web)
```

## Adding a New Module

1. Create directory structure under `src/modules/<name>/`
2. Define entity in `domain/<entity>.rs`
3. Define repository trait in `domain/repository.rs`
4. Implement use case in `application/<action>.rs`
5. Implement repository in `infrastructure/postgres_repository.rs`
6. Create HTTP handlers in `infrastructure/handlers.rs`
7. Register module in `src/modules/mod.rs`
8. Add routes in `src/server/routes.rs`

## Shared Components

### AppState
```rust
pub struct AppState {
    pub db_pool: PgPool,
}
```

### ApiResponse
```rust
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}
```
- `success: true` → data contains result
- `success: false` → data contains error message

### AppError
```rust
pub enum AppError {
    Internal(String),    // 500 - Unexpected errors
    NotFound(String),    // 404 - Resource not found
    BadRequest(String),  // 400 - Invalid input
    Conflict(String),    // 409 - Duplicate/conflict
    Database(String),    // 500 - DB errors (logged, generic response)
}
```

## Graceful Shutdown

The server handles SIGTERM/SIGINT signals:
1. Stops accepting new connections
2. Waits up to 30s for active requests to complete
3. Closes database pool
4. Exits cleanly

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| POSTGRES_USER | postgres | DB user |
| POSTGRES_PASSWORD | postgres | DB password |
| POSTGRES_HOST | localhost | DB host |
| POSTGRES_PORT | 5432 | DB port |
| POSTGRES_DB | postgres | DB name |
| SERVER_HOST | 127.0.0.1 | Server bind host |
| SERVER_PORT | 8080 | Server bind port |

## Development

```bash
# Start database
cd back/utils/dev && docker compose up -d

# Run server
cd back && cargo run

# Test endpoints
curl http://localhost:9002/api/v1/health
curl http://localhost:9002/api/v1/topics
```

## Current Modules

| Module | Endpoints | Description |
|--------|-----------|-------------|
| topics | GET /api/v1/topics | Returns all topics from catalogue |
