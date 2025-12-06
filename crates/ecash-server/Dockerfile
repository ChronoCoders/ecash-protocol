FROM rust:1.91-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/ecash-core ./crates/ecash-core
COPY crates/ecash-server ./crates/ecash-server

RUN cargo build --release -p ecash-server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/ecash-server /usr/local/bin/ecash-server

RUN useradd -m -u 1000 ecash && \
    chown -R ecash:ecash /app

USER ecash

EXPOSE 8080

CMD ["ecash-server"]
