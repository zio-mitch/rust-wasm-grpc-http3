## 🤖 AI Agent Instructions: Docker Naming Standards
Agents MUST strictly follow these naming conventions for all Docker-related resources. Any new service, volume, or environment variable must conform to the following patterns:

1. Container & Service Naming
Prefix: All containers and services MUST start with app_.

Pattern: app_[functional_role]

Standard Roles:

app_router: The Caddy/Gateway entry point.

app_api: The Rust/Tonic backend.

app_wasm_artifact_builder: The compilation worker.

Enforcement: container_name in docker-compose.yml MUST match the service key exactly.

2. Volume Naming (Scoping)
Volumes must be scoped to prevent collision and clarify ownership.

Pattern: app_[service_name]_[purpose]

Cache Volumes: Use the suffix _cargo_cache and _rust_target for Rust services.

Example: app_api_cargo_cache, app_wasm_artifact_builder_rust_target.

Data Exchange: Shared volumes for artifacts must use the _artifact suffix.

Example: app_wasm_artifact.

3. Network Naming
Pattern: app_network.

Internal DNS: Services must be addressed using their app_ prefixed name (e.g., http://app_api:PORT).

4. Environment Variables
System Variables: Use uppercase with underscores.

Service-Specific: Prefix with the service role if used for configuration.

Example: API_PORT, API_HOST.

Note: WEBUI_HOST, does not is a service, since is just where the ROUTER service respond serving the WEBUI (that is the deployed artifact).

Internal Path Overrides: Use standard Cargo/Rust env names where possible (e.g., CARGO_TARGET_DIR).

5. Local Path Mapping
Relative Mounts: Use ./[service_role] for source code binding.

Example: ./api for app_api, ./app for app_wasm_artifact_builder.

The Proto Hub: The shared protocol directory MUST always be named ./proto at the host root.