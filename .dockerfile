FROM rust:1.90-slim-bullseye AS builder

RUN apt-get update && apt-get install -y \
    sqlite3 \
    libsqlite3-dev \
    pkg-config \
    libssl-dev \
    openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN cargo install sea-orm-cli

COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "pub fn dummy() {}" > backend/src/lib.rs
RUN cd backend && cargo build --release
RUN rm -f backend/target/release/deps/backend*

COPY backend/src ./backend/src
COPY backend/migration ./backend/migration

RUN cd backend && cargo build --release

RUN cd backend/migration && cargo build --release

FROM debian:bullseye-slim AS final

WORKDIR /app

RUN apt-get update && apt-get install -y \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend/target/release/backend .
COPY --from=builder /app/backend/migration/target/release/migration ./migration-cli
COPY --from=builder /app/backend/migration ./migration

COPY frontend/dist ./dist

ENV DATABASE_URL=sqlite:/app/data/database.db
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

CMD ["./backend"]

