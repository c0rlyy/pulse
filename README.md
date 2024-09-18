# Rust Backend API

## Overview

This project is a Rust-based backend meant to be a chat app built with Axum and PostgreSQL. Currently It provides endpoints for user authentication and management, utilizing JWTs for secure access.

## Curent Features

- **User Authentication**: Login and token management.
- **User Management**: Retrieve user details.
- **User Search**: Lets you find users by name, exludes private users  
- **Error Handling**: Custom error responses for different scenarios.

# Todo

- **Websockets comuncation**
- **Stomp protocol**
- **Better Error Handling**
- **Integrate with message broker**
- **Add a dm feature**
- **Add a channel feature**

## Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/): Install Rust via [rustup](https://rustup.rs/).
- [PostgreSQL](https://www.postgresql.org/): Ensure PostgreSQL is installed and running.
- [Docker](https://www.docker.com/): For containerization (optional but recommended). Other Wise you will have to set up your own db

### Clone the Repository

```bash
git clone <https://github.com/c0rlyy/pulse.git>
cd <pulse>
```

Create a `.env` file in the root directory with the following content

if you used docker-compose make sure this reflects the crednetials there

```env
DATABASE_URL=postgres://username:password@localhost/dbname
HMAC_KEY=your_secret_key
```

Replace username, password, localhost, and dbname with your PostgreSQL configuration.
Build and Run

Build the Project:

```bash
cargo build
```

Run the Project and build it at the same time:

```bash
cargo run
```

The server will start on http://localhost:3000 by default.
## Docker Setup (Optional)

If you prefer to use Docker, you can build and run the application with Docker Compose. Ensure docker-compose.yml is properly configured.

Build and Start the Container:

from the root of the repository run

```bash
docker compose up -d
```

or

```bash
docker-compose up -d
```

This will build the Docker image and start the container.
# Database migrations
To apply migariton defined in your migration folder run

#### make sure you have downloaded sqlx-cli

```bash
cargo sqlx migrate run
```

# API Documentation

For detailed API documentation, check out the [API documentation](./API.md).
