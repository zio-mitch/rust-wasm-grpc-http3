# 🚀 Rust Full-Stack: gRPC-Web, WASM & HTTP/3
This project is a "Single Model" reference architecture for high-performance modern web applications. It leverages Rust on both ends (Frontend and Backend) to ensure memory safety, native speed, and a shared end-to-end type system.

## 🖼️ Frontend UI Model (WASM + egui)
The frontend UI is built with `egui`/`eframe` and runs in the browser as WebAssembly.

- The UI is rendered on a `canvas` via WASM (immediate-mode GUI).
- `index.html` is only the host page/bootstrapping shell.
- API communication remains gRPC-Web with generated protobuf types.

## 🌐 Modern Protocols & Networking
The project is designed to operate on the most advanced protocols available today, with an architecture ready for the next evolution of the Web.

### HTTP/3 and QUIC
The infrastructure uses Caddy as the Edge Router to natively support [HTTP/3](https://httpwg.org/specs/rfc9114.html), the latest iteration of the HTTP protocol based on QUIC (Quick UDP Internet Connections).

Advantage: Drastic reduction in latency by eliminating Head-of-Line Blocking (typical of TCP) and supporting 0-RTT handshakes via UDP.

Implementation: External traffic is handled via UDP/443 [QUIC](https://quicwg.org/). Internal communication to the Rust container occurs via h2c (HTTP/2 Cleartext) for maximum stability within the isolated Docker network.

Current Constraints: WASM and HTTP/2
It is important to note that currently, WebAssembly (WASM) in the browser is constrained by standard Web APIs (fetch/XHR).

Under the hood: The browser currently forces [gRPC-Web](https://github.com/grpc/grpc-web) calls over HTTP/2.

The Future WebTransport: Native support for gRPC directly over UDP/QUIC (without bridges or translation layers) will be possible as the [WebTransport API](https://www.w3.org/groups/wg/webtransport) standard matures.

Roadmap: Full and stable integration for complex bidirectional binary protocols in WASM is expected to become the industry standard between 2026 and 2027, as WebTransport APIs eventually replace WebSockets and gRPC-Web bridges.

See also the [Project Roadmap](ROADMAP.md) for a detailed migration path and technical debt tracker related to the transition to native QUIC/WebTransport.

## 🏗️ "Single Model" Architecture & Hot Reload
The project follows the Single Source of Truth philosophy:

Protocol Buffers (.proto): A single schema defines the data. Any change to the .proto file automatically generates updated code for both the Client (WASM) and the Server (Tonic).

Docker Hot Reload: Source code is mounted as a volume into the local development containers.

The Backend (app_api) and the WASM Builder (app_artifact_builder) monitor file changes and recompile instantly upon saving .rs files.

## 🛠️ Getting Started
Prerequisites
Docker & Docker Compose.

A modern browser (Chrome/Brave recommended for local HTTP/3 support).

Setup Procedure
Launch Containers:

Bash
docker compose up --build

### Troubleshooting
> [!IMPORTANT] Domains resolution:
> **Domain Resolution:** If `make start-services` shows ❌ for the `.localhost` domains, your OS may not be auto-resolving them. 
> You might need to add the following to your `/etc/hosts` file:
> `127.0.0.1 app.localhost api.localhost`

> [!IMPORTANT] Caddy certificates
> Caddy generates local certificates for .localhost domains. Browsers will block cross-domain calls unless manually authorized.
> Open https://app.localhost and accept the certificate, without this step cannot access the app frontend in the browser.
> Open https://api.localhost and accept the certificate, without this step, gRPC calls will fail with a generic Network Error.

Configuration Changes
Rust Code: Reloading is automatic thanks to file monitoring within the containers.

Caddyfile: If you modify the router configuration (e.g., changing a domain, port, or filter), Caddy does not automatically detect the change for security reasons. You must manually restart the container:

Bash
docker compose restart app_router

## 📂 Project Structure
/proto: gRPC definitions (The single data model).

/wasm_artifact: egui (Rust) frontend compiled to WASM (canvas-based rendering).

/api: Tonic (Rust) backend with native gRPC-Web layer support.

Technological stack optimized for performance, scalability, and end-to-end type safety.