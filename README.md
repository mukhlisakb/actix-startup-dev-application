# Actix Startup Template

A Rust web application template using Actix-web, SQLx, and a modular architecture.

## Features

- **Actix-web**: High-performance web framework.
- **SQLx**: Async database driver for PostgreSQL.
- **Modular Structure**: Separated into `api`, `core` (domain), `infrastructure`, and `shared` crates.
- **Health Check**: Built-in health check endpoint.
- **Database Pool**: Configured connection pooling with timeouts and limits.

## Prerequisites

- Rust (latest stable)
- PostgreSQL (running locally or via Docker)

## Setup

1. **Environment Variables**:
   Copy `.env.example` to `.env` (create if not exists) and set your database credentials.
   ```bash
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
   ```

2. **Run Database**:
   You can use the provided `docker-compose.yml`:
   ```bash
   docker-compose up -d
   ```

3. **Run Application**:
   ```bash
   cargo run -p api
   ```

## API Endpoints

### Health Check

- **URL**: `/health`
- **Method**: `GET`
- **Success Response**:
  ```json
  {
    "status": "OK",
    "timestamp": "2023-10-27T10:00:00+00:00"
  }
  ```

## Project Structure

- `api`: HTTP layer, routes, and request handlers.
- `core`: Business logic and domain entities (renamed to `app_core` in Cargo to avoid conflicts).
- `infrastructure`: Database connections and external services.
- `shared`: Common utilities, configuration, and error types.

## Testing

Run unit tests:
```bash
cargo test
```

## Troubleshooting

- **Database Connection Error**: Ensure Postgres is running and the connection string in `DatabaseConfig` (or `.env` if implemented) is correct.
- **Compilation Error (prelude)**: If you see errors about `core` prelude, ensure `app_core` is used instead of `core` for the internal crate.
