// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A ToolItem subclass that displays buttons

use std::ptr;

use glib::translate::{ToGlibPtr, ToTmp};
use gtk::{self, ffi};

/// ToolButton — A ToolItem subclass that displays buttons
struct_Widget!(ToolButton);

impl ToolButton {
    pub fn new<T: gtk::WidgetTrait>(icon_widget: Option<&T>, label: Option<&str>) -> Option<ToolButton> {
        let tmp_pointer = unsafe {
            let mut tmp_label = label.to_tmp_for_borrow();
            let icon_widget_ptr = match icon_widget {
                Some(i) => i.unwrap_widget(),
                None    => ptr::null_mut(),
            };
            ffi::gtk_tool_button_new(icon_widget_ptr, tmp_label.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, ToolButton)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<ToolButton> {
        let tmp_pointer = unsafe {
            let mut tmp_stock_id = stock_id.to_tmp_for_borrow();
            ffi::gtk_tool_button_new_from_stock(tmp_stock_id.to_glib_ptr())
        };
        check_pointer!(tmp_pointer, ToolButton)
    }
}

impl_drop!(ToolButton);
impl_TraitWidget!(ToolButton);

impl gtk::ContainerTrait for ToolButton {}
impl gtk::BinTrait for ToolButton {}
impl gtk::ToolItemTrait for ToolButton {}
impl gtk::ToolButtonTrait for ToolButton {}

impl_widget_events!(ToolButton);
impl_button_events!(ToolButton);
