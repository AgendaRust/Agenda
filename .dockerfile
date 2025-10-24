# --- Estágio 1: Builder ---
# Usamos uma imagem Rust mais recente (1.82) que suporta a edition 2024
FROM rust:1.90-slim-bullseye AS builder

# Instala dependências necessárias para compilar (especificamente para sqlx-sqlite)
RUN apt-get update && apt-get install -y \
    sqlite3 \
    libsqlite3-dev \
    pkg-config \
    libssl-dev \
    openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Instala o sea-orm-cli com a versão exata do Cargo.toml
# Esta é a correção principal para o erro 'edition2024'
RUN cargo install sea-orm-cli

# --- Otimização de Cache do Cargo ---
# Adicionamos 'backend/' ao caminho
COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
RUN mkdir -p backend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "pub fn dummy() {}" > backend/src/lib.rs
RUN cd backend && cargo build --release
RUN rm -f backend/target/release/deps/backend*

# Copia o código-fonte real do backend
# Adicionamos 'backend/' ao caminho
COPY backend/src ./backend/src
COPY backend/migration ./backend/migration

# Constrói o binário de release
# (Já não precisamos da linha "COPY main.rs" porque ele já está em backend/src)
RUN cd backend && cargo build --release

# Constrói o binário de migração
RUN cd backend/migration && cargo build --release

# --- Estágio 2: Final ---
# Usamos uma imagem slim do Debian para a imagem final
FROM debian:bullseye-slim AS final

WORKDIR /app

# Instala apenas o sqlite3
RUN apt-get update && apt-get install -y \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Copia os artefatos do 'builder'
COPY --from=builder /app/backend/target/release/backend .
COPY --from=builder /app/backend/migration/target/release/migration ./migration-cli
COPY --from=builder /app/backend/migration ./migration

# Copia a pasta 'dist' do frontend
# Adicionamos 'frontend/' ao caminho
COPY frontend/dist ./dist

# Cria um diretório para o volume do banco de dados com permissões corretas
RUN mkdir -p /app/data && chmod 777 /app/data

# Define o DATABASE_URL padrão (será sobrescrito pelo compose)
ENV DATABASE_URL=sqlite:/app/data/database.db
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000

# Define o comando padrão para iniciar o servidor
CMD ["./backend"]

