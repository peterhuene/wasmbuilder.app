MODULES := ./node_modules/.bin
ESLINT := $(MODULES)/eslint
PARCEL := $(MODULES)/parcel
CARGO := cargo
YARN := yarn
NPX := npx
ENTRY_POINT := src/index.html
WASM_OPT := wasm-opt
WASM_STRIP := wasm-strip

help:
	@grep -E '^[a-zA-Z\._-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## builds the graph component
	@$(CARGO) component build --release -p graph

bindgen: build ## generates bindings for the graph component
	@$(NPX) jsct transpile target/wasm32-unknown-unknown/release/graph.wasm --tla-compat -o src

opt: bindgen # optimizes the graph wasm module
	@$(WASM_OPT) -Os src/graph.core.wasm -o src/graph.core.wasm
	@$(WASM_STRIP) src/graph.core.wasm

bundle: opt ## bundles the application
	@$(PARCEL) build $(ENTRY_POINT)

format: ## formats source code
	@$(CARGO) fmt
	@$(YARN) run prettier -w .

test: ## runs tests
	@$(CARGO) test

lint: ## runs linting
	@$(CARGO) component clippy --release --target wasm32-unknown-unknown
	@$(ESLINT) src

run: bindgen ## runs development
	@$(PARCEL) $(ENTRY_POINT) -p 3000

setup: ## installs build dependencies
	@$(YARN)
	@$(CARGO) install --git https://github.com/bytecodealliance/cargo-component
