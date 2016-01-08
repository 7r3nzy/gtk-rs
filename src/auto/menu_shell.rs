// This file was generated by gir (9c3dda3) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuShell(Object<ffi::GtkMenuShell>): Widget, Container, Buildable;

    match fn {
        get_type => || ffi::gtk_menu_shell_get_type(),
    }
}

pub trait MenuShellExt {
    fn activate_item<T: Upcast<Widget>>(&self, menu_item: &T, force_deactivate: bool);
    fn append<T: Upcast<MenuItem>>(&self, child: &T);
    //#[cfg(gtk_3_6)]
    //fn bind_model<T: Upcast</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>, with_separators: bool);
    fn cancel(&self);
    fn deactivate(&self);
    fn deselect(&self);
    fn get_parent_shell(&self) -> Option<Widget>;
    fn get_selected_item(&self) -> Option<Widget>;
    fn get_take_focus(&self) -> bool;
    fn insert<T: Upcast<Widget>>(&self, child: &T, position: i32);
    fn prepend<T: Upcast<Widget>>(&self, child: &T);
    fn select_first(&self, search_sensitive: bool);
    fn select_item<T: Upcast<Widget>>(&self, menu_item: &T);
    fn set_take_focus(&self, take_focus: bool);
}

impl<O: Upcast<MenuShell>> MenuShellExt for O {
    fn activate_item<T: Upcast<Widget>>(&self, menu_item: &T, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(self.to_glib_none().0, menu_item.to_glib_none().0, force_deactivate.to_glib());
        }
    }

    fn append<T: Upcast<MenuItem>>(&self, child: &T) {
        unsafe {
            ffi::gtk_menu_shell_append(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    //#[cfg(gtk_3_6)]
    //fn bind_model<T: Upcast</*Ignored*/gio::MenuModel>>(&self, model: Option<&T>, action_namespace: Option<&str>, with_separators: bool) {
    //    unsafe { TODO: call ffi::gtk_menu_shell_bind_model() }
    //}

    fn cancel(&self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(self.to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(self.to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(self.to_glib_none().0);
        }
    }

    fn get_parent_shell(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_parent_shell(self.to_glib_none().0))
        }
    }

    fn get_selected_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_selected_item(self.to_glib_none().0))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_shell_get_take_focus(self.to_glib_none().0))
        }
    }

    fn insert<T: Upcast<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    fn prepend<T: Upcast<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_menu_shell_prepend(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(self.to_glib_none().0, search_sensitive.to_glib());
        }
    }

    fn select_item<T: Upcast<Widget>>(&self, menu_item: &T) {
        unsafe {
            ffi::gtk_menu_shell_select_item(self.to_glib_none().0, menu_item.to_glib_none().0);
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(self.to_glib_none().0, take_focus.to_glib());
        }
    }

}
