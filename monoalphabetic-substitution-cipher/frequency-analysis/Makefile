DOCKER_RUN_OPTS := --rm -v $(PWD):/workspace -w /workspace

UID := $(shell id -u)
GID := $(shell id -g)
DOCKER_RUN_WRITE_OPTS := $(DOCKER_RUN_OPTS) -u $(UID):$(GID)

.PHONY: default
default: compile

.PHONY: check-rust-formatting
check-rust-formatting:
	docker build -t check-rust-formatting -f ci/check-rust-formatting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-rust-formatting

# renovate: depName=mvdan/shfmt
SHFMT_VERSION=v3.12.0-alpine@sha256:204a4d2d876123342ad394bd9a28fb91e165abc03868790d4b39cfa73233f150

.PHONY: check-shell-formatting
check-shell-formatting:
	docker run $(DOCKER_RUN_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --diff ci/*

.PHONY: fix-rust-formatting
fix-rust-formatting:
	docker build -t fix-rust-formatting -f ci/fix-rust-formatting.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) fix-rust-formatting

.PHONY: fix-shell-formatting
fix-shell-formatting:
	docker run $(DOCKER_RUN_WRITE_OPTS) mvdan/shfmt:$(SHFMT_VERSION) --simplify --write ci/*

.PHONY: check-rust-linting
check-rust-linting:
	docker build -t check-rust-linting -f ci/check-rust-linting.Dockerfile .
	docker run $(DOCKER_RUN_OPTS) check-rust-linting

.PHONY: compile
compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) compile

.PHONY: unit-test
unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run $(DOCKER_RUN_WRITE_OPTS) unit-test
