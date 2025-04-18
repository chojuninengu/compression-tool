FROM rust:1.77 as builder

WORKDIR /usr/src/app
COPY Cargo.toml .
COPY src/ src/
RUN rm -f Cargo.lock && cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/release/rust-compressor /usr/local/bin/compressor

ENTRYPOINT ["compressor"] 