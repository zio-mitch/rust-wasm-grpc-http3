.PHONY: help all tools-setup-vscode \ 
	services-start services-stop services-reset \ 
	proto-inspect \
	router-debug router-inspect-frontend-artifact \
	api-debug \
	wasm-client-builder-rebuild wasm-client-builder-watch-logs wasm-client-builder-inspect-artifact

.DEFAULT_GOAL := help

DOCKER_COMPOSE := DOCKER_BUILDKIT=1 docker compose -f .devcontainer/docker-compose.yml --env-file .devcontainer/.env

help: ## Show this help message
	@echo "--- 🛠️ gRPC-Rust Project Management ---"
	@echo "Usage: make [target]"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-35s\033[0m %s\n", $$1, $$2}'

all: tools-setup-vscode services-start ## Quickstart

tools-setup-vscode: ## Setup VS Code settings from example file
	@if [ -f ./.vscode/settings.json ]; then \
		echo ".vscode/settings.json already exists. Skipping..."; \
	else \
		cp ./.vscode/settings.json.example ./.vscode/settings.json \
		echo "✅  generated successfully."; \
	fi

services-start: ensure-local-hosts ## Start all containers
	@echo "--- Starting all services ---"
	$(DOCKER_COMPOSE) up -d

services-stop: ## Stop all containers
	@echo "--- Stopping all services ---"
	$(DOCKER_COMPOSE) down


services-reset: ## Reset services: deep clean of containers, volumes, orphans and force a fresh build
	@echo "--- Resetting all services (Deep Clean, consumes bandwidth and is slow!) ---"
	$(DOCKER_COMPOSE) down -v --remove-orphans && \
	$(DOCKER_COMPOSE) build --parallel --no-cache && \
	$(DOCKER_COMPOSE) up -d --force-recreate

services-status: ## Show status of services
	@echo "--- Showing containers status --- "
	$(DOCKER_COMPOSE) ps -a

ensure-local-hosts: ## Check if host can access the domains used by services (e.g., client.localhost, api.localhost)
	@echo "--- Checking host access to service domains ---"
	@ping -c 1 client.localhost > /dev/null && echo "✅ client.localhost is reachable" || echo "❌ client.localhost is NOT reachable"
	@ping -c 1 api.localhost > /dev/null && echo "✅ api.localhost is reachable" || echo "❌ api.localhost is NOT reachable"

proto-inspect: ## Inspect shared proto definitions across services
	@echo "--- Checking generated protos ---"
	$(DOCKER_COMPOSE) exec app_api ls -R /usr/src/app/proto
	$(DOCKER_COMPOSE) exec app_client_artifact_builder ls -R /usr/src/app/proto


router-debug: ## Open an interactive shell inside the Router container
	@echo "--- Entering app_router. Type 'exit' to leave ---"
	$(DOCKER_COMPOSE) exec -it app_router sh

router-inspect-frontend-artifact: ## Inspect the frontend artifacts from the Router's perspective
	@echo "--- Checking files in /usr/share/caddy (Router's perspective) ---"
	$(DOCKER_COMPOSE) exec app_router ls -R /usr/share/caddy

api-debug: ## Open an interactive shell inside the API container
	@echo "--- Entering app_api. Type 'exit' to leave ---"
	$(DOCKER_COMPOSE) exec -it app_api sh

wasm-client-builder-rebuild: ## Manually trigger a fresh WASM client build by restarting the builder container
	@echo "--- Restarting WASM client builder to trigger a new build cycle ---"
	$(DOCKER_COMPOSE) restart app_client_artifact_builder

wasm-client-builder-watch-logs: ## Stream live logs from the WASM client builder to monitor 'cargo watch' status
	@echo "--- Streaming live builder logs... ---"
	$(DOCKER_COMPOSE) logs -tf app_client_artifact_builder

wasm-client-builder-inspect-artifact: ## Inspect the output directory of the WASM client builder directly
	@echo "--- Checking files in /artifact_pkg (Builder's perspective) ---"
	$(DOCKER_COMPOSE) exec app_client_artifact_builder ls -la /artifact_pkg

