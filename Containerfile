# syntax=docker/dockerfile:1

# ----------------------------------------
# Stage 1: richlsをビルドする
# ----------------------------------------
FROM rust:1.95.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ----------------------------------------
# Stage 2: 実行用イメージを作成する
# ----------------------------------------
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION
ARG LICENSE

LABEL org.opencontainers.image.title="richls" \
      org.opencontainers.image.description="An extended file listing tool written in Rust" \
      org.opencontainers.image.url="https://rin0323.github.io/richls/" \
      org.opencontainers.image.source="https://github.com/rin0323/richls" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${GIT_REVISION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.licenses="${LICENSE}" \
      org.opencontainers.image.authors="Yamaguchi Rin"

COPY --from=builder \
     --chown=65532:65532 \
     /app/target/release/richls \
     /app/richls

WORKDIR /work

USER 65532:65532

ENTRYPOINT ["/app/richls"]
