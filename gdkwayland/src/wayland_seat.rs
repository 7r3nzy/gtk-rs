// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::ToGlibPtr;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::sys::client::wl_proxy;
use wayland_client::Proxy;

glib::wrapper! {
    #[doc(alias = "GdkWaylandSeat")]
    pub struct WaylandSeat(Object<ffi::GdkWaylandSeat>) @extends gdk::Seat;

    match fn {
        type_ => || ffi::gdk_wayland_seat_get_type(),
    }
}

impl WaylandSeat {
    #[doc(alias = "gdk_wayland_seat_get_wl_seat")]
    #[doc(alias = "get_wl_seat")]
    pub fn wl_seat(&self) -> WlSeat {
        unsafe {
            let ptr = ffi::gdk_wayland_seat_get_wl_seat(self.to_glib_none().0);
            Proxy::from_c_ptr(ptr as *mut wl_proxy).into()
        }
    }
}
