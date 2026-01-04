DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: compile

# renovate: depName=ghcr.io/developerc286/clean_git_history
CLEAN_GIT_HISTORY_VERSION=1.1.6@sha256:93fd9c692f6e629956921b8d068ccad33760882b6e0c6d4d32cd963380aec25f

.PHONY: check-clean-git-history
check-clean-git-history:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/clean_git_history:$(CLEAN_GIT_HISTORY_VERSION) $(FROM)

# renovate: depName=ghcr.io/developerc286/conventional_commits_linter
CONVENTIONAL_COMMITS_LINTER_VERSION=0.17.1@sha256:f1b947937ee884ba7f886d04939cd4858f9aeafb50dcf94925a516c50e43021b

.PHONY: check-conventional-commits-linting
check-conventional-commits-linting:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/developerc286/conventional_commits_linter:$(CONVENTIONAL_COMMITS_LINTER_VERSION) --type angular $(FROM)

# renovate: depName=ghcr.io/google/yamlfmt
YAMLFMT_VERSION=0.20.0@sha256:cd11483ba1119371593a7d55386d082da518e27dd932ee00db32e5fb6f3a58c0

.PHONY: check-yaml-formatting
check-yaml-formatting:
	docker run $(DOCKER_RUN_OPTS) ghcr.io/google/yamlfmt:$(YAMLFMT_VERSION) -verbose -lint -dstar .github/workflows/*

.PHONY: fix-yaml-formatting
fix-yaml-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) ghcr.io/google/yamlfmt:$(YAMLFMT_VERSION) -verbose -dstar .github/workflows/*

# renovate: depName=rhysd/actionlint
ACTIONLINT_VERSION=1.7.10@sha256:ef8299f97635c4c30e2298f48f30763ab782a4ad2c95b744649439a039421e36

.PHONY: check-github-actions-workflows-linting
check-github-actions-workflows-linting:
	docker run $(DOCKER_RUN_WRITE_OPTS) rhysd/actionlint:$(ACTIONLINT_VERSION) -verbose -color
