# Use the official Rust image as the base image
FROM rust:1.75-slim AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml first for better caching
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-docker-app /usr/local/bin/rust-docker-app

# Expose port
EXPOSE 3000

# Run the application
CMD ["rust-docker-app"]