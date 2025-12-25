# Palette

Palette is a modern, full-stack web application example designed for seamless LLM interactions.

## Getting Started

### Prerequisites

- [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)

### Setup

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/KiraKiraAyu/palette
    cd palette
    ```

2.  **Generate JWT keys:**

    The application uses RSA keys for signing JWTs. You can generate them using OpenSSL.

    ```bash
    mkdir backend/keys
    openssl genpkey -algorithm RSA -out backend/keys/private_key.pem -pkeyopt rsa_keygen_bits:2048
    openssl rsa -pubout -in backend/keys/private_key.pem -out backend/keys/public_key.pem
    ```

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
