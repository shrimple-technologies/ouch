#[cfg(feature = "css")] mod css;
mod window;
use gtk::{glib, prelude::*};
// use gio::{SimpleAction, prelude::*};

fn main() -> glib::ExitCode {
	let app = adw::Application::builder()
		.application_id("ht.sr.git.shrimple.Ouch")
		.build();

	// create_action(&app, "about", &about);

	#[cfg(feature = "css")] app.connect_startup(css::init);
	app.connect_activate(window::init);

	app.run()
}

/* fn about() {
	let about = adw::AboutWindow::builder()
		.transient_for(&window::init)
		.application_name("Ouch Browser")
		.application_icon("ht.sr.git.shrimple.Ouch")
		.developer_name("Shrimple Technologies")
		.version("0.1.0")
		.developers([
			"Max Walters https://max.is-probably.gay/",
			"Ally Walters"
		])
		.license_type(gtk::License::Gpl30)
		.issue_url("https://todo.sr.ht/~shrimple/ouch")
		.release_notes("
		<ul>
			<li>Initial preview release!</li>
		</ul>");
	about.present();
}

fn create_action(app: &adw::Application, name: &str, callback: &Fn) {
	let action = gio::SimpleAction::builder()
		.(name, None);
	action.connect("activate", callback);
	app.add_action(action);
} */

