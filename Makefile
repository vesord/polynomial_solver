CARGO := $(shell which cargo)
INSTALL_RUST = install_rust.sh
NAME = computer1

.PHONY: all
all:
ifndef CARGO
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $(INSTALL_RUST)
	chmod +x $(INSTALL_RUST)
	./$(INSTALL_RUST) -y
	source '$$HOME/.cargo/env'
endif
	cargo build --release
	cp target/release/$(NAME) .

.PHONY: clean
clean:
	rm -rf target

.PHONY: re
re: clean all

.PHONY: test
test:
	cargo test
