FROM rust:1.77 AS builder

WORKDIR /usr/src/app
COPY Cargo.toml .
COPY src/ src/
RUN rm -f Cargo.lock && cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/release/compression-tool /usr/local/bin/compressor

ENTRYPOINT ["compressor"] 