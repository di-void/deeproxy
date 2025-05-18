# Deeproxy

A simple HTTP proxy server with caching capabilities written in Rust. My solution to [this project](https://roadmap.sh/projects/caching-server) on roadmaps.sh.

## Features

- HTTP proxy server
- File-based response caching
- Cache control through CLI
- Support for custom origin servers
- Cache hit/miss headers

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/di-void/deeproxy.git
cd deeproxy
cargo build --release
```

## Usage

### Starting the Proxy Server

To start the proxy server, use the following command:

```bash
cargo run --release -- --port <PORT> --origin <ORIGIN_URL>
```

Example:

```bash
cargo run --release -- --port 3000 --origin http://example.com
```

This will start the proxy server on `http://localhost:3000` forwarding requests to `http://example.com`.

### Cache Management

To clear the cache:

```bash
cargo run clear-cache
```

## How it Works

1. When a request is made to the proxy server, it first checks if the response is cached
2. If cached (HIT):
   - Returns the cached response with `X-Cache: HIT` header
3. If not cached (MISS):
   - Forwards the request to the origin server
   - Caches the response
   - Returns the response with `X-Cache: MISS` header

## Dependencies

- `hyper` - HTTP server/client
- `tokio` - Async runtime
- `clap` - Command line argument parsing
- `serde` - Serialization/deserialization
- `sha2` - URL hashing for cache keys
- And more (see `Cargo.toml`)

## Development

Build the project:

```bash
cargo build
```

## Limitations

- Only supports HTTP (not HTTPS) origins
- Basic caching implementation without cache invalidation
- No cache size limits

## Nice-to-haves

- Check and respect cache-control headers
- Cached data compression
- Size or memory limits for cached responses
