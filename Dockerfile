# Build stage
FROM rust:1.91-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/server /app/server

# Copy certificate generation script (optional)
COPY generate_certs.sh ./
RUN chmod +x generate_certs.sh

# Create certs directory for volume mount
RUN mkdir -p /app/certs

# Create a non-root user with group for cert access
RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g appuser appuser && \
    chown -R appuser:appuser /app && \
    chmod 755 /app/certs
USER appuser

# Expose the application port (default 8080)
EXPOSE 8080

CMD ["/app/server"]
