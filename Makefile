PROJECT_DIR := $(dir $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: fmt fmt-haskell fmt-nix

fmt: fmt-rs fmt-nix

fmt-rs:
	rustfmt $$(find ${PROJECT_DIR} -name '*.rs')
	
fmt-nix:
	nixfmt $$(find ${PROJECT_DIR} -name '*.nix')
