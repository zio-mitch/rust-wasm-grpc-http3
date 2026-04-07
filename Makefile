.PHONY: help all tools-setup-vscode \ 
	services-start services-stop services-reset \ 
	proto-inspect \
	router-debug router-inspect-frontend-artifact \
	api-debug \
	wasm-artifact-builder-rebuild wasm-artifact-builder-watch-logs wasm-artifact-builder-inspect-artifact

.DEFAULT_GOAL := help

help: ## Show this help message
	@echo "--- 🛠️ gRPC-Rust Project Management ---"
	@echo "Usage: make [target]"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-35s\033[0m %s\n", $$1, $$2}'

all: tools-setup-vscode services-start ## Quickstart

tools-setup-vscode: ## Setup VS Code settings from example file
	@if [ -f .vscode/settings.json ]; then \
		echo "⚠️  .vscode/settings.json already exists. Skipping..."; \
	else \
		echo "--- VS Code AI Configuration ---"; \
		read -p "Enter your Google Cloud Project ID: " project_id; \
		sed "s/YOUR-GOOGLE-CLOUD-PROJECT-ID-HERE/$$project_id/g" .vscode/settings.json.example > .vscode/settings.json; \
		echo "✅ .vscode/settings.json generated successfully."; \
	fi

services-start: ensure-local-hosts ## Start all containers
	@echo "--- Starting all services ---"
	docker compose up -d

services-stop: ## Stop all containers
	@echo "--- Stopping all services ---"
	docker compose down


services-reset: ## Reset services: deep clean of containers, volumes, orphans and force a fresh build
	@echo "--- Resetting all services (Deep Clean, consumes bandwidth and is slow!) ---"
	docker compose down -v --remove-orphans && \
	docker compose build --no-cache && \
	docker compose up -d --force-recreate

ensure-local-hosts: ## Check if host can access the domains used by services (e.g., app.localhost, api.localhost)
	@echo "--- Checking host access to service domains ---"
	@ping -c 1 app.localhost > /dev/null && echo "✅ app.localhost is reachable" || echo "❌ app.localhost is NOT reachable"
	@ping -c 1 api.localhost > /dev/null && echo "✅ api.localhost is reachable" || echo "❌ api.localhost is NOT reachable"

proto-inspect: ## Inspect shared proto definitions across services
	@echo "--- Checking generated protos ---"
	docker compose exec app_api ls -R /usr/src/app/proto
	docker compose exec app_wasm_artifact_builder ls -R /usr/src/app/proto


router-debug: ## Open an interactive shell inside the Router container
	@echo "--- Entering app_router. Type 'exit' to leave ---"
	docker compose exec -it app_router sh

router-inspect-frontend-artifact: ## Inspect the frontend artifacts from the Router's perspective
	@echo "--- Checking files in /usr/share/caddy (Router's perspective) ---"
	docker compose exec app_router ls -R /usr/share/caddy

api-debug: ## Open an interactive shell inside the API container
	@echo "--- Entering app_api. Type 'exit' to leave ---"
	docker compose exec -it app_api sh

wasm-artifact-builder-rebuild: ## Manually trigger a fresh WASM build by restarting the builder container
	@echo "--- Restarting WASM builder to trigger a new build cycle ---"
	docker compose restart app_wasm_artifact_builder


wasm-artifact-builder-watch-logs: ## Stream live logs from the WASM builder to monitor 'cargo watch' status
	@echo "--- Streaming live builder logs... ---"
	docker compose logs -f app_wasm_artifact_builder

wasm-artifact-builder-inspect-artifact: ## Inspect the output directory of the WASM builder directly
	@echo "--- Checking files in /artifact_pkg (Builder's perspective) ---"
	docker compose exec app_wasm_artifact_builder ls -la /artifact_pkg

