FROM rust:1.72 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend_rust .
EXPOSE 5000
CMD ["./backend_rust"]
