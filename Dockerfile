FROM rust:1-slim AS builder

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get -y update && apt-get install -y pkg-config libpq-dev libssl-dev

RUN cargo install cargo-build-deps

WORKDIR /app

RUN cargo new --bin youtube
WORKDIR /app/youtube

COPY Cargo.toml ./
RUN cargo build-deps --release

COPY src ./src
RUN cargo build --release
RUN strip target/release/youtube

FROM debian:stable-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get -y update && apt-get install -y libpq5 tini

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

WORKDIR /app

COPY --from=builder /app/youtube/target/release/youtube /app/youtube

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/youtube"]
