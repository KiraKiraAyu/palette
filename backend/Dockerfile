FROM rust:1.92-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo new --bin backend
WORKDIR /app/backend

COPY Cargo.toml Cargo.lock ./
COPY migration ./migration

RUN cargo build --release

COPY src ./src

RUN touch src/main.rs

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/backend/target/release/palette /app/palette

COPY keys /app/keys

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./palette"]
