FROM arm64v8/alpine:latest
COPY ./target/release/main /app/main

RUN apk install ca-certificates

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
CMD ["/app/main"]
