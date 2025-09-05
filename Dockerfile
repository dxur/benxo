# --- Stage 1: Build backend and generate bindings ---
FROM rust:1.88-slim AS backend-builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY ./Cargo.toml ./Cargo.toml
COPY ./macros ./macros
COPY ./backend ./backend

RUN --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=backend-target,target=/app/target \
    cargo build --release -p backend && \
    cargo test export_bindings --release -p backend && \
    cp /app/target/release/backend /bin/backend

# --- Stage 2: Backend runtime image ---
FROM debian:bookworm-slim AS backend
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /bin/backend /bin/backend
CMD ["/bin/backend"]

# --- Stage 3: Frontend builder ---
FROM oven/bun:latest AS frontend-builder
WORKDIR /app

COPY --from=backend-builder /app/backend/bindings ./backend/bindings
COPY ./frontend ./frontend

RUN --mount=type=cache,target=/app/frontend/node_modules cd frontend && bun install
RUN cd frontend && bun run build

FROM caddy AS proxy
WORKDIR /srv
COPY --from=frontend-builder /app/frontend/dist ./www
