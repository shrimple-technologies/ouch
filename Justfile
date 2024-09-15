VERSION := "0.4.1"
ID := "site.srht.shrimple.ouch"
BLUEPRINT_FILES := "src/ui/window.blp src/ui/about.blp src/ui/about-shrimple.blp src/ui/help-overlay.blp src/ui/preferences.blp"

default:
	@just --list

build:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		{{ BLUEPRINT_FILES }}
	@cargo build --features devel

build-release:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		{{ BLUEPRINT_FILES }}
	@cargo build --release

build-flatpak:
	@flatpak-builder \
		--force-clean \
		--user \
		--gpg-sign=22C359EDF1E87959D2DAD548E4BE7E015E072434 \
		--repo=.build/repo \
		.build \
		build-aux/flatpak/{{ ID }}.json
	@flatpak build-bundle \
		.build/repo \
		{{ ID }}.flatpak \
		{{ ID }} \
		--runtime-repo=https://flathub.org/repo/flathub.flatpakrepo

# MAINTAINERS: Run this command when you have finished releasing a new version of ouch (e.g. pushed version bump commit, pushed tag, updated tar.gz file checksums).
pack:
	@rm -rf ouch-*.*.* .tmp {{ ID }}.flatpak .build .flatpak-builder
	@mkdir -p .tmp .tmp/assets
	@just build-release
	@just build-flatpak
	@cp {{ ID }}.flatpak .tmp
	@cp target/release/ouch .tmp
	@cp res/{{ ID }}.desktop .tmp/assets
	@cp res/{{ ID }}.svg .tmp/assets
	@cp res/{{ ID }}-symbolic.svg .tmp/assets
	@cp res/site.srht.shrimple.svg .tmp/assets
	@cp licenses/GPL-3.0-or-later.txt .tmp
	@mv .tmp/GPL-3.0-or-later.txt .tmp/license.txt
	@echo "\#!/usr/bin/bash" >> .tmp/install.sh
	@echo "sudo install -Dm 755 ouch --target-directory /usr/local/bin" >> .tmp/install.sh
	@echo "sudo install -Dm 644 assets/site.srht.shrimple.ouch.desktop --target-directory /usr/share/applications" >> .tmp/install.sh
	@echo "sudo install -Dm 644 assets/site.srht.shrimple.ouch.svg --target-directory /usr/share/icons/hicolor/scalable/apps/" >> .tmp/install.sh
	@echo "sudo install -Dm 644 assets/site.srht.shrimple.ouch-symbolic.svg --target-directory /usr/share/icons/hicolor/symbolic/apps/" >> .tmp/install.sh
	@echo "sudo install -Dm 644 assets/site.srht.shrimple.svg --target-directory /usr/share/icons/hicolor/scalable/apps/" >> .tmp/install.sh
	@chmod +x .tmp/install.sh
	@tar \
		-czvf \
		ouch-{{ VERSION }}.tar.gz \
		--directory=.tmp \
		.
	@rm -rf .tmp {{ ID }}.flatpak .build .flatpak-builder
	
clean:
	@rm -rf .tmp {{ ID }}.flatpak .build .flatpak-builder ouch-*.*.*

fmt:
	@blueprint-compiler format -f -t -s 4 {{ BLUEPRINT_FILES }}
	@cargo fmt
	
check:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		{{ BLUEPRINT_FILES }}
	@cargo check

build-translations:
	@sudo msgfmt -o /usr/share/locale/fr/LC_MESSAGES/ouch.mo po/fr.po
	@sudo msgfmt -o /usr/share/locale/pt_BR/LC_MESSAGES/ouch.mo po/pt_BR.po
	@sudo msgfmt -o /usr/share/locale/nb_NO/LC_MESSAGES/ouch.mo po/nb_NO.po

run:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp \
		src/ui/about.blp \
		src/ui/help-overlay.blp \
		src/ui/about-shrimple.blp \
		src/ui/preferences.blp
	@cargo run --features devel
