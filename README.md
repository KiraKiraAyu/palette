# Palette

Palette is a modern, full-stack web application example designed for seamless LLM interactions.

## Getting Started

### Prerequisites

- [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)

### Running the App

The easiest way to run Palette is using Docker Compose:

```bash
docker compose up -d
```

This will start: `http://localhost:80`

## Local Development

### Configuration

```bash
cp .env.example .env
```

### Backend

Navigate to the `backend` directory:

```bash
cd backend
cargo run
```

### Frontend

Navigate to the `frontend` directory:

```bash
cd frontend
pnpm install
pnpm dev
```
