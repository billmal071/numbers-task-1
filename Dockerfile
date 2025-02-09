FROM rust:latest as builder

# update rust (debian?)
RUN rustup update

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

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/learn_async_rust"]
