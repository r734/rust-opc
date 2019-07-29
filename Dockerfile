FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM bitnami/minideb:buster
WORKDIR /
COPY --from=builder /build/target/release/rust-opc .

EXPOSE 80

ENTRYPOINT ["./rust-opc"]