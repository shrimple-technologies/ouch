/* lua.rs
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

use mlua::prelude::*;

pub fn load(src: &str) -> LuaResult<()> {
	let lua = Lua::new();	
	let table = lua.create_table()?;

	table.set("version", "0.5.0-rc.1")?;

	lua.globals().set("ouch", table)?;
	lua.globals().set(
		"print"
		lua.create_function(|_: &Lua, text: String| {
			// This **SHOULD** have the same functionality as the
			// standard Lua `print` function.
			//
			// This function is intended to be a replacement for
			// the standard Lua `print` function, but diffrentiates
			// it's output from Ouch Browser's debug outputs.
			println!("PLUGIN |  {}", text);
			Ok(())
		})?
	)?;

	lua.load(src).exec()?;

	Ok(())
}
