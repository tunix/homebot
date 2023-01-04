FROM docker.io/rust:1.66.0 AS builder

WORKDIR /usr/src/homebot
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

ENV RUST_LOG=info

RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/home /usr/local/bin/home

ENTRYPOINT ["home"]
