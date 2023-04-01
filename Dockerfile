FROM rust:1.68-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev upx

COPY . .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo build --release --no-default-features --features rustls
RUN upx --ultra-brute target/release/dynv6

FROM alpine:latest

VOLUME /opt/dynv6/data

WORKDIR /opt/dynv6/data

COPY --from=builder /app/target/release/dynv6 /

CMD [ "/dynv6" ]