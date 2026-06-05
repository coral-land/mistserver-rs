.DEFAULT_GOAL := help

# -----------------------------------------------------------------------------
# Commands, add them with ## <the description> to see them when they run make
# -----------------------------------------------------------------------------

up: ## Run docker compose
	docker compose up -d 

down: ## Docker compose down
	docker compose down --remove-orphans

restart: ## Docker compose restart
	docker compose restart

run: ## Run the main process
	@echo "Running main process..."
	cargo run
	# your command here

test: ## Run tests
	@echo "Running tests..."
	# your test command

clean: ## Remove temporary files
	@echo "Cleaning..."

deploy: ## Deploy to production
	@echo "Deploying..."
	# your deploy script

# -----------------------------------------------------------------------------
# Dynamic help – automatically lists all targets with '##'
# -----------------------------------------------------------------------------
help: ## Show this help
	@printf '\n\033[1;34mUsage:\033[0m\n  make \033[36m<target>\033[0m\n\n\033[1;34mTargets:\033[0m\n'
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""

# Optional: catch unknown targets and show help
%:
	@echo "Unknown target: $@"
	@$(MAKE) help --no-print-directory