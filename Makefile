# Declare phony targets to prevent conflicts with local files
.PHONY: inspect-protos inspect-frontend debug-router inspect-builder-out rebuild-frontend watch-frontend-logs

## Inspect shared proto definitions across services
inspect-protos:
	@echo "--- Checking generated protos ---"
	docker compose exec app_artifact_builder ls -R /usr/src/app/proto

## List the frontend artifacts from the Router's perspective
inspect-frontend:
	@echo "--- Checking files in /usr/share/caddy (Router's perspective) ---"
	docker compose exec app_router ls -R /usr/share/caddy

## Open an interactive shell inside the Router container
debug-router:
	@echo "--- Entering app_router. Type 'exit' to leave ---"
	docker compose exec -it app_router sh

## Inspect the output directory of the WASM builder directly
# Verify if wasm-pack successfully wrote to the /artifact_pkg volume mount
inspect-builder-out:
	@echo "--- Checking files in /artifact_pkg (Builder's perspective) ---"
	docker compose exec app_artifact_builder ls -la /artifact_pkg

## Manually trigger a fresh WASM build by restarting the builder container
rebuild-frontend:
	@echo "--- Restarting WASM builder to trigger a new build cycle ---"
	docker compose restart app_artifact_builder

## Stream live logs from the WASM builder to monitor 'cargo watch' status
watch-frontend-logs:
	@echo "--- Streaming live builder logs... ---"
	docker compose logs -f app_artifact_builder