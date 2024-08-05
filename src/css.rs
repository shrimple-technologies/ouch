pub fn init(_app: &adw::Application) {
	let provider = gtk::CssProvider::new();
	let style_manager = adw::StyleManager::default();

	if style_manager.is_dark() {
		provider.load_from_string(include_str!("css/dark.css"));
	} else {
		provider.load_from_string(include_str!("css/light.css"));
	}

	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to a display."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}
