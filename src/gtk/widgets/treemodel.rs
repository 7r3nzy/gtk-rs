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

use glib::ffi::GType;
use gtk::ffi;
use gtk;
use std::c_str::CString;
use std::string::raw::from_buf;

pub struct TreeModel {
    pointer: *mut ffi::C_GtkTreeModel
}

impl TreeModel {
    pub fn get_flags(&self) -> gtk::TreeModelFlags {
        unsafe { ffi::gtk_tree_model_get_flags(self.pointer) }
    }

    pub fn get_n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.pointer) }
    }

    pub fn get_column_type(&self, index_: i32) -> GType {
        unsafe { ffi::gtk_tree_model_get_column_type(self.pointer, index_) }
    }

    pub fn get_iter(&self, iter: &mut ffi::C_GtkTreeIter, path: &gtk::TreePath) -> bool {
        match unsafe { ffi::gtk_tree_model_get_iter(self.pointer, iter, path.get_pointer()) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_iter_from_string(&self, iter: &mut ffi::C_GtkTreeIter, path_string: &str) -> bool {
        path_string.with_c_str(|c_str| {
            match unsafe { ffi::gtk_tree_model_get_iter_from_string(self.pointer, iter, c_str) } {
                0 => false,
                _ => true
            }
        })
    }
    
    pub fn get_iter_first(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_get_iter_first(self.pointer, iter) } {
            0 => false,
            _ => true
        }
    }

    pub fn get_path(&self, iter: &mut ffi::C_GtkTreeIter) -> Option<gtk::TreePath> {
        let tmp_pointer = unsafe { ffi::gtk_tree_model_get_path(self.pointer, iter) };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(gtk::TreePath::wrap_pointer(tmp_pointer))
        }
    }

    pub fn iter_next(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_next(self.pointer, iter) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_previous(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_previous(self.pointer, iter) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_children(&self, iter: &mut ffi::C_GtkTreeIter, parent: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_children(self.pointer, iter, parent) } {
            0 => false,
            _ => true
        }
    }
    
    pub fn iter_has_child(&self, iter: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_has_child(self.pointer, iter) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_n_children(&self, iter: &mut ffi::C_GtkTreeIter) -> i32 {
        unsafe { ffi::gtk_tree_model_iter_n_children(self.pointer, iter) }
    }

    pub fn iter_nth_child(&self, iter: &mut ffi::C_GtkTreeIter, parent: &mut ffi::C_GtkTreeIter, n: i32) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_nth_child(self.pointer, iter, parent, n) } {
            0 => false,
            _ => true
        }
    }

    pub fn iter_parent(&self, iter: &mut ffi::C_GtkTreeIter, child: &mut ffi::C_GtkTreeIter) -> bool {
        match unsafe { ffi::gtk_tree_model_iter_parent(self.pointer, iter, child) } {
            0 => false,
            _ => true
        }
    }

    #[allow(unused_variable)]
    pub fn get_string_from_iter(&self, iter: &mut ffi::C_GtkTreeIter) -> String {
        let string = unsafe { ffi::gtk_tree_model_get_string_from_iter(self.pointer, iter) };

        if string.is_null() {
            String::new()
        } else {
            unsafe {
                // used to free the returned string
                let container = CString::new(string as *const i8, true);

                from_buf(string as *const u8)
            }
        }
    }

    pub fn row_changed(&self, path: &gtk::TreePath, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_tree_model_row_changed(self.pointer, path.get_pointer(), iter) }
    }

    pub fn row_inserted(&self, path: &gtk::TreePath, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_tree_model_row_inserted(self.pointer, path.get_pointer(), iter) }
    }

    pub fn row_has_child_toggled(&self, path: &gtk::TreePath, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_tree_model_row_has_child_toggled(self.pointer, path.get_pointer(), iter) }
    }

    pub fn row_deleted(&self, path: &gtk::TreePath) {
        unsafe { ffi::gtk_tree_model_row_deleted(self.pointer, path.get_pointer()) }
    }

    pub fn rows_reordered(&self, path: &gtk::TreePath, iter: &mut ffi::C_GtkTreeIter, new_order: &mut [i32]) {
        unsafe { ffi::gtk_tree_model_rows_reordered(self.pointer, path.get_pointer(), iter, new_order.as_mut_ptr()) }
    }

    pub fn ref_node(&self, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_tree_model_ref_node(self.pointer, iter) }
    }

    pub fn unref_node(&self, iter: &mut ffi::C_GtkTreeIter) {
        unsafe { ffi::gtk_tree_model_unref_node(self.pointer, iter) }
    }

    #[doc(hidden)]
    pub fn get_pointer(&self) -> *mut ffi::C_GtkTreeModel {
        self.pointer
    }

    #[doc(hidden)]
    pub fn wrap_pointer(c_treemodel: *mut ffi::C_GtkTreeModel) -> TreeModel {
        TreeModel {
            pointer: c_treemodel
        }
    }
}

impl Drop for TreeModel {
    fn drop(&mut self) {
        unsafe {
            ::glib::ffi::g_object_unref(self.pointer as *mut ::glib::ffi::C_GObject);
        }
    }
}

impl Clone for TreeModel {
    fn clone(&self) -> TreeModel {
        let pointer = unsafe {
            ::glib::ffi::g_object_ref(self.pointer as *mut ::glib::ffi::C_GObject)
        };

        TreeModel {
            pointer: pointer as *mut ffi::C_GtkTreeModel
        }
    }
}
