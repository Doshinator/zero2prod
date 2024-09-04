FROM rust:1.80.1 as builder
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]