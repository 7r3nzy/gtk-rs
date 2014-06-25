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

// FIXME: @jeremyletang implements the new index traits when it's available

use std::mem;
use libc::c_void;

use glib::ffi;
use glib::GlibContainer;

pub struct SList<T> {
    pointer: *ffi::C_GSList
}

impl<T> SList<T> {
    pub fn new() -> SList<T> {
        SList {
            pointer: ::std::ptr::null()
        }
    }

    pub fn append(&mut self, data: T) {
        unsafe {
            ffi::g_slist_append(self.pointer, mem::transmute(box data));
        }
    }

    pub fn prepend(&mut self, data: T) {
        unsafe {
            ffi::g_slist_prepend(self.pointer, mem::transmute(box data));
        }
    }

    pub fn nth(&self, n: u32) -> &T {
        unsafe {
            mem::transmute::<*c_void, &T>(ffi::g_slist_nth_data(self.pointer, n))
        }
    }

    pub fn last(&self) -> &T {
        let elem = unsafe { ffi::g_slist_last(self.pointer) };
        unsafe { mem::transmute::<*c_void, &T>((*elem).data)}
    }

    pub fn insert(&mut self, data: T, position: i32) {
        unsafe {
            ffi::g_slist_insert(self.pointer, mem::transmute(box data), position);
        }
    }

    pub fn concat(&mut self, list: SList<T>) {
        unsafe {
            ffi::g_slist_concat(self.pointer, list.unwrap());
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::g_slist_reverse(self.pointer);
        }
    }
}

impl<T> Collection for SList<T> {
    fn len(&self) -> uint {
        unsafe { ffi::g_slist_length(self.pointer) as uint }
    }
}

impl<T> FromIterator<T> for SList<T> {
    fn from_iter<It: Iterator<T>>(it: It) -> SList<T> {
        let mut new_list = SList::new();
        new_list.extend(it);
        new_list
    }
}

impl<T> Extendable<T> for SList<T> {
    fn extend<It: Iterator<T>>(&mut self, mut it: It) {
        for elem in it {
            self.append(elem);
        }
    }
}

impl<T> Clone for SList<T> {
    fn clone(&self) -> SList<T> {
        unsafe {
            GlibContainer::wrap(ffi::g_slist_copy(self.pointer))
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for SList<T> {
    fn drop(&mut self) {
        unsafe { ffi::g_slist_free(self.pointer); }
    }
}

impl<T> GlibContainer<*ffi::C_GSList> for SList<T> {
    fn wrap(pointer: *ffi::C_GSList) -> SList<T> {
        SList {
            pointer: pointer
        }
    }

    fn unwrap(&self) -> *ffi::C_GSList {
        self.pointer
    }
}