# Task Scheduler

A simple RESTful task management API built with **Rust**, **Axum**, and **PostgreSQL** (via sqlx).

Users can create, retrieve, and delete tasks associated with their account. The project focuses on clean API design, basic rate limiting, logging, and progressive security improvements.

> **Status**: Active development 
> **Primary goals**: Practice Rust web development, authentication, and secure API patterns for real use cases.
> **CAP**: in event of failure the focus will be on Consistency and Availability. 

## Features

- Create and retrieve tasks per user
- Basic user management endpoints
- Rate limiting (tower-governor / axum-limit)
- Structured logging
- CORS support
- Environment-based configuration
- PostgreSQL persistence

**Coming soon / In progress**
- Proper JWT authentication
- Password hashing
- Input validation improvements
- Search functionality
- Validation of Ports

## Tech Stack

| Component       | Technology              |
|----------------|-------------------------|
| Language       | Rust                   |
| Web Framework  | Axum 0.8               |
| Database       | PostgreSQL + sqlx      |
| Rate Limiting  | tower-governor, axum-limit |
| Logging        | log + simple-logging   |
| Serialization  | serde / serde_json     |

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- A `.env` file (see below)

### Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost:####/task_scheduler
LOG_FILE=app.log
JWT_SECRET=Super-Secret-key  # for future JWT work
HOST=127.0.0.1                                              #will be used by port checker



# Clone the repository
git clone https://github.com/Laplaza1/task_scheduler.git
cd task_scheduler

# Make sure PostgreSQL is running and the database exists
# (the current version recreates tables on startup – this will change)

# Run
cargo run




