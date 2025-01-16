# Build stage
FROM rust:1.78 as builder
WORKDIR /app
COPY . /app/sui-token-gen
WORKDIR /app/sui-token-gen
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/sui-token-gen/target/release/sui-token-gen /app/sui-token-gen

# Default to bash shell for interactive access
CMD ["bash"]
