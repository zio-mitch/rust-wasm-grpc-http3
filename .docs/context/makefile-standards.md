# Makefile Automation Standards

This document defines the interface and naming conventions for the project's automation layer. Both human contributors and AI agents MUST adhere to these patterns when invoking or extending the Makefile.

## 🤖 AI Agent Instructions
- **Discovery**: Always run `make help` to discover available capabilities.
- **Execution**: Use `make all` for initial environment setup.
- **Validation**: Before suggesting a commit, ensure `make proto-inspect` passes if protocol definitions were modified.
- **Context**: Every target is self-documented. Read the `##` comments in the Makefile for runtime requirements.

---

## 🏷️ Naming Conventions

All targets follow a **Category-Action** (Noun-Action) pattern in `kebab-case`.

### 1. Service Lifecycle (`services-`)
Manages Docker Compose orchestration.
- `services-start`: Spin up the stack and verify networking.
- `services-stop`: Gracefully halt all containers.
- `services-reset`: Destructive rebuild (volumes, orphans, and no-cache build).

### 2. Tooling & Environment (`tools-`)
Handles local developer setup and IDE integration.
- `tools-setup-vscode`: Interactive generator for `settings.json`.

### 3. Observability & Debugging
- `*-debug`: Opens an interactive shell in a specific container (e.g., `api-debug`).
- `*-inspect`: Non-interactive read-only checks of the container filesystem (e.g., `proto-inspect`).
- `*-watch-logs`: Streams real-time logs from a specific background worker.

---

## 🛠️ Implementation Rules

### Atomic Chaining
Targets that perform sequential steps MUST use `&& \` to ensure the process halts on the first error.
*Example:* `docker down && docker up`

### Idempotency
Setup targets MUST check for existing artifacts before overwriting.
*Example:* `if [ -f .file ]; then echo "exists"; else generate; fi`

### Silent Output
Commands MUST be prefixed with `@` to keep the terminal output focused on the process results rather than the shell syntax.

### Documentation Requirements
Every functional target MUST include a trailing `##` comment on the same line to be indexed by the `make help` generator.