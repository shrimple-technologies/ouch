use adw::prelude::*;
use gio::ActionEntry;
use glib::clone;
use gtk::glib;
use url::Url;
use webkit6::{prelude::*, NetworkError, WebView};

pub fn init(app: &adw::Application) {
	let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));
	let window = builder
		.object::<adw::ApplicationWindow>("window")
		.expect("Couldn't get window");
	let osv = builder
		.object::<adw::OverlaySplitView>("osv")
		.expect("Couldn't get osv");
	let web_view_frame = builder
		.object::<gtk::Frame>("frame")
		.expect("Couldn't get web view frame");
	let url_dialog = builder
		.object::<adw::Dialog>("url_dialog")
		.expect("Couldn't get url dialog");
	let url_button = builder
		.object::<gtk::Button>("url_button")
		.expect("Couldn't get url button");
	let toggle_sidebar = builder
		.object::<gtk::ToggleButton>("toggle_sidebar")
		.expect("Couldn't get sidebar toggle");
	let url_bar = builder
		.object::<gtk::Entry>("url_bar")
		.expect("Couldn't get url bar");
	let url_bar_button = builder
		.object::<gtk::Button>("url_bar_button")
		.expect("Couldn't get url bar button");

	let help_overlay = gtk::Builder::from_string(include_str!("ui/help-overlay.ui"))
		.object::<gtk::ShortcutsWindow>("help_overlay")
		.expect("Couldn't get help overlay");
	help_overlay.set_transient_for(Some(&window));

	let preferences = gtk::Builder::from_string(include_str!("ui/preferences.ui"))
		.object::<adw::PreferencesDialog>("preferences")
		.expect("Couldn't get preferences dialog");

	let about = gtk::Builder::from_string(include_str!("ui/about.ui"))
		.object::<adw::AboutWindow>("about")
		.expect("Couldn't get about window");
	about.set_transient_for(Some(&window));
	about.set_developers(&["Max Walters", "Ally Walters"]);
	about.add_acknowledgement_section(
		Some("Acknowledgements"),
		&["The Browser Company", "The GNOME Developers"],
	);
	about.add_acknowledgement_section(Some("Banner designs"), &["Max Walters"]);

	let about = gtk::Builder::from_string(include_str!("ui/about-shrimple.ui"))
		.object::<adw::AboutWindow>("about_shrimple")
		.expect("Couldn't get about window");

	#[cfg(feature = "devel")]
	window.add_css_class("devel");

	let web_view = WebView::new();

	web_view.connect_load_failed(|web_view, _, fail_url, error| {
		if !error.matches(NetworkError::Cancelled) {
			let content = error_page(error.message());
			web_view.load_alternate_html(&content, fail_url, None);
		}
		false
	});

	url_dialog.present(Some(&window));
	web_view_frame.set_child(Some(&web_view));

	window.set_application(Some(app));
	window.present();

	let action_quit = ActionEntry::builder("quit")
		.activate(|window: &adw::ApplicationWindow, _, _| {
			window.close();
		})
		.build();
	let action_cmd = ActionEntry::builder("cmd")
		.activate(clone!(
			#[strong]
			web_view,
			#[strong]
			url_bar,
			#[strong]
			url_dialog,
			#[strong]
			window,
			move |_, _, _| {
				let buffer = gtk::EntryBuffer::new(web_view.uri());
				url_bar.set_buffer(&buffer);
				url_dialog.present(Some(&window));
			}
		))
		.build();
	let action_about = ActionEntry::builder("about")
		.activate(move |_, _, _| {
			about.present();
		})
		.build();
	let action_help = ActionEntry::builder("show-help-overlay")
		.activate(move |_, _, _| {
			help_overlay.present();
		})
		.build();
	let action_preferences = ActionEntry::builder("show-preferences")
		.activate(clone!(
			#[strong]
			window,
			move |_, _, _| {
				preferences.present(Some(&window));
			}
		))
		.build();
	window.add_action_entries([
		action_quit,
		action_cmd,
		action_about,
		action_help,
		action_preferences,
	]);

	url_dialog.connect_close_attempt(clone!(
		#[strong]
		url_dialog,
		move |_| {
			url_dialog.close();
		}
	));

	url_bar.connect_activate(clone!(
		#[strong]
		web_view,
		#[strong]
		url_dialog,
		#[strong]
		url_button,
		move |url_bar| {
			let url = url_bar.buffer().text().as_str().to_string();
			web_view.load_uri(&format!("https://{url}"));

			let url = Url::parse(
				&web_view
					.uri()
					.expect("Couldn't get web view's url")
					.as_str(),
			);
			url_button.set_label(
				url.expect("Couldn't get url")
					.host_str()
					.expect("Couldn't get url's host"),
			);

			url_dialog.close();
		}
	));

	toggle_sidebar.clone().connect_clicked(move |_| {
		osv.set_show_sidebar(toggle_sidebar.is_active());
	});

	url_button.connect_clicked(clone!(
		#[strong]
		url_dialog,
		#[strong]
		web_view,
		#[strong]
		url_bar,
		move |_| {
			let buffer = gtk::EntryBuffer::new(web_view.uri());
			url_bar.set_buffer(&buffer);
			url_dialog.present(Some(&window));
		}
	));

	url_bar_button.connect_clicked(clone!(
		#[strong]
		url_bar,
		#[strong]
		web_view,
		#[strong]
		url_dialog,
		move |_| {
			let url = url_bar.buffer().text().as_str().to_string();
			web_view.load_uri(&format!("https://{url}"));
			url_dialog.close();
		}
	));
}

fn error_page(msg: &str) -> String {
	format!(
		r#"
		<!doctype html>
			<html>
				<head>
					<style>
						* {{ margin: 0; }}

	   					body {{
							font-family: sans-serif;
		   					margin-left: 20vh;
							margin-right: 20vh;
							position: absolute;
							top: 40%;
							transform: translateY(-40%);
						}}

						svg {{ margin-bottom: 0.5rem; }}
					</style>
				</head>
				<body>
					<svg height="64px" viewBox="0 0 16 16" width="64px" xmlns="http://www.w3.org/2000/svg">
						<path d="m 3 0 c -1.660156 0 -3 1.339844 -3 3 v 7 c 0 1.660156 1.339844 3 3 3 h 10 c 1.660156 0 3 -1.339844 3 -3 v -7 c 0 -1.660156 -1.339844 -3 -3 -3 z m 0 2 h 10 c 0.554688 0 1 0.445312 1 1 v 7 c 0 0.554688 -0.445312 1 -1 1 h -10 c -0.554688 0 -1 -0.445312 -1 -1 v -7 c 0 -0.554688 0.445312 -1 1 -1 z m 3 2 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m 4 0 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m -2 3 c -1.429688 0 -2.75 0.761719 -3.464844 2 c -0.136718 0.238281 -0.054687 0.546875 0.183594 0.683594 c 0.238281 0.136718 0.546875 0.054687 0.683594 -0.183594 c 0.535156 -0.929688 1.523437 -1.5 2.597656 -1.5 s 2.0625 0.570312 2.597656 1.5 c 0.136719 0.238281 0.445313 0.320312 0.683594 0.183594 c 0.238281 -0.136719 0.320312 -0.445313 0.183594 -0.683594 c -0.714844 -1.238281 -2.035156 -2 -3.464844 -2 z m -3 7 c -1.105469 0 -2 0.894531 -2 2 h 10 c 0 -1.105469 -0.894531 -2 -2 -2 z m 0 0" fill="\#2e3436"/>
					</svg>
					<h3>There was an error loading this website</h3>
					<small>{msg}</small>
				</body>
			</html>"#
	)
}
