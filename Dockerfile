FROM rust:alpine

RUN apk add --no-cache musl-dev
RUN mkdir /build
RUN mkdir /output

WORKDIR /build
COPY ./src ./src
COPY ./tests ./tests
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY rust-toolchain ./

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl
CMD cp -r /build/target /output
