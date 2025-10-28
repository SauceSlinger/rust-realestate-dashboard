# Backend Dockerfile
FROM rust:1.75 as backend-builder

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src

RUN cargo build --release

# Frontend Dockerfile
FROM node:20 as frontend-builder

WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Final image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    sqlite3 \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/backend /app/backend

# Copy frontend build
COPY --from=frontend-builder /app/dist /app/frontend/dist

# Create data directory for SQLite database
RUN mkdir -p /app/data

ENV DATABASE_PATH=/app/data/realestate.db

EXPOSE 3000

CMD ["/app/backend"]
