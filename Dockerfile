FROM rust:1.69.0 AS builder

WORKDIR /usr/src/rust-forum-server
COPY . .
RUN cargo install --path .


FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/rust-forum-server /usr/local/bin/rust-forum-server
CMD ["rust-forum-server"]
