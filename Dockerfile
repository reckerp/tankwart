# Build stage
FROM rust:slim-trixie AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo clean && cargo build --release

# Runtime stage
FROM debian:trixie-slim

# Install CA certificates for HTTPS requests
RUN apt-get update && \
  apt-get install -y ca-certificates && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/tankwart /app/tankwart

# Create a non-root user
RUN useradd -m -u 1000 tankwart && \
  chown -R tankwart:tankwart /app

USER tankwart

CMD ["/app/tankwart"]
