FROM rust:1.67-alpine AS builder

WORKDIR /app
COPY . .
RUN cargo build --release


FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/mini-poker .
CMD ["./mini-poker"]
