# Multi-stage build for minimal final image
FROM rust:1.75-slim as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Final stage - minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 agi-aef

WORKDIR /home/agi-aef

# Copy binary from builder
COPY --from=builder /app/target/release/agi-aef /usr/local/bin/agi-aef

# Change ownership
RUN chown -R agi-aef:agi-aef /home/agi-aef

# Switch to non-root user
USER agi-aef

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command - start API server
CMD ["agi-aef", "serve", "--host", "0.0.0.0", "--port", "8080"]
