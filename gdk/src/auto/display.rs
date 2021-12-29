// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AppLaunchContext;
use crate::Atom;
use crate::Device;
use crate::DeviceManager;
use crate::Event;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use crate::Monitor;
use crate::Screen;
#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
use crate::Seat;
use crate::Window;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkDisplay")]
    pub struct Display(Object<ffi::GdkDisplay>);

    match fn {
        type_ => || ffi::gdk_display_get_type(),
    }
}

impl Display {
    #[doc(alias = "gdk_display_beep")]
    pub fn beep(&self) {
        unsafe {
            ffi::gdk_display_beep(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_close")]
    pub fn close(&self) {
        unsafe {
            ffi::gdk_display_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_device_is_grabbed")]
    pub fn device_is_grabbed(&self, device: &Device) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_device_is_grabbed(
                self.to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_flush")]
    pub fn flush(&self) {
        unsafe {
            ffi::gdk_display_flush(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_get_app_launch_context")]
    #[doc(alias = "get_app_launch_context")]
    pub fn app_launch_context(&self) -> Option<AppLaunchContext> {
        unsafe {
            from_glib_full(ffi::gdk_display_get_app_launch_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_get_default_cursor_size")]
    #[doc(alias = "get_default_cursor_size")]
    pub fn default_cursor_size(&self) -> u32 {
        unsafe { ffi::gdk_display_get_default_cursor_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_display_get_default_group")]
    #[doc(alias = "get_default_group")]
    pub fn default_group(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_display_get_default_group(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_get_default_screen")]
    #[doc(alias = "get_default_screen")]
    pub fn default_screen(&self) -> Screen {
        unsafe { from_glib_none(ffi::gdk_display_get_default_screen(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "gdk_display_get_default_seat")]
    #[doc(alias = "get_default_seat")]
    pub fn default_seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_display_get_default_seat(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gdk_display_get_device_manager")]
    #[doc(alias = "get_device_manager")]
    pub fn device_manager(&self) -> Option<DeviceManager> {
        unsafe { from_glib_none(ffi::gdk_display_get_device_manager(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_get_event")]
    #[doc(alias = "get_event")]
    pub fn event(&self) -> Option<Event> {
        unsafe { from_glib_full(ffi::gdk_display_get_event(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_get_maximal_cursor_size")]
    #[doc(alias = "get_maximal_cursor_size")]
    pub fn maximal_cursor_size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gdk_display_get_maximal_cursor_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_display_get_monitor")]
    #[doc(alias = "get_monitor")]
    pub fn monitor(&self, monitor_num: i32) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_monitor(
                self.to_glib_none().0,
                monitor_num,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_display_get_monitor_at_point")]
    #[doc(alias = "get_monitor_at_point")]
    pub fn monitor_at_point(&self, x: i32, y: i32) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_monitor_at_point(
                self.to_glib_none().0,
                x,
                y,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_display_get_monitor_at_window")]
    #[doc(alias = "get_monitor_at_window")]
    pub fn monitor_at_window(&self, window: &Window) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::gdk_display_get_monitor_at_window(
                self.to_glib_none().0,
                window.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_display_get_n_monitors")]
    #[doc(alias = "get_n_monitors")]
    pub fn n_monitors(&self) -> i32 {
        unsafe { ffi::gdk_display_get_n_monitors(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_display_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gdk_display_get_name(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_display_get_primary_monitor")]
    #[doc(alias = "get_primary_monitor")]
    pub fn primary_monitor(&self) -> Option<Monitor> {
        unsafe { from_glib_none(ffi::gdk_display_get_primary_monitor(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gdk_display_get_screen")]
    #[doc(alias = "get_screen")]
    pub fn screen(&self, screen_num: i32) -> Screen {
        unsafe {
            from_glib_none(ffi::gdk_display_get_screen(
                self.to_glib_none().0,
                screen_num,
            ))
        }
    }

    #[doc(alias = "gdk_display_has_pending")]
    pub fn has_pending(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_has_pending(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_is_closed")]
    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_is_closed(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "gdk_display_list_seats")]
    pub fn list_seats(&self) -> Vec<Seat> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_display_list_seats(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_notify_startup_complete")]
    pub fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_display_notify_startup_complete(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_display_peek_event")]
    pub fn peek_event(&self) -> Option<Event> {
        unsafe { from_glib_full(ffi::gdk_display_peek_event(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_put_event")]
    pub fn put_event(&self, event: &Event) {
        unsafe {
            ffi::gdk_display_put_event(self.to_glib_none().0, event.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_request_selection_notification")]
    pub fn request_selection_notification(&self, selection: &Atom) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_request_selection_notification(
                self.to_glib_none().0,
                selection.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_set_double_click_distance")]
    pub fn set_double_click_distance(&self, distance: u32) {
        unsafe {
            ffi::gdk_display_set_double_click_distance(self.to_glib_none().0, distance);
        }
    }

    #[doc(alias = "gdk_display_set_double_click_time")]
    pub fn set_double_click_time(&self, msec: u32) {
        unsafe {
            ffi::gdk_display_set_double_click_time(self.to_glib_none().0, msec);
        }
    }

    #[doc(alias = "gdk_display_store_clipboard")]
    pub fn store_clipboard(&self, clipboard_window: &Window, time_: u32, targets: &[Atom]) {
        let n_targets = targets.len() as i32;
        unsafe {
            ffi::gdk_display_store_clipboard(
                self.to_glib_none().0,
                clipboard_window.to_glib_none().0,
                time_,
                targets.to_glib_none().0,
                n_targets,
            );
        }
    }

    #[doc(alias = "gdk_display_supports_clipboard_persistence")]
    pub fn supports_clipboard_persistence(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_clipboard_persistence(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_supports_cursor_alpha")]
    pub fn supports_cursor_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_cursor_alpha(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_supports_cursor_color")]
    pub fn supports_cursor_color(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_cursor_color(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_supports_input_shapes")]
    pub fn supports_input_shapes(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_input_shapes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_supports_selection_notification")]
    pub fn supports_selection_notification(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_supports_selection_notification(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_display_supports_shapes")]
    pub fn supports_shapes(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_shapes(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_display_sync")]
    pub fn sync(&self) {
        unsafe {
            ffi::gdk_display_sync(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_display_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_display_get_default()) }
    }

    #[doc(alias = "gdk_display_open")]
    pub fn open(display_name: &str) -> Option<Display> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_display_open(display_name.to_glib_none().0)) }
    }

    #[doc(alias = "closed")]
    pub fn connect_closed<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<F: Fn(&Display, bool) + 'static>(
            this: *mut ffi::GdkDisplay,
            is_error: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(is_error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "monitor-added")]
    pub fn connect_monitor_added<F: Fn(&Self, &Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn monitor_added_trampoline<F: Fn(&Display, &Monitor) + 'static>(
            this: *mut ffi::GdkDisplay,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"monitor-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    monitor_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "monitor-removed")]
    pub fn connect_monitor_removed<F: Fn(&Self, &Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn monitor_removed_trampoline<F: Fn(&Display, &Monitor) + 'static>(
            this: *mut ffi::GdkDisplay,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"monitor-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    monitor_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "opened")]
    pub fn connect_opened<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn opened_trampoline<F: Fn(&Display) + 'static>(
            this: *mut ffi::GdkDisplay,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"opened\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    opened_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "seat-added")]
    pub fn connect_seat_added<F: Fn(&Self, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_added_trampoline<F: Fn(&Display, &Seat) + 'static>(
            this: *mut ffi::GdkDisplay,
            seat: *mut ffi::GdkSeat,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(seat))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seat-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seat_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "seat-removed")]
    pub fn connect_seat_removed<F: Fn(&Self, &Seat) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn seat_removed_trampoline<F: Fn(&Display, &Seat) + 'static>(
            this: *mut ffi::GdkDisplay,
            seat: *mut ffi::GdkSeat,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(seat))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seat-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seat_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}
