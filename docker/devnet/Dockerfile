# Multi-stage build for ORIUM blockchain node
FROM rust:1.75-bullseye as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /orium

# Copy source code
COPY . .

# Build the node (release mode for performance)
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create user for running the node
RUN useradd -m -u 1000 -U -s /bin/sh -d /orium orium

# Copy the binary from builder stage
COPY --from=builder /orium/target/release/orium-node /usr/local/bin/orium-node

# Create data directory
RUN mkdir -p /data && chown -R orium:orium /data

# Switch to non-root user
USER orium

# Expose ports
EXPOSE 9944 9933 30333 9615

# Set data directory as volume
VOLUME ["/data"]

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:9933/health || exit 1

# Default command
ENTRYPOINT ["/usr/local/bin/orium-node"]
