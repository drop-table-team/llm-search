FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl libssl-dev

COPY --from=builder /app/target/release/llm-search .

ENV ADDRESS=0.0.0.0:8080

EXPOSE 8080

CMD ["./llm-search"]