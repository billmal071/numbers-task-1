FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/learn_async_rust /usr/local/bin/
CMD ["learn_async_rust"]
