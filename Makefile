PREFIX          ?= $(HOME)/.local
BINDIR           = $(PREFIX)/bin
EXTENSION_UUID   = ff-profiles@baxyz.tech
EXTENSION_DIR    = $(HOME)/.local/share/gnome-shell/extensions/$(EXTENSION_UUID)
DESKTOP_DIR      = $(HOME)/.local/share/applications

.PHONY: all build install install-bin install-extension install-desktop uninstall clean

all: build

build:
	cargo build --release

install: install-bin install-extension install-desktop
	@echo ""
	@echo "Installation complete."
	@echo "  Binary   : $(BINDIR)/ff-profiles"
	@echo "  Extension: $(EXTENSION_DIR)"
	@echo ""
	@echo "Enable the GNOME Shell extension with:"
	@echo "  gnome-extensions enable $(EXTENSION_UUID)"
	@echo "or via GNOME Extensions app / extensions.gnome.org"

install-bin: build
	install -Dm755 target/release/ff-profiles $(BINDIR)/ff-profiles

install-extension:
	install -Dm644 extension/metadata.json  $(EXTENSION_DIR)/metadata.json
	install -Dm644 extension/extension.js   $(EXTENSION_DIR)/extension.js

install-desktop:
	install -Dm644 data/tech.baxyz.ff-profiles.desktop \
		$(DESKTOP_DIR)/tech.baxyz.ff-profiles.desktop

uninstall:
	rm -f  $(BINDIR)/ff-profiles
	rm -rf $(EXTENSION_DIR)
	rm -f  $(DESKTOP_DIR)/tech.baxyz.ff-profiles.desktop

clean:
	cargo clean
