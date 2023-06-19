FROM rust:1.70 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

#

FROM arm64v8/debian:bullseye-slim
COPY --from=builder /build/target/release/main /app/main

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
CMD ["/app/main"]
