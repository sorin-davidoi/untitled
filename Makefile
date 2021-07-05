SHELL := /bin/bash
.POSIX:
.PHONY: commitlint editorconfig-checker lint fmt test
.EXPORT_ALL_VARIABLES:
DOCKER_BUILDKIT = 1
COMPOSE_DOCKER_CLI_BUILD = 1

commitlint: ## Check that the commit messages adhere to the Conventional Commits specification
	docker-compose run --rm commitlint

editorconfig-checker: ## Check that the files respect the preferences in .editorconfig
	docker-compose run --rm editorconfig-checker


lint: ## Check code for common mistakes with cargo-clippy
	docker-compose run --rm app cargo clippy --all-targets --all-features -- --deny warnings --deny missing_docs --deny clippy::missing_docs_in_private_items

fmt: ## Check code formatting with cargo-fmt
	docker-compose run --rm app cargo fmt

test: ## Run tests with cargo test
	docker-compose run --rm app cargo test --all-features
