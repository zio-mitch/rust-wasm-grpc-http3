# 🗺️ Project Roadmap: Transition to Native QUIC & WebTransport
Currently, this project uses a gRPC-Web bridge because browsers and WebAssembly (WASM) do not yet support native gRPC over QUIC streams. This document outlines the technical debt and the migration path for when WebTransport and QUIC become fully accessible via WASM.

## 🎯 Phase 1: Current Architecture (gRPC-Web over HTTP/2)
The current setup requires a translation layer to handle browser limitations.

## 🛑 Infrastructure (Caddyfile)
Current configuration requires manual CORS and header management:

```caddy
### CURRENT STATE: gRPC-Web Bridge
api.localhost {
    header {
        Access-Control-Allow-Origin https://app.localhost
        Access-Control-Allow-Methods "POST, OPTIONS"
        Access-Control-Allow-Headers "keep-alive,user-agent,cache-control,content-type,content-transfer-encoding,x-accept-content-transfer-encoding,x-accept-response-streaming,x-user-agent,x-grpc-web,grpc-timeout"
        Access-Control-Expose-Headers "grpc-status,grpc-message"
    }

    @options method OPTIONS
    respond @options 204

    reverse_proxy app_api:50051 {
        transport http {
            versions h2c 2
        }
    }
}
```

## 🛑 Backend (Rust/Tonic)
Current server must accept HTTP/1.1 and use a specific compatibility layer:

```rust
// CURRENT STATE: Tonic-Web Compatibility
Server::builder()
    .accept_http1(true) // Required for gRPC-Web
    .layer(GrpcWebLayer::new()) // Necessary bridge layer
    .add_service(service)
    .serve(addr)
    .await?;
```
## 🚀 Phase 2: Future Migration (Native QUIC/WebTransport)
Timeline: Expected 2026-2027 (Following W3C WebTransport Stability).

### ✅ 1. Remove gRPC-Web Dependencies
Action: Uninstall tonic-web crate.

Why: Native QUIC streams remove the need for the "Web" translation of gRPC.

Backend Change:

```rust
Rust
// FUTURE STATE: Pure gRPC over QUIC/UDP
Server::builder()
    // .accept_http1(true) -> DELETE: No longer needed
    // .layer(GrpcWebLayer::new()) -> DELETE: No longer needed
    .add_service(service)
    .serve_with_incoming(quic_incoming) // Use UDP/QUIC listener
    .await?;
```

### ✅ 2. Infrastructure Simplification
Action: Simplify Caddyfile.

Why: Caddy will act as a pure L7 QUIC proxy. Browsers using WebTransport handle security headers differently, reducing manual CORS boilerplate.

Caddyfile Change:

```caddy
# FUTURE STATE: Pure QUIC Proxy
api.localhost {
    # No more header hacks or gRPC-Web specific blocks
    reverse_proxy app_api:50051
}
```

### ✅ 3. Frontend WASM Upgrade
Action: Replace the current grpc-web client with a WebTransport Connector.

Reference: Follow the W3C WebTransport WG specifications.

## 🛠️ Technical Debt Tracker
[ ] Dependency: Monitor wtransport or quinn for WASM-compatible QUIC implementations.

[ ] Standard: Wait for HTTP/3 RFC 9114 to be fully exploitable via WASM WebTransport API.

[ ] Cleanup: Mark tonic-web as deprecated once the UDP listener is stable.

## 🔗 Reference Working Groups
QUIC Protocol: quicwg.org

HTTP/3 Spec: IETF HTTPWG

WASM Networking: W3C WASM WG