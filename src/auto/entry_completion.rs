// This file was generated by gir (9c3dda3) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use TreeModel;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct EntryCompletion(Object<ffi::GtkEntryCompletion>): Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new())
        }
    }

    pub fn new_with_area<T: Upcast<CellArea>>(area: &T) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_new_with_area(area.to_glib_none().0))
        }
    }

    pub fn complete(&self) {
        unsafe {
            ffi::gtk_entry_completion_complete(self.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_4)]
    pub fn compute_prefix(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_entry_completion_compute_prefix(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn delete_action(&self, index_: i32) {
        unsafe {
            ffi::gtk_entry_completion_delete_action(self.to_glib_none().0, index_);
        }
    }

    pub fn get_completion_prefix(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_completion_prefix(self.to_glib_none().0))
        }
    }

    pub fn get_entry(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_entry(self.to_glib_none().0))
        }
    }

    pub fn get_inline_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_completion(self.to_glib_none().0))
        }
    }

    pub fn get_inline_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_inline_selection(self.to_glib_none().0))
        }
    }

    pub fn get_minimum_key_length(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_minimum_key_length(self.to_glib_none().0)
        }
    }

    pub fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(ffi::gtk_entry_completion_get_model(self.to_glib_none().0))
        }
    }

    pub fn get_popup_completion(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_completion(self.to_glib_none().0))
        }
    }

    pub fn get_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_set_width(self.to_glib_none().0))
        }
    }

    pub fn get_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_entry_completion_get_popup_single_match(self.to_glib_none().0))
        }
    }

    pub fn get_text_column(&self) -> i32 {
        unsafe {
            ffi::gtk_entry_completion_get_text_column(self.to_glib_none().0)
        }
    }

    pub fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_markup(self.to_glib_none().0, index_, markup.to_glib_none().0);
        }
    }

    pub fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            ffi::gtk_entry_completion_insert_action_text(self.to_glib_none().0, index_, text.to_glib_none().0);
        }
    }

    pub fn insert_prefix(&self) {
        unsafe {
            ffi::gtk_entry_completion_insert_prefix(self.to_glib_none().0);
        }
    }

    pub fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_completion(self.to_glib_none().0, inline_completion.to_glib());
        }
    }

    pub fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_inline_selection(self.to_glib_none().0, inline_selection.to_glib());
        }
    }

    //pub fn set_match_func(&self, func: /*Unknown conversion*/Unknown rust type: "EntryCompletionMatchFunc", func_data: Fundamental: Pointer, func_notify: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_entry_completion_set_match_func() }
    //}

    pub fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_minimum_key_length(self.to_glib_none().0, length);
        }
    }

    pub fn set_model<T: Upcast<TreeModel>>(&self, model: Option<&T>) {
        unsafe {
            ffi::gtk_entry_completion_set_model(self.to_glib_none().0, model.to_glib_none().0);
        }
    }

    pub fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_completion(self.to_glib_none().0, popup_completion.to_glib());
        }
    }

    pub fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_set_width(self.to_glib_none().0, popup_set_width.to_glib());
        }
    }

    pub fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            ffi::gtk_entry_completion_set_popup_single_match(self.to_glib_none().0, popup_single_match.to_glib());
        }
    }

    pub fn set_text_column(&self, column: i32) {
        unsafe {
            ffi::gtk_entry_completion_set_text_column(self.to_glib_none().0, column);
        }
    }

}
