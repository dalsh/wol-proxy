FROM rust:alpine as builder
WORKDIR /usr/src/myapp
COPY . .
RUN apk add --no-cache libc-dev
RUN cargo build -r && cargo install --path .
RUN chmod +x /usr/local/cargo/bin/wol-proxy

FROM scratch
WORKDIR /
COPY --from=builder /usr/local/cargo/bin/wol-proxy /
ENTRYPOINT ["/wol-proxy"]
