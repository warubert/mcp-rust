FROM rust:1.80-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust_mcp /usr/local/bin/rust_mcp
RUN touch /app/production-crash.log
CMD ["rust_mcp"]