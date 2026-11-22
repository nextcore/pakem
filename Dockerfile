# Multi-stage build for Pakem Framework Application
FROM rust:1.80-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build release binaries
RUN cargo build --release &&     (cp /usr/src/app/target/release/pakem-server /app-server 2>/dev/null || cp /usr/src/app/target/release/canting /app-server 2>/dev/null || true)

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app-server /app/pakem-server
COPY .env.example /app/.env
COPY resources /app/resources
COPY database /app/database

EXPOSE 8000
CMD ["./pakem-server"]
