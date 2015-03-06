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

use gdk::ffi;
use glib::translate::{FromGlibPtr, ToGlibPtr, ToTmp};
use libc::{c_char};

#[derive(Copy)]
pub struct Atom {
    pointer: ffi::C_GdkAtom
}

impl Atom {
    /*pub fn new() -> Atom {
        Atom {
            pointer: ::std::ptr::mut_null()
        }
    }*/

    pub fn intern(atom_name: &str, only_if_exists: bool) -> Option<Atom> {
        let tmp = unsafe {
            let mut tmp_atom_name = atom_name.to_tmp_for_borrow();
            ffi::gdk_atom_intern(tmp_atom_name.to_glib_ptr(), ::glib::to_gboolean(only_if_exists))
        };

        if tmp.is_null() {
            None
        } else {
            Some(Atom {
                pointer: tmp
            })
        }
    }

    pub fn intern_static_string(atom_name: &str) -> Option<Atom> {
        let tmp = unsafe {
            let mut tmp_atom_name = atom_name.to_tmp_for_borrow();
            ffi::gdk_atom_intern_static_string(tmp_atom_name.to_glib_ptr())
        };

        if tmp.is_null() {
            None
        } else {
            Some(Atom {
                pointer: tmp
            })
        }
    }

    pub fn name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::take(
                ffi::gdk_atom_name(self.pointer) as *const c_char)
        }
    }

    // I can't use the gObject macros for this object
    pub fn wrap_pointer(pointer: ffi::C_GdkAtom) -> Atom {
        Atom {
            pointer: pointer
        }
    }

    pub fn unwrap_pointer(&self) -> ffi::C_GdkAtom {
        self.pointer
    }
}