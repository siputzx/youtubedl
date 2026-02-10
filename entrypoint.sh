#!/bin/bash
set -e

export PATH="/home/appuser/.local/bin:$PATH"

if ! command -v yt-dlp &> /dev/null; then
    uv tool install yt-dlp --python python3
else
    uv tool upgrade yt-dlp
fi

YTDLP_VERSION=$(yt-dlp --version 2>/dev/null || echo "unknown")
echo "System Ready | yt-dlp version: $YTDLP_VERSION"

exec /app/youtube-downloader
