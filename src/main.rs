use gtk::{glib, prelude::*};
mod css;
mod window;

fn main() -> glib::ExitCode {
	#[cfg(feature = "debug")]
	#[cfg(feature = "devel")]
	println!("[ouch/main] running as developer build");

	let app = adw::Application::builder()
		.application_id("site.srht.shrimple.Ouch")
		.build();

	app.connect_startup(css::init);
	app.connect_activate(window::init);

	app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
	app.set_accels_for_action("win.cmd", &["<Ctrl>L"]);
	app.set_accels_for_action("win.show-preferences", &["<Ctrl>comma"]);

	app.run()
}
