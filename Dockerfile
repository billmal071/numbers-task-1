FROM rust:latest as builder

RUN rustup update

RUN apt-get update && \
    apt-get install -y musl-tools && \  
    rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src                       \
    && touch src/lib.rs
RUN cargo build --release

RUN rm -rf src
COPY src ./src
RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /target/release/learn_async_rust /usr/local/bin/learn_async_rust

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/learn_async_rust"]
