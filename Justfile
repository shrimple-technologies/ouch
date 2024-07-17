build:
	@blueprint-compiler batch-compile \
		ui \
		ui \
		src/ui/window.blp
	@cargo build

run:
	@blueprint-compiler batch-compile \
		ui \
		ui \
		src/ui/window.blp
	@cargo run
