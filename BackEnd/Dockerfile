# Use official Rust image
FROM rust:1 AS builder

# Create app directory
WORKDIR /backend

# Copy source code
COPY . .

# Build the backend in release mode
RUN cargo build --release

# Final minimal image
FROM debian:bookworm-slim

# Install openssl runtime for Rust
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder stage
COPY --from=builder /backend/target/release/rust-backend /usr/local/bin/rust-backend

# Expose backend port
EXPOSE 3001

# Run
ENTRYPOINT ["rust-backend"]
