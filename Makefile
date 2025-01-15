# Variables
CARGO := cargo
PROJECT_NAME := $(shell grep "name" Cargo.toml | head -n1 | cut -d'"' -f2)
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release
DEBUG_DIR := $(TARGET_DIR)/debug

# Colors for terminal output
CYAN := \033[36m
RESET := \033[0m

.PHONY: all build test clean check fmt lint doc run release help

# Default target
all: build

help:
	@echo "$(CYAN)Available commands:$(RESET)"
	@echo "  make build    - Build the debug version"
	@echo "  make release  - Build the release version"
	@echo "  make test     - Run all tests"
	@echo "  make check    - Run cargo check"
	@echo "  make fmt      - Format code"
	@echo "  make lint     - Run clippy lints"
	@echo "  make doc      - Generate documentation"
	@echo "  make clean    - Clean build artifacts"
	@echo "  make run      - Run debug version"
	@echo "  make bench    - Run benchmarks"
	@echo "  make update   - Update dependencies"
	@echo "  make audit    - Security audit dependencies"

build:
	@echo "$(CYAN)Building debug version...$(RESET)"
	$(CARGO) build

release:
	@echo "$(CYAN)Building release version...$(RESET)"
	$(CARGO) build --release

test:
	@echo "$(CYAN)Running tests...$(RESET)"
	$(CARGO) test --all-features
	$(CARGO) test --doc

check:
	@echo "$(CYAN)Checking project...$(RESET)"
	$(CARGO) check
	$(CARGO) fmt -- --check
	$(CARGO) clippy -- -D warnings

fmt:
	@echo "$(CYAN)Formatting code...$(RESET)"
	$(CARGO) fmt

lint:
	@echo "$(CYAN)Running clippy...$(RESET)"
	$(CARGO) clippy -- -D warnings

doc:
	@echo "$(CYAN)Generating documentation...$(RESET)"
	$(CARGO) doc --no-deps --document-private-items
	@echo "Documentation available at target/doc/$(PROJECT_NAME)/index.html"

clean:
	@echo "$(CYAN)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean
	rm -rf **/*.rs.bk
	rm -rf .coverage
	rm -rf docs

run:
	@echo "$(CYAN)Running debug version...$(RESET)"
	$(CARGO) run

bench:
	@echo "$(CYAN)Running benchmarks...$(RESET)"
	$(CARGO) bench

update:
	@echo "$(CYAN)Updating dependencies...$(RESET)"
	$(CARGO) update

audit:
	@echo "$(CYAN)Auditing dependencies...$(RESET)"
	$(CARGO) audit

# Development workflow targets
dev: fmt lint test
	@echo "$(CYAN)Development checks passed!$(RESET)"

# CI workflow target
ci: check test
	@echo "$(CYAN)CI checks passed!$(RESET)"

# Install development dependencies
setup:
	@echo "$(CYAN)Installing development dependencies...$(RESET)"
	rustup component add clippy rustfmt
	cargo install cargo-audit cargo-watch cargo-tarpaulin

# Watch mode for development
watch:
	@echo "$(CYAN)Watching for changes...$(RESET)"
	cargo watch -x check -x test -x run

# Code coverage
coverage:
	@echo "$(CYAN)Generating code coverage report...$(RESET)"
	cargo tarpaulin --out Html
	@echo "Coverage report available at tarpaulin-report.html"
