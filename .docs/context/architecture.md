# Single Model Architecture (gRPC + WASM)

This project uses a **Single Source of Truth** for data models. All communication between the Backend (Rust/Tonic) and the Frontend (Rust/Sycamore) is governed by shared Protocol Buffers.

## 📁 The `proto/` Directory
The root `/proto` directory contains all `.proto` definitions. 

### 1. Mounting Logic (Docker)
To ensure consistency without duplicating files, the directory is bind-mounted into multiple containers:
- **API Container (`app_api`)**: Mounted at `/usr/src/app/proto`. Used by `tonic-build` to generate Server traits.
- **WASM Builder (`app_artifact_builder`)**: Mounted at `/usr/src/app/proto`. Used to generate the gRPC-web client for the browser.

### 2. Synchronization Workflow
When a `.proto` file is modified:
1. The **WASM Builder** detects the change (via `cargo watch`) and regenerates the frontend artifact.
2. The **API Container** recompiles the server logic upon the next restart or trigger.
3. Use `make proto-inspect` to verify that both environments see the same file versions.

## 🤖 AI Agent Context
- **Code Generation**: Do not manually write Rust structs for data models. Always modify the `.proto` files and let the build system generate the types.
- **Pathing**: Inside containers, always refer to the mounted `/proto` path, not the host root path.