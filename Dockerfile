FROM rust:1.33-slim AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev
RUN mkdir /app
WORKDIR /app
ADD . .
RUN cargo test
RUN cargo build --release --verbose
EXPOSE 8000
ENTRYPOINT ["/app/target/release/image_uploader"]
