/* src/lua.rs
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
use mlua::prelude::*;
use std::sync::Arc;
#[path = "consts.rs"]
mod consts;

pub fn load(src: &str, window: Arc<adw::ApplicationWindow>) -> LuaResult<()> {
	let lua = Lua::new();
	let table = lua.create_table()?;
	let win = lua.create_table()?;

	let window_clone = window.clone();
	win.set(
		"dialog",
		lua.create_function(
			move |_: &Lua, (title, content): (String, String)| {
				let dialog =
					adw::AlertDialog::new(Some(&title), Some(&content));

				dialog.add_response("default", "OK");
				dialog.set_response_appearance(
					"default",
					adw::ResponseAppearance::Suggested,
				);
				dialog.present(Some(window_clone.as_ref()));

				Ok(())
			},
		)?,
	)?;

	table.set("VERSION", consts::VERSION)?;
	table.set("window", win)?;

	lua.globals().set("ouch", table)?;
	lua.globals().set(
		"print",
		lua.create_function(|_: &Lua, text: String| {
			// This **SHOULD** have the same functionality as the
			// standard Lua `print` function.
			//
			// This function is intended to be a replacement for
			// the standard Lua `print` function, but diffrentiates
			// it's output from Ouch Browser's debug outputs.
			println!("[PLUGIN] {}", text);
			Ok(())
		})?,
	)?;

	match lua.load(src).exec() {
		Err(error) => {
			println!("[PLUGIN|ERROR] {}", error.to_string());

			let textview = gtk::TextView::new();
			textview.buffer().set_text(&error.to_string().as_str());
			textview.set_editable(false);
			textview.set_cursor_visible(false);
			textview.set_monospace(true);
			textview.set_wrap_mode(gtk::WrapMode::Word);
			textview.add_css_class("card");

			let dialog =
				adw::AlertDialog::new(
                    Some("There was a problem loading a plugin"),
                    Some("If you don't know where this is coming from, please report this to Ouch Browser's bug tracker.")
                );

			dialog.add_response("default", "Ignore");
			dialog.set_response_appearance(
				"default",
				adw::ResponseAppearance::Destructive,
			);
			dialog.set_extra_child(Some(&textview));
			dialog.present(Some(window.clone().as_ref()));

			return Err(error);
		}
		Ok(()) => return Ok(()),
	}
}
