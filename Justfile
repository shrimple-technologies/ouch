VERSION := "0.3.3"

build:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp \
		src/ui/about.blp \
		src/ui/about-shrimple.blp \
		src/ui/help-overlay.blp \
		src/ui/preferences.blp
	@cargo build --features devel

build-release:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp \
		src/ui/about.blp \
		src/ui/about-shrimple.blp \
		src/ui/help-overlay.blp \
		src/ui/preferences.blp
	@cargo build --release

build-flatpak:
	@flatpak-builder \
		--force-clean \
		--install \
		--user \
		--repo=.build/repo \
		.build \
		build-aux/flatpak/site.srht.shrimple.Ouch.json
	@flatpak build-bundle \
		.build/repo \
		site.srht.shrimple.Ouch.flatpak \
		site.srht.shrimple.Ouch \
		--runtime-repo=https://flathub.org/repo/flathub.flatpakrepo

# MAINTAINERS: Run this command when you have finished releasing a new version of Ouch (e.g. pushed version bump commit, pushed tag, updated tar.gz file checksums).
pack:
	@rm -rf ouch-*.*.* .tmp site.srht.shrimple.Ouch.flatpak .build .flatpak-builder
	@mkdir -p .tmp
	@just build-release
	@just build-flatpak
	@cp site.srht.shrimple.Ouch.flatpak .tmp
	@cp target/release/ouch .tmp
	@cp res/site.srht.shrimple.Ouch.desktop .tmp
	@cp res/site.srht.shrimple.Ouch.svg .tmp
	@cp res/site.srht.shrimple.svg .tmp
	@cp licenses/GPL-3.0-or-later.txt .tmp
	@mv .tmp/GPL-3.0-or-later.txt .tmp/license.txt
	@tar \
		-czvf \
		ouch-{{VERSION}}.tar.gz \
		--directory=.tmp \
		.
	@rm -rf .tmp site.srht.shrimple.Ouch.flatpak .build .flatpak-builder
	
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
