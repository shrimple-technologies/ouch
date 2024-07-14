mod css;
mod window;
use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
	let app = adw::Application::builder()
		.application_id("ht.sr.git.shrimple.Ouch")
		.build();

	app.connect_startup(css::init);
	app.connect_activate(window::init);

	app.run()
}
