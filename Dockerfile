FROM rust:1.68-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev libressl-dev

COPY . .
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse RUSTFLAGS="" cargo build --release


FROM alpine:latest

VOLUME /opt/dynv6/data

WORKDIR /opt/dynv6/data

COPY --from=builder /app/target/release/dynv6 /

CMD [ "/dynv6" ]