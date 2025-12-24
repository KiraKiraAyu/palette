# Palette

Palette is a full-stack web application.

## Project Structure

- `backend`: The Rust backend server.
- `frontend`: The Vue.js frontend application.
- `database`: Docker configuration for the database.
- `proxy`: Proxy configuration.

## Getting Started

### Setup

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/KiraKiraAyu/palette
    cd palette
    ```

2.  **Set up environment variables:**

    Copy `.env.example` file to `.env` in the root of the project and add the environment variables.

    ```env
    # Server Configuration
    HOST=127.0.0.1
    PORT=3000

    # Database URL (adjust with your database credentials)
    DATABASE_URL=postgres://user:password@localhost:5432/database_name

    # JWT Settings
    JWT_PRIVATE_KEY_PATH=backend/keys/private_key.pem
    JWT_PUBLIC_KEY_PATH=backend/keys/public_key.pem
    JWT_EXPIRES_IN=86400
    ```

3.  **Generate JWT keys:**

    The application uses RSA keys for signing JWTs. You can generate them using OpenSSL.

    ```bash
    mkdir backend/keys
    openssl genpkey -algorithm RSA -out backend/keys/private_key.pem -pkeyopt rsa_keygen_bits:2048
    openssl rsa -pubout -in backend/keys/private_key.pem -out backend/keys/public_key.pem
    ```
