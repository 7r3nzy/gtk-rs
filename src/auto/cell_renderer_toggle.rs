// This file was generated by gir (9c3dda3) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererToggle(Object<ffi::GtkCellRendererToggle>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_toggle_get_type(),
    }
}

impl CellRendererToggle {
    pub fn new() -> CellRendererToggle {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_toggle_new()).downcast_unchecked()
        }
    }

    pub fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_activatable(self.to_glib_none().0))
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_active(self.to_glib_none().0))
        }
    }

    pub fn get_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_toggle_get_radio(self.to_glib_none().0))
        }
    }

    pub fn set_activatable(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_activatable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_active(&self, setting: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_active(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_radio(&self, radio: bool) {
        unsafe {
            ffi::gtk_cell_renderer_toggle_set_radio(self.to_glib_none().0, radio.to_glib());
        }
    }

}
