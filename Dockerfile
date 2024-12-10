# Step 1: Use the official Rust image for building
FROM rust:latest AS builder

WORKDIR /app

# Install musl-tools
RUN apt-get update && apt-get install -y musl-tools

# Copy the Cargo.toml to cache dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl --bin polars_rust_tut

# Copy the rest of the source code
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Use a minimal runtime image
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/polars_rust_tut .

EXPOSE 8080
CMD ["./polars_rust_tut"]
