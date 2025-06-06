# Use the official Rust image as base
FROM rust:1.75 as builder

# Install cargo-leptos
RUN cargo install cargo-leptos

# Install wasm-pack and trunk for WASM compilation
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install trunk

# Add the wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src
COPY style ./style

# Build the application
RUN cargo leptos build --release

# Use a smaller base image for the runtime
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/webpage /app/webpage

# Copy the site directory (contains static files and WASM)
COPY --from=builder /app/target/site /app/site

# Set environment variables
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./webpage"]
