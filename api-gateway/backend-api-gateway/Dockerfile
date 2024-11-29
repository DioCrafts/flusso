# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy file
RUN rm -f src/main.rs

# Copy the actual source code
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/api-gateway /app/api-gateway

# Create non-root user
RUN useradd -m -U -s /bin/false api-gateway && \
    chown -R api-gateway:api-gateway /app

USER api-gateway

# Expose port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV RUN_MODE=production

# Run the application
CMD ["./api-gateway"]