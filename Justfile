build:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp \
		src/ui/about.blp \
		src/ui/help-overlay.blp \
		src/ui/preferences.blp
	@cargo build

run:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp \
		src/ui/about.blp \
		src/ui/help-overlay.blp \
		src/ui/preferences.blp
	@cargo run
