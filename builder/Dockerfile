# Use the latest Ubuntu image as the base
FROM ubuntu:latest AS builder

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev pkg-config ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env"

# Add Cargo to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install Git and configure it to use OpenSSL backend
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
RUN git config --global http.sslBackend "openssl"

# Build the release binary
RUN cargo build --release

# Create a new stage for the final image
FROM ubuntu:latest

# Install Redis and CA certificates
RUN apt-get update && \
    apt-get install -y redis-server ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the release binary from the builder stage
COPY --from=builder /app/target/release/builder /usr/local/bin/builder

# Expose the port your application runs on (replace 8080 with your port)
EXPOSE 8080

# Start Redis in the background
CMD service redis-server start && builder