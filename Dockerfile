FROM docker.io/library/rust:1.93-alpine3.21 AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make

WORKDIR /build

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM docker.io/library/alpine:3.21

RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    ffmpeg \
    deno \
    python3 \
    py3-pip \
    curl \
    bash \
    libgcc

COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/bin/uv

RUN addgroup -S appgroup && adduser -S appuser -G appgroup && \
    mkdir -p /home/appuser/.local/bin /app/downloads/audio /app/downloads/video /app/downloads/merge /app/cookies && \
    chown -R appuser:appgroup /home/appuser /app

WORKDIR /app

COPY --from=builder /build/target/release/youtube-downloader /app/youtube-downloader
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/youtube-downloader /app/entrypoint.sh

USER appuser

ENV PORT=3000 \
    PATH="/home/appuser/.local/bin:/usr/bin:/app:${PATH}"

EXPOSE 3000

ENTRYPOINT ["/app/entrypoint.sh"]
