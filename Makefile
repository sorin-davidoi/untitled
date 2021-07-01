SHELL := /bin/bash
.POSIX:
.PHONY: editorconfig-checker
.EXPORT_ALL_VARIABLES:
DOCKER_BUILDKIT = 1
COMPOSE_DOCKER_CLI_BUILD = 1

editorconfig-checker: ## Check that the files respect the preferences in .editorconfig
	docker-compose run --rm editorconfig-checker
