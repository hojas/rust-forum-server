# FROM rust:1.68.2-alpine AS builder
FROM rust:1.68.2-alpine

WORKDIR /usr/src/rust-forum-server
COPY . .
RUN cargo install --path .
CMD ["rust-forum-server"]

#FROM debian:bullseye-slim
#
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/local/cargo/bin/rust-forum-server /usr/local/bin/rust-forum-server
#CMD ["rust-forum-server"]
