FROM ubuntu:24.04
LABEL authors="kongsgaard"

ARG WASM_PACK_VERSION="0.13.1"
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends curl ca-certificates && \
    ARCH="$(uname -m)" && \
    case "$ARCH" in \
      x86_64) WASM_ARCH="x86_64-unknown-linux-musl" ;; \
      aarch64|arm64) WASM_ARCH="aarch64-unknown-linux-musl" ;; \
      *) echo "Unsupported architecture: $ARCH" && exit 1 ;; \
    esac && \
    curl -sSL "https://github.com/rustwasm/wasm-pack/releases/download/v${WASM_PACK_VERSION}/wasm-pack-v${WASM_PACK_VERSION}-${WASM_ARCH}.tar.gz" | \
    tar -xz -C /usr/local/bin --strip-components 1 "wasm-pack-v${WASM_PACK_VERSION}-${WASM_ARCH}/wasm-pack" && \
    chmod +x /usr/local/bin/wasm-pack && \
    rm -rf /var/lib/apt/lists/*

USER ubuntu

# Set work directory
WORKDIR /workspaces
