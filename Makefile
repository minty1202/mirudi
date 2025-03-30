BINARY_NAME=mirudi
INSTALL_PATH=${HOME}/.local/bin

build:
	cargo build --release

install: build
	mkdir -p $(INSTALL_PATH)
	cp target/release/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "Installed to $(INSTALL_PATH)/$(BINARY_NAME)"

uninstall:
	rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@echo "Uninstalled from $(INSTALL_PATH)/$(BINARY_NAME)"

lint:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo fmt --all -- --check
	@echo "Code style checked"

format:
	cargo fmt --all
	@echo "Code formatted"

test:
	cargo test --all-targets --all-features -- --nocapture
	@echo "Tests passed"
