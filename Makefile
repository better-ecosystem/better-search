APP_NAME = Better-Search
TARGET_DIR = target/release
BIN_PATH = $(TARGET_DIR)/$(APP_NAME)

install: 
	cargo build --release
	install -Dm755 $(BIN_PATH) ~/.local/bin/search

uninstall:
	rm -rf ~/.local/bin/search

