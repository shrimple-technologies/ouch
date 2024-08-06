pub fn init(_: &adw::Application) {
	#[cfg(feature = "debug")]
	println!("[ouch/css] initializing stylesheets...");

	let provider = gtk::CssProvider::new();
	provider.load_from_string(include_str!("css/base.css"));

	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to a display."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);

	#[cfg(feature = "debug")]
	println!("[ouch/css] stylesheets initialized");
}
