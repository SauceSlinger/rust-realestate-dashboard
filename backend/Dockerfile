# Backend Dockerfile
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/realestate-backend /app/
COPY --from=builder /usr/src/app/migrations /app/migrations

# Create data directory
RUN mkdir -p /app/data

EXPOSE 3000

ENV DATABASE_URL=sqlite:/app/data/realestate.db
ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["./realestate-backend"]
