FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev libressl-dev
WORKDIR /app

COPY server ./server
COPY shared_logic ./shared_logic

WORKDIR /app/server
RUN cargo build --release

FROM alpine:3.19
WORKDIR /app
COPY --from=builder /app/server/target/release/server ./server
EXPOSE 8080
CMD ["./server"]