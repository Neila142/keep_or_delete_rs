.PHONY: all clean \

RUST_TARGETS := \
	aarch64-apple-darwin \
	x86_64-apple-darwin \
	x86_64-unknown-linux-gnu \
	aarch64-unknown-linux-gnu \
	x86_64-pc-windows-gnu

amd64-win:
	cross build --release --target x86_64-pc-windows-gnu

amd64-linux:
	cross build --release --target x86_64-unknown-linux-gnu

amd64-mac:
	cross build --release --target x86_64-apple-darwin

arm64-linux:
	cross build --release --target aarch64-unknown-linux-gnu

arm64-mac:
	cross build --release --target aarch64-apple-darwin

all: amd64-win amd64-linux amd64-mac arm64-linux arm64-mac

win: amd64-win

mac: arm64-mac amd64-mac

linux: amd64-linux arm64-linux


clean:
	@echo "Cleaning all targets..."
	@for tgt in $(RUST_TARGETS); do \
		echo "→ clean $$tgt"; \
		cargo clean --target $$tgt; \
	done
