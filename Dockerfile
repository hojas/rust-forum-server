FROM rust:1.69.0-alpine AS builder

WORKDIR /usr/src/rust-forum-server
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-forum-server /usr/local/bin/rust-forum-server
CMD ["rust-forum-server"]
