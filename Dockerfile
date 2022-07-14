FROM alpine:latest

WORKDIR /app

COPY ./target/x86_64-unknown-linux-musl/release/dwd .

VOLUME ["/app/config.yaml"]

ENTRYPOINT ["./dwd", "-c", "config.yaml"]
