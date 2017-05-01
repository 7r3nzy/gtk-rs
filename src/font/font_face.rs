#[cfg(feature = "glib")]
use glib::translate::*;
use libc::c_char;
use ffi;
use std::ffi::{CString,CStr};

use ffi::enums::{
    FontType,
    FontWeight,
    FontSlant,
};

#[cfg(feature = "glib")]
glib_wrapper! {
    pub struct FontFace(Shared<ffi::cairo_font_face_t>);

    match fn {
        ref => |ptr| ffi::cairo_font_face_reference(ptr),
        unref => |ptr| ffi::cairo_font_face_destroy(ptr),
    }
}

#[cfg(not(feature = "glib"))]
pub struct FontFace(*mut ffi::cairo_font_face_t);

impl FontFace {
    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face: FontFace = unsafe {
            FontFace::from_raw_full(ffi::cairo_toy_font_face_create(CString::new(family).unwrap().as_ptr(), slant, weight))
        };
        font_face.ensure_status();
        font_face
    }

    #[cfg(feature = "glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_full(ptr)
    }

    #[cfg(not(feature = "glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr)
    }

    #[cfg(feature = "glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        from_glib_none(ptr)
    }

    #[cfg(not(feature = "glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_font_face_t) -> FontFace {
        assert!(!ptr.is_null());
        FontFace(ptr)
    }

    #[cfg(feature = "glib")]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.to_glib_none().0
    }

    #[cfg(not(feature = "glib"))]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_face_t {
        self.0
    }

    pub fn toy_get_family(&self) -> Option<String> {
        let family_name = unsafe {
            ffi::cairo_toy_font_face_get_family(self.to_raw_none())
        };
        to_optional_string(family_name)
    }

    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe {
            ffi::cairo_toy_font_face_get_slant(self.to_raw_none())
        }
    }

    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe {
            ffi::cairo_toy_font_face_get_weight(self.to_raw_none())
        }
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_face_status(self.to_raw_none())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_font_face_get_type(self.to_raw_none())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_font_face_get_reference_count(self.to_raw_none()) as usize
        }
    }
}

fn to_optional_string(str:*const c_char) -> Option<String> {
    if str.is_null() { None }
    else {
        let str = unsafe { CStr::from_ptr(str).to_bytes() };
        Some(String::from_utf8_lossy(str).into_owned())
    }
}

