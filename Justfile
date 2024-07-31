build:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp
	@cargo build

run:
	@blueprint-compiler batch-compile \
		src/ui \
		src/ui \
		src/ui/window.blp
	@cargo run
