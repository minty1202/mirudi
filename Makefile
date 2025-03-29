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
