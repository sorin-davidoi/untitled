SHELL := /bin/bash
.POSIX:
.PHONY: commitlint editorconfig-checker
.EXPORT_ALL_VARIABLES:
DOCKER_BUILDKIT = 1
COMPOSE_DOCKER_CLI_BUILD = 1

commitlint: ## Check that the commit messages adhere to the Conventional Commits specification
	docker-compose run --rm commitlint

editorconfig-checker: ## Check that the files respect the preferences in .editorconfig
	docker-compose run --rm editorconfig-checker
