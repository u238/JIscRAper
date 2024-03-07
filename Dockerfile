FROM rust:1.71.1 as builder
WORKDIR /usr/src/jiscraper
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/JIscRAper /usr/local/bin/JIscRAper
ENTRYPOINT [ "JIscRAper" ]