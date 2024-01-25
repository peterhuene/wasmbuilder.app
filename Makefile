CARGO := cargo
RUSTUP := rustup
YARN := yarn
NPX := npx
ENTRY_POINT := src/index.html
WASM_OPT := wasm-opt
WASM_STRIP := wasm-strip

help:
	@grep -E '^[a-zA-Z\._-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## builds the graph component
	@$(CARGO) component build --release -p graph --target wasm32-unknown-unknown

bindgen: build ## generates bindings for the graph component
	@$(NPX) jco transpile target/wasm32-unknown-unknown/release/graph.wasm --tla-compat -o src

opt: bindgen # optimizes the graph wasm module
	@$(WASM_OPT) -Os src/graph.core.wasm -o src/graph.core.wasm
	@$(WASM_STRIP) src/graph.core.wasm

bundle: opt ## bundles the application
	@$(NPX) parcel build $(ENTRY_POINT) --no-scope-hoist

format: ## formats source code
	@$(CARGO) fmt
	@$(YARN) run prettier -w .

lint: ## runs linting
	@$(CARGO) component clippy --release --target wasm32-unknown-unknown
	@$(NPX) eslint src

run: bindgen ## runs development
	@$(NPX) parcel $(ENTRY_POINT) -p 3000

setup: ## installs build dependencies
	@$(YARN)
	@$(CARGO) install cargo-component
	@$(RUSTUP) target add wasm32-unknown-unknown
