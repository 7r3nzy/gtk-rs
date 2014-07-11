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

#![allow(visible_private_types)]

use libc::c_int;
use gtk::ffi;
use gtk::signals::Signal;
use gtk::enums;
use gdk;
use gtk;
use glib;

pub trait Widget: ffi::FFIWidget {
    fn show_all(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_show_all(self.get_widget());
        }
    }

    fn show_now(&self) {
        unsafe { ffi::gtk_widget_show_now(self.get_widget()) }
    }

    fn hide(&self) {
        unsafe { ffi::gtk_widget_hide(self.get_widget()) }
    }

    fn map(&self) {
        unsafe { ffi::gtk_widget_map(self.get_widget()) }
    }

    fn unmap(&self) {
        unsafe { ffi::gtk_widget_unmap(self.get_widget()) }
    }

    fn realize(&self) {
        unsafe { ffi::gtk_widget_realize(self.get_widget()) }
    }

    fn unrealize(&self) {
        unsafe { ffi::gtk_widget_unrealize(self.get_widget()) }
    }

    fn queue_draw(&self) {
        unsafe { ffi::gtk_widget_queue_draw(self.get_widget()) }
    }

    fn queue_resize(&self) {
        unsafe { ffi::gtk_widget_queue_resize(self.get_widget()) }
    }

    fn queue_resize_no_redraw(&self) {
        unsafe { ffi::gtk_widget_queue_resize_no_redraw(self.get_widget()) }
    }

    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_scale_factor(self.get_widget()) }
    }

    fn activate(&self) -> bool {
        match unsafe { ffi::gtk_widget_activate(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn reparent(&self, new_parent: &Widget) {
        unsafe { ffi::gtk_widget_reparent(self.get_widget(), new_parent.get_widget()) }
    }

    fn is_focus(&self) -> bool {
        match unsafe { ffi::gtk_widget_is_focus(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn grab_focus(&self) {
        unsafe { ffi::gtk_widget_grab_focus(self.get_widget()) }
    }

    fn grab_default(&self) {
        unsafe { ffi::gtk_widget_grab_default(self.get_widget()) }
    }

    fn set_name(&self, name: &str) {
        name.with_c_str(|c_str| {
            unsafe { ffi::gtk_widget_set_name(self.get_widget(), c_str) }
        })
    }

    fn get_name(&self) -> Option<String> {
        let tmp = unsafe { ffi::gtk_widget_get_name(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::std::str::raw::from_c_str(tmp)) }
        }
    }

    fn set_state(&self, state: enums::StateType) {
        unsafe { ffi::gtk_widget_set_state(self.get_widget(), state) }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe { ffi::gtk_widget_set_sensitive(self.get_widget(), match sensitive {
            true => ffi::Gtrue,
            false => ffi::Gfalse
        }) }
    }

    fn set_parent(&self, parent: &Widget) {
        unsafe { ffi::gtk_widget_set_parent(self.get_widget(), parent.get_widget()) }
    }

    /*fn set_parent_window(&self, parent: &Widget) {
        unsafe { gtk_widget_set_parent_window(self.get_widget(), parent.get_widget()) }
    }*/

    fn get_toplevel(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_toplevel(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn get_ancestor(&self, widget_type: i32) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_ancestor(self.get_widget(), widget_type) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn is_ancestor(&self, ancestor: &Widget) -> bool {
        match unsafe { ffi::gtk_widget_is_ancestor(self.get_widget(), ancestor.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn hide_on_delete(&self) -> bool {
        match unsafe { ffi::gtk_widget_hide_on_delete(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_direction(&self, dir: enums::TextDirection) {
        unsafe { ffi::gtk_widget_set_direction(self.get_widget(), dir) }
    }

    fn get_direction(&self) -> enums::TextDirection {
        unsafe { ffi::gtk_widget_get_direction(self.get_widget()) }
    }

    fn set_default_direction(dir: enums::TextDirection) {
        unsafe { ffi::gtk_widget_set_default_direction(dir) }
    }

    fn get_default_direction() -> enums::TextDirection {
        unsafe { ffi::gtk_widget_get_default_direction() }
    }

    fn in_destruction(&self) -> bool {
        match unsafe { ffi::gtk_widget_in_destruction(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn unparent(&self) {
        unsafe { ffi::gtk_widget_unparent(self.get_widget()) }
    }

    fn translate_coordinates(&self, dest_widget: &Widget, src_x: i32, src_y: i32) -> Option<(i32, i32)> {
        let mut dest_x = 0i32;
        let mut dest_y = 0i32;

        match unsafe { ffi::gtk_widget_translate_coordinates(self.get_widget(), dest_widget.get_widget(), src_x, src_y, &mut dest_x, &mut dest_y) } {
            ffi::Gtrue => Some((dest_x, dest_y)),
            _ => None
        }
    }

    fn override_background_color(&self, state: gtk::StateFlags, color: &gdk::RGBA) {
        unsafe { ffi::gtk_widget_override_background_color(self.get_widget(), state, color) }
    }

    fn override_color(&self, state: gtk::StateFlags, color: &gdk::RGBA) {
        unsafe { ffi::gtk_widget_override_color(self.get_widget(), state, color) }
    }

    fn override_symbolic_color(&self, name: &str, color: &gdk::RGBA) {
        name.with_c_str(|c_str| {
            unsafe { ffi::gtk_widget_override_symbolic_color(self.get_widget(), c_str, color) }
        });
    }

    fn override_cursor(&self, cursor: &gdk::RGBA, secondary_cursor: &gdk::RGBA) {
        unsafe { ffi::gtk_widget_override_cursor(self.get_widget(), cursor, secondary_cursor) }
    }

    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_queue_draw_area(self.get_widget(), x, y, width, height) }
    }

    fn set_app_paintable(&self, app_paintable: bool) {
        unsafe { ffi::gtk_widget_set_app_paintable(self.get_widget(), match app_paintable {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) }
    }

    fn set_double_buffered(&self, double_buffered: bool) {
        unsafe { ffi::gtk_widget_set_double_buffered(self.get_widget(), match double_buffered {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) }
    }

    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool) {
        unsafe { ffi::gtk_widget_set_redraw_on_allocate(self.get_widget(), match redraw_on_allocate {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) }
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        match unsafe { ffi::gtk_widget_mnemonic_activate(self.get_widget(), match group_cycling {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    /*fn send_expose(&self, event: &mut gdk::Event) -> i32 {
        unsafe { ffi::gtk_widget_send_expose(self.get_widget(), event) }
    }

    fn send_focus_change(&self, event: &mut gdk::Event) -> bool {
        match unsafe { ffi::gtk_widget_send_expose(self.get_widget(), event) } {
            Gtrue => true,
            _ => false
        }
    }*/

    fn child_focus(&self, direction: gtk::DirectionType) -> bool {
        match unsafe { ffi::gtk_widget_child_focus(self.get_widget(), direction) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_child_visible(&self) -> bool {
        match unsafe { ffi::gtk_widget_get_child_visible(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_parent(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_parent(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn has_screen(&self) -> bool {
        match unsafe { ffi::gtk_widget_has_screen(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn get_size_request(&self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;

        unsafe { ffi::gtk_widget_get_size_request(self.get_widget(), &mut width, &mut height) };
        (width, height)
    }

    fn set_child_visible(&self, is_visible: bool) {
        unsafe { ffi::gtk_widget_set_child_visible(self.get_widget(), match is_visible {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) }
    }

    fn set_size_request(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_set_size_request(self.get_widget(), width, height) }
    }

    fn set_no_show_all(&self, no_show_all: bool) {
        unsafe { ffi::gtk_widget_set_no_show_all(self.get_widget(), match no_show_all {
                true => ffi::Gtrue,
                false => ffi::Gfalse
            }) }
    }

    fn get_no_show_all(&self) -> bool {
        match unsafe { ffi::gtk_widget_get_no_show_all(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn list_mnemonic_labels(&self) -> glib::List<Self> {
        let tmp = unsafe { ffi::gtk_widget_list_mnemonic_labels(self.get_widget()) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Self> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(ffi::FFIWidget::wrap(*it));
            }
            tmp_vec
        }
    }

    fn add_mnemonic_label(&self, label: &Widget) {
        unsafe { ffi::gtk_widget_add_mnemonic_label(self.get_widget(), label.get_widget()) }
    }

    fn remove_mnemonic_label(&self, label: &Widget) {
        unsafe { ffi::gtk_widget_remove_mnemonic_label(self.get_widget(), label.get_widget()) }
    }

    fn is_composited(&self) -> bool {
        match unsafe { ffi::gtk_widget_is_composited(self.get_widget()) } {
            ffi::Gtrue => true,
            _ => false
        }
    }

    fn set_margin_right(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_right(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_left(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_left(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_top(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_bottom(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.get_widget(), margin as c_int)
        }
    }

    fn get_margin_right(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_right(self.get_widget()) as i32
        }
    }

    fn get_margin_left(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_left(self.get_widget()) as i32
        }
    }

    fn get_margin_top(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.get_widget()) as i32
        }
    }

    fn get_margin_bottom(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.get_widget()) as i32
        }
    }

    fn connect<'a>(&self, signal: Box<Signal<'a>>) -> () {
        use std::mem::transmute;

        unsafe {
            let signal_name     = signal.get_signal_name();
            let trampoline      = signal.get_trampoline();

            let user_data_ptr   = transmute(box signal);

            signal_name.replace("_", "-").with_c_str(|signal_name| {
                ffi::glue_signal_connect(
                    self.get_widget(),
                    signal_name,
                    Some(trampoline),
                    user_data_ptr
                )
            });
        }
    }

    fn get_allocated_width(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.get_widget()) as i32
        }
    }

    fn get_allocated_height(&self) -> i32{
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.get_widget()) as i32
        }
    }
}