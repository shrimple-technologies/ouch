/* main.rs
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

use gtk::{/* gio, */ glib, prelude::*};
mod css;
mod window;
use gettextrs::*;

fn main() -> glib::ExitCode {
	textdomain("ouch").expect("Couldn't get gettext domain");
	bind_textdomain_codeset("ouch", "UTF-8").expect("Couldn't get gettext domain");

	let app = adw::Application::builder()
		.application_id("site.srht.shrimple.ouch")
		// .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
		.build();

	app.connect_startup(css::init);
	app.connect_activate(window::init);

	app.set_accels_for_action("window.close", &["<Ctrl>Q"]);
	app.set_accels_for_action("win.cmd", &["<Ctrl>L", "<Alt>D"]);
	app.set_accels_for_action("win.show-preferences", &["<Ctrl>comma"]);
	app.set_accels_for_action("win.copy-link", &["<Ctrl><Shift>C"]);
	app.set_accels_for_action("win.zoom-in", &["<Ctrl>plus"]);

	app.run()
}
