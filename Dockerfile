FROM rust:1.84.0-slim-bullseye AS build

# Install build dependencies
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    libssl-dev \
    libclang-dev \
    libudev-dev \
    pkg-config \
    wget \
    git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create dummy project to take advantage of Docker cache for dependencies
RUN USER=root cargo new --bin block-entries-service
WORKDIR /block-entries-service

# Copy manifests and fetch dependencies first (for caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source and build
COPY . .
RUN cargo build --release

# Final image
FROM rust:1.84.0-slim-bullseye

# Minimal runtime dependencies
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy binary from builder
COPY --from=build /block-entries-service/target/release/block-entries-service .

# Ensure it's executable
RUN chmod +x block-entries-service

# Logging level (optional)
ENV RUST_LOG=info

ENTRYPOINT ["./block-entries-service"]