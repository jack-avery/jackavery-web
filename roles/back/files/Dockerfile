FROM arm64v8/debian:bullseye-slim
COPY ./target/release/main /app/main

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install ca-certificates -y

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
CMD ["/app/main"]
