# So new files are owned by the user.
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: check-rust-formatting check-shell-formatting fix-rust-formatting fix-shell-formatting check-rust-linting compile unit-test

check-rust-formatting:
	docker build -t check-rust-formatting -f ci/check-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-formatting

# renovate: depName=mvdan/shfmt
SHFMT_VERSION=v3.11.0-alpine@sha256:394d755b6007056a2e6d7537ccdbdcfca01b9855ba91e99df0166ca039c9d422
  
check-shell-formatting:
	docker pull mvdan/shfmt:$(SHFMT_VERSION)
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:$(SHFMT_VERSION) --simplify --diff ci/* 

fix-rust-formatting:
	docker build -t fix-rust-formatting -f ci/fix-rust-formatting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) fix-rust-formatting

fix-shell-formatting:
	docker pull mvdan/shfmt:$(SHFMT_VERSION)
	docker run --rm -v $(PWD):/workspace -w /workspace -u $(UID):$(GID) mvdan/shfmt:$(SHFMT_VERSION) --simplify --write ci/* 

check-rust-linting:
	docker build -t check-rust-linting -f ci/check-rust-linting.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) check-rust-linting

compile:
	docker build -t compile -f ci/compile.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) compile

unit-test:
	docker build -t unit-test -f ci/unit-test.Dockerfile .
	docker run --rm -v $(PWD):/workspace -u $(UID):$(GID) unit-test