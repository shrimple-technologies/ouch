use adw::prelude::*;
use webkit6::{prelude::*, WebView};

pub fn init(app: &adw::Application) {
	let builder = gtk::Builder::from_string(
		include_str!("ui/window.ui")
	);
	let window = builder
		.object::<adw::ApplicationWindow>("window")
		.expect("Couldn't get window");
	let web_view_frame = builder
		.object::<gtk::Frame>("frame")
		.expect("Couldn't get web view frame");

	let web_view = WebView::new();

	web_view.load_uri("https://google.com/");
	web_view_frame.set_child(Some(&web_view));

	window.set_application(Some(app));
	window.present();
}
