FROM rust:1.88-slim as builder

RUN USER=root cargo new --bin app
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/app /app/app
CMD ["./app"]
