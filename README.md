# Rust User API

A simple REST API built with Rust, Axum, and PostgreSQL for managing users.

## Features

- CRUD operations for users
- PostgreSQL database with SQLx
- Automatic database migrations
- Docker Compose setup
- Async/await with Tokio

## Tech Stack

- **Axum** - Web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization

## Prerequisites

- Rust (latest stable)
- Docker & Docker Compose
- Git

## Quick Start

1. **Clone the repository**
```bash
git clone https://github.com/AbdulAbdullah/axum.git
cd axum
```

2. **Set up environment variables**
```bash
cp .env.example .env
```

Edit `.env` and update if needed:
```env
DATABASE_URL=postgresql://user:password@localhost:5432/simple_api
```

3. **Start the database**
```bash
docker compose up -d db
```

4. **Run the application**
```bash
cargo run
```

The API will be available at `http://localhost:8000`

## API Endpoints

### Base
- `GET /` - Welcome message

### Users
- `GET /users` - Get all users
- `GET /users/:id` - Get user by ID
- `POST /users` - Create a new user
- `PUT /users/:id` - Update a user
- `DELETE /users/:id` - Delete a user

## Usage Examples

### Create a user
```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

### Get all users
```bash
curl http://localhost:8000/users
```

### Get a specific user
```bash
curl http://localhost:8000/users/1
```

### Update a user
```bash
curl -X PUT http://localhost:8000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "email": "jane@example.com"}'
```

### Delete a user
```bash
curl -X DELETE http://localhost:8000/users/1
```

## Project Structure

```
.
├── src/
│   └── main.rs          # Main application code
├── migrations/          # Database migrations
├── Cargo.toml          # Rust dependencies
├── compose.yml         # Docker Compose configuration
├── Dockerfile          # Docker image configuration
└── README.md           # This file
```

## Database Migrations

Migrations are automatically run on startup. To create a new migration:

```bash
sqlx migrate add <migration_name>
```

## Docker

### Run everything with Docker
```bash
docker compose up --build
```

### Stop services
```bash
docker compose down
```

### View logs
```bash
docker compose logs -f
```

## Development

### Run tests
```bash
cargo test
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```
