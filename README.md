# YouTube Downloader Server

[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://github.com/siputzx/youtubedl)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Modular, high-performance YouTube downloader dengan Proof of Work authentication. Dibangun dengan Rust untuk performa maksimal dan keamanan tinggi.

## 🌟 Kelebihan

### Performa Tinggi
- **Multi-threading**: Download concurrent dengan kontrol otomatis berdasarkan CPU cores
- **Memory efficient**: Menggunakan Rust dengan zero-cost abstractions
- **Async I/O**: Non-blocking operations untuk handling request maksimal
- **Smart caching**: File caching otomatis dengan cleanup scheduler
- **Minimalist logging**: Request-only logging dengan latency tracking

### Keamanan
- **Proof of Work (PoW)**: Anti-spam protection dengan SHA256 challenge
- **API Key Support**: Premium access tanpa PoW untuk user terverifikasi
- **Path traversal protection**: Security validation untuk file serving
- **Session management**: Secure cookie-based authentication
- **Rate limiting**: Built-in protection dari abuse

### Fitur Lengkap
- **Multiple formats**: Audio (MP3), Video (MP4), Merge (Video+Audio)
- **Quality selection**: Hierarki format dari 360p hingga 4K
- **Auto cookies detection**: Otomatis detect semua cookies*.txt di folder
- **Flexible cookies**: Support 1-unlimited cookie files, random selection
- **Proxy support**: HTTP/HTTPS/SOCKS5 proxy support
- **Auto yt-dlp update**: Selalu menggunakan yt-dlp versi terbaru saat container start
- **Metadata embedding**: Auto embed thumbnail dan metadata ke MP3
- **Progress tracking**: Real-time download progress monitoring
- **Smart retry**: Automatic fallback ke format alternatif

### Container-Ready
- **Alpine-based**: Image kecil dengan semua dependencies
- **Auto-update yt-dlp**: Install/update otomatis saat start
- **Health checks**: Auto-monitoring container health
- **Volume support**: Persistent storage untuk downloads dan cookies
- **Environment-based**: Full configuration via environment variables

## 🚀 Quick Start

### Pull dan Run dengan Docker

```bash
docker pull ghcr.io/siputzx/youtubedl:latest

# Basic run
docker run -d \
  --name youtube-downloader \
  -p 3000:3000 \
  ghcr.io/siputzx/youtubedl:latest

# Dengan cookies dan proxy
docker run -d \
  --name youtube-downloader \
  -p 3000:3000 \
  -e PROXY=http://proxy.example.com:8080 \
  -v ./cookies:/app/cookies \
  ghcr.io/siputzx/youtubedl:latest
```

### Pull dan Run dengan Podman

```bash
podman pull ghcr.io/siputzx/youtubedl:latest

podman run -d \
  --name youtube-downloader \
  -p 3000:3000 \
  -v ./cookies:/app/cookies:Z \
  ghcr.io/siputzx/youtubedl:latest
```

## ⚙️ Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Port server HTTP |
| `MAX_CONCURRENT` | `auto` | Maksimal concurrent downloads |
| `CACHE_DURATION` | `18000000` | Cache duration (ms) |
| `MAX_VIDEO_DURATION` | `10800` | Max video duration (seconds) |
| `MAX_AUDIO_DURATION` | `18000` | Max audio duration (seconds) |
| `MAX_FILE_SIZE` | `1073741824` | Max file size (bytes) |
| `POW_DIFFICULTY` | `1` | PoW difficulty (1-5) |
| `USE_COOKIES` | `true` | Enable cookies (true/false) |
| `PROXY` | `` | Proxy URL (http/https/socks5) |
| `VALID_APIKEYS` | `siputzxteam,cepetan` | Comma-separated API keys |

## 🍪 Cookie Management

### Auto Detection
Otomatis detect **semua** file `cookies*.txt`:

```
/app/cookies/
├── cookies1.txt    ✓
├── cookies2.txt    ✓
├── cookies_yt.txt  ✓
└── other.txt       ✗
```

### Example
```bash
mkdir -p ./cookies
cp cookies1.txt ./cookies/
cp cookies2.txt ./cookies/

docker run -d -p 3000:3000 \
  -v ./cookies:/app/cookies \
  ghcr.io/siputzx/youtubedl:latest
```

Output: `Found 2 cookie file(s)`

## 🌐 Proxy Support

```bash
# HTTP Proxy
docker run -d -p 3000:3000 \
  -e PROXY=http://proxy.example.com:8080 \
  ghcr.io/siputzx/youtubedl:latest

# SOCKS5 dengan auth
docker run -d -p 3000:3000 \
  -e PROXY=socks5://user:pass@proxy:1080 \
  ghcr.io/siputzx/youtubedl:latest
```

## 📊 Logging

Minimalist request logging:
```
YouTube Downloader v2.0.0 - Listening on 0.0.0.0:3000
[14:23:45] POST /akumaudownload - 200 (12ms)
[14:23:48] GET /download - 202 (15ms)
[14:25:12] GET /files/abc.mp3 - 200 (234ms)
```

Format: `[HH:MM:SS] METHOD PATH - STATUS (latency_ms)`

## 🔄 Auto Update YT-DLP

**Tidak perlu rebuild image** untuk update yt-dlp:

```bash
# Force update
docker restart youtube-downloader
```

Container otomatis install/update yt-dlp saat start (~5-10 detik).

## 🔌 API Endpoints

### GET `/`
Server status
```json
{
  "status": "ok",
  "system": {
    "uptime": "2h 15m",
    "active_cookies": "3 files"
  }
}
```

### POST `/akumaudownload`
Get PoW challenge

### POST `/cekpunyaku`
Verify PoW solution

### GET `/download?url=VIDEO_ID&type=audio&apikey=KEY`
Download video/audio

### GET `/files/{filename}`
Serve downloaded file

## 📝 Contoh Production

```bash
docker run -d \
  --name ytdl-prod \
  -p 8080:8080 \
  -e PORT=8080 \
  -e POW_DIFFICULTY=3 \
  -e PROXY=socks5://proxy:1080 \
  -e VALID_APIKEYS=key1,key2,key3 \
  -v /data/downloads:/app/downloads \
  -v /data/cookies:/app/cookies \
  --restart unless-stopped \
  --cpus=4 \
  --memory=2g \
  ghcr.io/siputzx/youtubedl:latest
```

## 🛠️ Troubleshooting

```bash
# Check logs
docker logs youtube-downloader

# Check yt-dlp
docker exec youtube-downloader yt-dlp --version

# Check cookies
docker exec youtube-downloader ls -la /app/cookies/

# Test proxy
docker exec youtube-downloader wget -e use_proxy=yes google.com
```

## 📄 License

MIT License

## 🙏 Credits

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Auto-updated
- [actix-web](https://actix.rs/) - Web framework
- [uv](https://github.com/astral-sh/uv) - Package installer

---

**Made with ❤️ using Rust & Alpine Linux**
