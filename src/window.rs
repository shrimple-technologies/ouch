use adw::prelude::*;

pub fn init(app: &adw::Application) {
	let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));
	let window = builder
		.object::<adw::ApplicationWindow>("window")
		.expect("Couldn't get window");

	window.set_application(Some(app));
	window.present();
}
