# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/

# Create dummy files to build dependencies
RUN mkdir -p backend/src shared/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "pub fn placeholder() {}" > shared/src/lib.rs

# Build dependencies
RUN cargo build --release --manifest-path backend/Cargo.toml

# Remove dummy files
RUN rm -rf backend/src shared/src

# Copy source code
COPY shared/src ./shared/src
COPY backend/src ./backend/src

# Build application
RUN cargo build --release --manifest-path backend/Cargo.toml

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/liecord-backend /app/liecord-backend

# Create uploads directory
RUN mkdir -p /app/uploads

EXPOSE 8080

CMD ["/app/liecord-backend"]
