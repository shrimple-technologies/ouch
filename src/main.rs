use gtk::{glib, prelude::*};
#[cfg(feature = "css")] mod css;
mod window;

fn main() -> glib::ExitCode {
	let app = adw::Application::builder()
		.application_id("ht.sr.git.shrimple.Ouch")
		.build();

	#[cfg(feature = "css")] app.connect_startup(css::init);
	app.connect_activate(window::init);

	app.set_accels_for_action("win.quit", &["<Ctrl>Q"]);
	app.set_accels_for_action("win.cmd", &["<Ctrl>L"]);

	app.run()
}
