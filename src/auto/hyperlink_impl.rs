// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Hyperlink;
use atk_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct HyperlinkImpl(Interface<atk_sys::AtkHyperlinkImpl>);

    match fn {
        get_type => || atk_sys::atk_hyperlink_impl_get_type(),
    }
}

pub const NONE_HYPERLINK_IMPL: Option<&HyperlinkImpl> = None;

pub trait HyperlinkImplExt: 'static {
    fn get_hyperlink(&self) -> Option<Hyperlink>;
}

impl<O: IsA<HyperlinkImpl>> HyperlinkImplExt for O {
    fn get_hyperlink(&self) -> Option<Hyperlink> {
        unsafe {
            from_glib_full(atk_sys::atk_hyperlink_impl_get_hyperlink(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for HyperlinkImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HyperlinkImpl")
    }
}
