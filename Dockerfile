# Build stage
FROM rust:1.78 as builder
WORKDIR /app
COPY . /app/api
WORKDIR /app/api
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/api/target/release/sui-token-gen /app/api

# Default to bash shell for interactive access
CMD ["bash"]
