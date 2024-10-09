/* window.rs
 *
 * Copyright 2024 Shrimple Technologies
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::prelude::*;
use gettextrs::*;
use glib::clone;
use gtk::gdk;
use gtk::gio;
use gtk::gio::ActionEntry;
use gtk::glib;
use url::Url;
use webkit::{prelude::*, NetworkError, WebView};
#[path = "lua.rs"]
mod lua;
#[path = "consts.rs"]
mod consts;

pub fn init(app: &adw::Application) {
	let builder = gtk::Builder::from_string(include_str!("ui/window.ui"));
	let window = builder
		.object::<adw::ApplicationWindow>("window")
		.expect("Couldn't get window");
	let osv = builder
		.object::<adw::OverlaySplitView>("osv")
		.expect("Couldn't get osv");
	let view = builder
		.object::<adw::TabView>("view")
		.expect("Couldn't get tab view");
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
	let toast_overlay = builder
		.object::<adw::ToastOverlay>("toast_overlay")
		.expect("Couldn't get toast overlay");
	let copy_link_button = builder
		.object::<gtk::Button>("copy_link_button")
		.expect("Couldn't get copy link button");
	let overview = builder
		.object::<adw::TabOverview>("overview")
		.expect("Couldn't get tab overview");
	let tabs = builder
		.object::<gtk::ListView>("tabs")
		.expect("Couldn't get list view");
	let frame = builder
		.object::<gtk::Frame>("frame")
		.expect("Couldn't get web view's frame");

	let help_overlay =
		gtk::Builder::from_string(include_str!("ui/help-overlay.ui"))
			.object::<gtk::ShortcutsWindow>("help_overlay")
			.expect("Couldn't get help overlay");
	help_overlay.set_transient_for(Some(&window));

	let preferences =
		gtk::Builder::from_string(include_str!("ui/preferences.ui"))
			.object::<adw::PreferencesDialog>("preferences")
			.expect("Couldn't get preferences dialog");

	let plugin_manager =
		gtk::Builder::from_string(include_str!("ui/plugin-manager.ui"))
			.object::<adw::PreferencesDialog>("plugin-manager")
			.expect("Couldn't get plugin manager dialog");

	let about = gtk::Builder::from_string(include_str!("ui/about.ui"))
		.object::<adw::AboutDialog>("about")
		.expect("Couldn't get about dialog");
	about.set_developers(&[
		"Max Walters <mdwalters.pm@proton.me>",
		"Ally Walters",
	]);
	about.set_translator_credits(gettext("translator-credits").as_str());
	about.add_acknowledgement_section(
		Some(gettext("Acknowledgements").as_str()),
		&["The Browser Company", "The GNOME Developers"],
	);
	about.add_acknowledgement_section(
		Some(gettext("Icon design").as_str()),
		&["Jakub Stiener https://jimmac.eu/"],
	);
	about.set_version(consts::VERSION);

	let about_shrimple =
		gtk::Builder::from_string(include_str!("ui/about-shrimple.ui"))
			.object::<adw::AboutDialog>("about_shrimple")
			.expect("Couldn't get about dialog");
	about_shrimple.add_acknowledgement_section(
		Some(gettext("Members").as_str()),
		&["Max Walters", "Ally Walters"],
	);

	let oobe_ui = gtk::Builder::from_string(include_str!("ui/oobe.ui"));
	let oobe = oobe_ui
		.object::<adw::Dialog>("oobe")
		.expect("Couldn't get OOBE dialog");
	let oobe_carousel = oobe_ui
		.object::<adw::Carousel>("carousel")
		.expect("Couldn't get OOBE carousel");

	#[cfg(feature = "devel")]
	window.add_css_class("devel");

	let web_view = WebView::new();
	let settings = gio::Settings::new("site.srht.shrimple.ouch");

	web_view.connect_load_changed(clone!(
		#[strong]
		view,
		#[strong]
		url_button,
		#[strong]
		tabs,
		move |_, load_event| {
			match load_event {
				webkit::LoadEvent::Started => {
					view.selected_page()
						.expect("Couldn't get tab page")
						.set_loading(true);
				}
				webkit::LoadEvent::Finished => {
					view.selected_page()
						.expect("Couldn't get tab page")
						.set_loading(false);
					tabs.set_model(Some(&view.pages()));

					let tab_page = view
						.selected_page()
						.expect("Couldn't get tab page")
						.child();
					let web_view = tab_page.downcast_ref::<WebView>().unwrap();

					// see https://todo.sr.ht/~shrimple/ouch/1
					if web_view.title() != None {
						let url = Url::parse(
							web_view
								.uri()
								.expect("Couldn't get web view's url")
								.as_str(),
						);

						let _url = url.clone();
						if url.expect("Couldn't get URL").scheme() == "file" {
							url_button
								.child()
								.expect("Couldn't get command palette toggle's label")
								.downcast_ref::<gtk::Label>()
								.unwrap()
								.set_label(_url.expect("Couldn't get url").path());
						} else {
							url_button
								.child()
								.expect("Couldn't get command palette toggle's label")
								.downcast_ref::<gtk::Label>()
								.unwrap()
								.set_label(
									_url.expect("Couldn't get url")
										.host_str()
										.expect("Couldn't get url's host"),
								);
						}

						view.selected_page()
							.expect("Couldn't get tab page")
							.set_title(
								web_view
									.title()
									.expect("Couldn't get title")
									.as_str(),
							);

						view.selected_page()
							.expect("Couldn't get tab page")
							.set_keyword(
								web_view
									.uri()
									.expect("Couldn't get web view's url")
									.as_str(),
							);
					}
				}
				_ => (),
			}
		}
	));

	web_view.connect_load_failed(|web_view, _, fail_url, error| {
		if !error.matches(NetworkError::Cancelled) {
			let content = error_page(error.message());
			web_view.load_alternate_html(&content, fail_url, None);
		}
		false
	});

	web_view.connect_script_dialog(clone!(
		#[strong]
		toast_overlay,
		#[strong]
		window,
		move |web_view, web_dialog| {
			match web_dialog.dialog_type() {
				webkit::ScriptDialogType::Alert => {
					let url = Url::parse(
						web_view
							.uri()
							.expect("Couldn't get web view's url")
							.as_str(),
					);

					let dialog = adw::AlertDialog::new(
						Some(
							Some(format!(
								"{} says",
								url.expect("Couldn't get url")
									.host_str()
									.expect("Couldn't get url's host"),
							))
							.expect("Couldn't get dialog header")
							.as_str(),
						),
						Some(
							&web_dialog
								.message()
								.expect("Could not get script dialog message"),
						),
					);
					dialog.add_response("default", "OK");
					dialog.set_response_appearance(
						"default",
						adw::ResponseAppearance::Suggested,
					);
					dialog.present(Some(&window));

					true
				}
				_ => {
					toast_overlay.add_toast(adw::Toast::new(
						"This script dialog type is invalid",
					));
					true
				}
			}
		}
	));

	if settings.boolean("oobe-shown") {
		url_dialog.present(Some(&window));
	} else {
		oobe.present(Some(&window));
	}

	view.append(&web_view);

	window.set_application(Some(app));
	window.present();

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
		.activate(clone!(
			#[strong]
			window,
			move |_, _, _| {
				about.present(Some(&window));
			}
		))
		.build();
	let action_about_shrimple = ActionEntry::builder("about-shrimple")
		.activate(clone!(
			#[strong]
			window,
			move |_, _, _| {
				about_shrimple.present(Some(&window));
			}
		))
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
	let action_copy_link = ActionEntry::builder("copy-link")
		.activate(clone!(
			#[strong]
			toast_overlay,
			#[strong]
			web_view,
			#[strong]
			copy_link_button,
			move |_, _, _| {
				if copy_link_button.get_sensitive() {
					let url = Some(web_view.uri().expect("Couldn't get URL"));

					gdk::Display::default()
						.unwrap()
						.clipboard()
						.set_text(url.expect("Couldn't get URL").as_str());
					toast_overlay.add_toast(adw::Toast::new(
						gettext("Link copied").as_str(),
					));
				}
			}
		))
		.build();
	let action_zoom_in = ActionEntry::builder("zoom-in")
		.activate(clone!(
			#[strong]
			web_view,
			#[strong]
			toast_overlay,
			move |_, _, _| {
				web_view.set_zoom_level(web_view.zoom_level() + 0.5);

				toast_overlay.add_toast(adw::Toast::new(
					gettext("Zoomed in to {}%")
						.replace(
							"{}",
							&(web_view.zoom_level() * 100.0).to_string(),
						)
						.as_str(),
				));
			}
		))
		.build();
	let action_oobe = ActionEntry::builder("show-oobe")
		.activate(clone!(
			#[strong]
			oobe,
			#[strong]
			oobe_carousel,
			#[strong]
			window,
			move |_, _, _| {
				oobe_carousel.scroll_to(&oobe_carousel.nth_page(0), false);
				oobe.present(Some(&window));
			}
		))
		.build();
	let action_oobe_replay = ActionEntry::builder("show-oobe-replay")
		.activate(clone!(
			#[strong]
			oobe,
			#[strong]
			oobe_carousel,
			#[strong]
			window,
			move |_, _, _| {
				oobe_carousel.scroll_to(&oobe_carousel.nth_page(0), false);
				oobe.set_can_close(true);
				oobe.present(Some(&window));
			}
		))
		.build();
	let action_oobe_next = ActionEntry::builder("oobe-next")
		.activate(clone!(
			#[strong]
			oobe_carousel,
			move |_, _, _| {
				if ((oobe_carousel.position() as u32) + 1)
					!= oobe_carousel.n_pages()
				{
					oobe_carousel.scroll_to(
						&oobe_carousel
							.nth_page((oobe_carousel.position() as u32) + 1),
						true,
					);
				}
			}
		))
		.build();
	let action_oobe_close = ActionEntry::builder("oobe-close")
		.activate(clone!(
			#[strong]
			oobe,
			#[strong]
			url_dialog,
			#[strong]
			window,
			#[strong]
			settings,
			move |_, _, _| {
				settings.set_boolean("oobe-shown", true).unwrap();
				oobe.force_close();

				if !oobe.can_close() {
					url_dialog.present(Some(&window));
				}
			}
		))
		.build();
	let action_plugin_manager = ActionEntry::builder("show-plugin-manager")
		.activate(clone!(
			#[strong]
			plugin_manager,
			#[strong]
			window,
			move |_, _, _| {
				plugin_manager.present(Some(&window));
			}
		))
		.build();
	window.add_action_entries([
		action_cmd,
		action_about,
		action_about_shrimple,
		action_help,
		action_preferences,
		action_copy_link,
		action_zoom_in,
		action_oobe,
		action_oobe_replay,
		action_oobe_next,
		action_oobe_close,
		action_plugin_manager,
	]);

	url_dialog.connect_close_attempt(|_| ());

	url_bar.connect_activate(clone!(
		#[strong]
		url_dialog,
		#[strong]
		url_button,
		#[strong]
		copy_link_button,
		#[strong]
		view,
		move |url_bar| {
			let url = url_bar.buffer().text().as_str().to_string();
			let tab_page =
				view.selected_page().expect("Couldn't get tab page").child();
			let web_view = tab_page.downcast_ref::<WebView>();

			if url == "" {
				web_view.unwrap().load_uri("https://start.ubuntu.com/");
			} else if let Some(scheme) = glib::Uri::peek_scheme(&url) {
				if scheme.as_str() == "https"
					|| scheme.as_str() == "http"
					|| scheme.as_str() == "file"
				{
					web_view.unwrap().load_uri(url.as_str());
				} else {
					web_view.unwrap().load_uri(&format!(
						"https://google.com/search?q={}",
						glib::Uri::escape_string(url.as_str(), None, true)
							.as_str()
					));
				}
			} else if url.as_str().contains(".") {
				web_view.unwrap().load_uri(&format!("https://{url}"));
			} else {
				web_view.unwrap().load_uri(&format!(
					"https://google.com/search?q={}",
					glib::Uri::escape_string(url.as_str(), None, true).as_str()
				));
			}

			let url = Url::parse(
				web_view
					.unwrap()
					.uri()
					.expect("Couldn't get web view's url")
					.as_str(),
			);

			let _url = url.clone();
			if url.expect("Couldn't get url").scheme() == "file" {
				url_button.set_label(_url.expect("Couldn't get url").path());
			} else {
				url_button
					.child()
					.expect("Couldn't get command palette toggle's label")
					.downcast_ref::<gtk::Label>()
					.unwrap()
					.set_label(
						_url.expect("Couldn't get url")
							.host_str()
							.expect("Couldn't get url's host"),
					);
			}

			url_dialog.force_close();
			copy_link_button.set_sensitive(true);
		}
	));

	toggle_sidebar.clone().connect_clicked(move |_| {
		if osv.is_collapsed() {
			toggle_sidebar.set_active(false);
			osv.set_show_sidebar(true);
		} else {
			if toggle_sidebar.is_active() {
				frame.set_margin_start(0);
			} else {
				frame.set_margin_start(10);
			}

			osv.set_show_sidebar(toggle_sidebar.is_active());
		}
	});

	url_button.connect_clicked(clone!(
		#[strong]
		url_dialog,
		#[strong]
		web_view,
		#[strong]
		url_bar,
		#[strong]
		window,
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
		url_dialog,
		move |_| {
			url_bar.activate();
			url_dialog.close();
		}
	));

	copy_link_button.connect_clicked(clone!(
		#[strong]
		toast_overlay,
		#[strong]
		web_view,
		move |_| {
			let url = Some(web_view.uri().expect("Couldn't get URL"));

			gdk::Display::default()
				.unwrap()
				.clipboard()
				.set_text(url.expect("Couldn't get URL").as_str());
			toast_overlay
				.add_toast(adw::Toast::new(gettext("Link copied").as_str()));
		}
	));

	overview.connect_create_tab(clone!(
		#[strong]
		view,
		#[strong]
		window,
		move |_| {
			let web_view = webkit::WebView::new();

			web_view.connect_load_changed(clone!(
				#[strong]
				view,
				#[strong]
				url_button,
				#[strong]
				tabs,
				move |_, load_event| {
					match load_event {
						webkit::LoadEvent::Started => {
							view.selected_page()
								.expect("Couldn't get tab page")
								.set_loading(true);
						}
						webkit::LoadEvent::Finished => {
							view.selected_page()
								.expect("Couldn't get tab page")
								.set_loading(false);
							tabs.set_model(Some(&view.pages()));

							let tab_page = view
								.selected_page()
								.expect("Couldn't get tab page")
								.child();
							let web_view =
								tab_page.downcast_ref::<WebView>().unwrap();

							// see https://todo.sr.ht/~shrimple/ouch/1
							if web_view.title() != None {
								let url = Url::parse(
									web_view
										.uri()
										.expect("Couldn't get web view's url")
										.as_str(),
								);

								let _url = url.clone();
								if url.expect("Couldn't get URL").scheme()
									== "file"
								{
									url_button
										.child()
										.expect("Couldn't get command palette toggle's label")
										.downcast_ref::<gtk::Label>()
										.unwrap()
										.set_label(_url.expect("Couldn't get url").path());
								} else {
									url_button
										.child()
										.expect("Couldn't get command palette toggle's label")
										.downcast_ref::<gtk::Label>()
										.unwrap()
										.set_label(
											_url.expect("Couldn't get url")
												.host_str()
												.expect("Couldn't get url's host"),
										);
								}

								view.selected_page()
									.expect("Couldn't get tab page")
									.set_title(
										web_view
											.title()
											.expect("Couldn't get title")
											.as_str(),
									);

								view.selected_page()
									.expect("Couldn't get tab page")
									.set_keyword(
										web_view
											.uri()
											.expect(
												"Couldn't get web view's url",
											)
											.as_str(),
									);
							}
						}
						_ => (),
					}
				}
			));

			web_view.connect_load_failed(|web_view, _, fail_url, error| {
				if !error.matches(NetworkError::Cancelled) {
					let content = error_page(error.message());
					web_view.load_alternate_html(&content, fail_url, None);
				}
				false
			});

			web_view.connect_script_dialog(clone!(
				#[strong]
				toast_overlay,
				#[strong]
				window,
				move |web_view, web_dialog| {
					match web_dialog.dialog_type() {
						webkit::ScriptDialogType::Alert => {
							let url = Url::parse(
								web_view
									.uri()
									.expect("Couldn't get web view's url")
									.as_str(),
							);

							let dialog = adw::AlertDialog::new(
								Some(
									Some(format!(
										"{} says",
										url.expect("Couldn't get url")
											.host_str()
											.expect("Couldn't get url's host"),
									))
									.expect("Couldn't get dialog header")
									.as_str(),
								),
								Some(&web_dialog.message().expect(
									"Could not get script dialog message",
								)),
							);
							dialog.add_response("default", "OK");
							dialog.set_response_appearance(
								"default",
								adw::ResponseAppearance::Suggested,
							);
							dialog.present(Some(&window));

							true
						}
						_ => {
							toast_overlay.add_toast(adw::Toast::new(
								"This script dialog type is invalid",
							));
							true
						}
					}
				}
			));

			url_dialog.present(Some(&window));

			view.append(&web_view)
		}
	));

	let _ = lua::load(include_str!("plugins/usermods/main.lua"), window.clone().into());
	let _ = lua::load(include_str!("plugins/vblock/main.lua"), window.clone().into());	
}

fn error_page(msg: &str) -> String {
	let error_header = gettext("There was a problem loading this website");

	format!(
		r#"
		<!doctype html>
			<html>
				<head>
					<title>{error_header}</title>
					<style>
						* {{ margin: 0; }}

	   					body {{
							position: absolute;
							font-family: system-ui;
		   					margin-left: 20vh;
							margin-right: 20vh;
							top: 40%;
							transform: translateY(-40%);
						}}

						svg {{ margin-bottom: 0.5rem; }}

						@media (prefers-color-scheme: light) {{
							body {{
								background-color: #fff;
								color: #000;
							}}
						}}

						@media (prefers-color-scheme: dark) {{
							body {{
								background-color: #000;
								color: #fff;
							}}
						}}
					</style>
				</head>
				<body>
					<svg height="64px" viewBox="0 0 16 16" width="64px" xmlns="http://www.w3.org/2000/svg">
						<path d="m 3 0 c -1.660156 0 -3 1.339844 -3 3 v 7 c 0 1.660156 1.339844 3 3 3 h 10 c 1.660156 0 3 -1.339844 3 -3 v -7 c 0 -1.660156 -1.339844 -3 -3 -3 z m 0 2 h 10 c 0.554688 0 1 0.445312 1 1 v 7 c 0 0.554688 -0.445312 1 -1 1 h -10 c -0.554688 0 -1 -0.445312 -1 -1 v -7 c 0 -0.554688 0.445312 -1 1 -1 z m 3 2 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m 4 0 c -0.550781 0 -1 0.449219 -1 1 s 0.449219 1 1 1 s 1 -0.449219 1 -1 s -0.449219 -1 -1 -1 z m -2 3 c -1.429688 0 -2.75 0.761719 -3.464844 2 c -0.136718 0.238281 -0.054687 0.546875 0.183594 0.683594 c 0.238281 0.136718 0.546875 0.054687 0.683594 -0.183594 c 0.535156 -0.929688 1.523437 -1.5 2.597656 -1.5 s 2.0625 0.570312 2.597656 1.5 c 0.136719 0.238281 0.445313 0.320312 0.683594 0.183594 c 0.238281 -0.136719 0.320312 -0.445313 0.183594 -0.683594 c -0.714844 -1.238281 -2.035156 -2 -3.464844 -2 z m -3 7 c -1.105469 0 -2 0.894531 -2 2 h 10 c 0 -1.105469 -0.894531 -2 -2 -2 z m 0 0" fill="\#3d3846"/>
					</svg>
					<h3>{error_header}</h3>
					<small>{msg}</small>
				</body>
			</html>"#
	)
}
