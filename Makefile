.PHONY: help start-services stop-services reset-services proto-inspect router-debug router-inspect-frontend-artifact api-debug artifact-builder-rebuild artifact-builder-watch-logs artifact-builder-inspect-artifact

.DEFAULT_GOAL := help

## Help
help:
	@echo "--- 🛠️ gRPC-Rust Project Management ---"
	@echo "Usage: make [target]"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: tools-setup-vscode start-services

## Start all containers
start-services: ensure-local-hosts
	@echo "--- Starting all services ---"
	docker compose up -d

## Stop all containers
stop-services:
	@echo "--- Stopping all services ---"
	docker compose down

## Reset services: deep clean of containers, volumes, orphans and force a fresh build
reset-services:
	@echo "--- Resetting all services (Deep Clean, consumes bandwidth and is slow!) ---"
	docker compose down -v --remove-orphans && \
	docker compose build --no-cache && \
	docker compose up -d --force-recreate

## Check if host can access the domains used by services (e.g., app.localhost, api.localhost)
ensure-local-hosts:
	@echo "--- Checking host access to service domains ---"
	@ping -c 1 app.localhost > /dev/null && echo "✅ app.localhost is reachable" || echo "❌ app.localhost is NOT reachable"
	@ping -c 1 api.localhost > /dev/null && echo "✅ api.localhost is reachable" || echo "❌ api.localhost is NOT reachable"

## Inspect shared proto definitions across services
proto-inspect:
	@echo "--- Checking generated protos ---"
	docker compose exec app_api ls -R /usr/src/app/proto
	docker compose exec app_artifact_builder ls -R /usr/src/app/proto

## Open an interactive shell inside the Router container
router-debug:
	@echo "--- Entering app_router. Type 'exit' to leave ---"
	docker compose exec -it app_router sh

## Inspect the frontend artifacts from the Router's perspective
router-inspect-frontend-artifact:
	@echo "--- Checking files in /usr/share/caddy (Router's perspective) ---"
	docker compose exec app_router ls -R /usr/share/caddy

## Open an interactive shell inside the API container
api-debug:
	@echo "--- Entering app_api. Type 'exit' to leave ---"
	docker compose exec -it app_api sh

## Manually trigger a fresh WASM build by restarting the builder container
artifact-builder-rebuild:
	@echo "--- Restarting WASM builder to trigger a new build cycle ---"
	docker compose restart app_artifact_builder

## Stream live logs from the WASM builder to monitor 'cargo watch' status
artifact-builder-watch-logs:
	@echo "--- Streaming live builder logs... ---"
	docker compose logs -f app_artifact_builder

## Inspect the output directory of the WASM builder directly
artifact-builder-inspect-artifact:
	@echo "--- Checking files in /artifact_pkg (Builder's perspective) ---"
	docker compose exec app_artifact_builder ls -la /artifact_pkg

## Setup VS Code settings from example file
tools-setup-vscode:
	@@if [ -f .vscode/settings.json ]; then \
		echo "⚠️  .vscode/settings.json already exists. Skipping..."; \
	else \
		echo "--- VS Code AI Configuration ---"; \
		read -p "Enter your Google Cloud Project ID: " project_id; \
		sed "s/YOUR-GOOGLE-CLOUD-PROJECT-ID-HERE/$$project_id/g" .vscode/settings.json.example > .vscode/settings.json; \
		echo "✅ .vscode/settings.json generated successfully."; \
	fi
