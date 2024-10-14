/* src/ui.rs
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
use gtk::{
	glib, glib::subclass::InitializingObject, prelude::*, subclass::prelude::*,
	CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/site/srht/shrimple/ouch/ui/window.ui")]
pub struct Window {
	#[template_child]
	pub window: TemplateChild<adw::ApplicationWindow>,
	#[template_child]
	pub osv: TemplateChild<adw::OverlaySplitView>,
	#[template_child]
	pub view: TemplateChild<adw::TabView>,
	#[template_child]
	pub url_dialog: TemplateChild<adw::Dialog>,
	#[template_child]
	pub url_button: TemplateChild<gtk::Button>,
	#[template_child]
	pub toggle_sidebar: TemplateChild<gtk::ToggleButton>,
	#[template_child]
	pub url_bar: TemplateChild<gtk::ToggleButton>,
	#[template_child]
	pub url_bar_button: TemplateChild<gtk::Button>,
	#[template_child]
	pub toast_overlay: TemplateChild<adw::ToastOverlay>,
	#[template_child]
	pub copy_link_button: TemplateChild<gtk::Button>,
	#[template_child]
	pub overview: TemplateChild<adw::TabOverview>,
	#[template_child]
	pub tabs: TemplateChild<gtk::ListView>,
	#[template_child]
	pub frame: TemplateChild<gtk::Frame>,
}
