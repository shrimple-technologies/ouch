VERSION := "0.4.0"
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
		--install \
		--user \
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
	@mkdir -p .tmp
	@just build-release
	@just build-flatpak
	@cp {{ ID }}.flatpak .tmp
	@cp target/release/ouch .tmp
	@cp res/{{ ID }}.desktop .tmp
	@cp res/{{ ID }}.metainfo.xml .tmp
	@cp res/{{ ID }}.svg .tmp
	@cp res/site.srht.shrimple.svg .tmp
	@cp licenses/GPL-3.0-or-later.txt .tmp
	@mv .tmp/GPL-3.0-or-later.txt .tmp/license.txt
	@tar \
		-czvf \
		ouch-{{ VERSION }}.x86_64.tar.gz \
		--directory=.tmp \
		.
	@rm -rf .tmp {{ ID }}.flatpak .build .flatpak-builder
	
clean:
	@rm -rf .tmp {{ ID }}.flatpak .build .flatpak-builder

fmt:
	@blueprint-compiler format -f -t -s 4 {{ BLUEPRINT_FILES }}
	@cargo fmt
	
check:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		{{ BLUEPRINT_FILES }}
	@cargo check

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
