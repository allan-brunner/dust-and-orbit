FROM rust:trixie AS wasm-builder
RUN cargo install wasm-pack
WORKDIR /app

COPY shared_logic ./shared_logic
WORKDIR /app/shared_logic

RUN wasm-pack build --target web --out-dir ./pkg

FROM node:20-alpine AS builder
WORKDIR /app

COPY --from=wasm-builder /app/shared_logic/pkg ./shared_logic/pkg

WORKDIR /app/client
COPY client/package*.json ./
RUN npm ci

COPY client ./
RUN npm run build

FROM caddy:2-alpine
WORKDIR /srv
COPY --from=builder /app/client/dist .
COPY client/Caddyfile /etc/caddy/Caddyfile