# Ouch Browser

Focus on your browsing.

## Building

To build Ouch, you need [GTK 4](https://gitlab.gnome.org/GNOME/gtk), [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita), and [Blueprint](https://gitlab.gnome.org/jwestman/blueprint-compiler) installed on your system.

```sh
blueprint-compiler batch-compile \
	ui \
	ui \
	src/ui/window.blp
cargo build
# cargo run
```
