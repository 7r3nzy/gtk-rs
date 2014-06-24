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

#![macro_escape]

macro_rules! check_pointer(
    ($tmp_pointer:ident, $gtk_struct:ident) => (
        if $tmp_pointer.is_null() {
            None
        } else {
            Some($gtk_struct {
                pointer:            $tmp_pointer,
                can_drop:           true
            })
        }
    );
)

macro_rules! struct_Widget(
    ($gtk_struct:ident) => (
        pub struct $gtk_struct {
            pointer:           *ffi::C_GtkWidget,
            can_drop:          bool
        }
    );
)

macro_rules! impl_GtkWidget(
    ($gtk_struct:ident) => (
        impl ::gtk::traits::WidgetTrait for $gtk_struct {
            fn get_widget(&self) -> *ffi::C_GtkWidget {
                self.pointer
            }

            fn wrap_widget(widget: *ffi::C_GtkWidget) -> $gtk_struct {
                $gtk_struct {
                    pointer:         widget,
                    can_drop:        false
                }
            }
        }
    );
)

/*macro_rules! struct_signal(
    ($gtk_struct:ident) => (

        #[doc(hidden)]
        pub struct SignalHandler {
            function: Option<fn(&mut $gtk_struct, *c_void)>,
            function_widget: Option<fn(&mut $gtk_struct, Option<&mut GtkWidget>)>,
            user_data: *c_void
        }
    );
)*/

/*macro_rules! impl_signals(
    ($gtk_struct:ident) => (
        impl Signal for $gtk_struct {
            fn connect(&mut self, signal: &str, function: fn()) -> () {
                unsafe {
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect(self.pointer, c_str, Some(function))
                    })
                }
            }

            fn connect_2p<B>(&mut self,
                             signal: &str,
                             function: fn(&mut $gtk_struct, *c_void),
                             user_data: Option<&B>) -> () {
                use std::mem::transmute;

                unsafe {
                    let tmp_sighandler_ptr: *c_void = transmute(box SignalHandler {
                        function: Some(function),
                        function_widget: None,
                        user_data: transmute(user_data.unwrap())
                    });
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer,
                                                    c_str,
                                                    Some(redirect_callback),
                                                    tmp_sighandler_ptr)
                    });
                    self.signal_handlers.push(transmute(tmp_sighandler_ptr));
                }
            }

            fn connect_2p_widget<B: GtkWidget>(&mut self,
                                               signal: &str,
                                               function: fn(&mut $gtk_struct, Option<&mut GtkWidget>),
                                               user_data: Option<&B>) -> () {
                use std::mem::transmute;
                use std::ptr;

                unsafe{
                    let tmp_sighandler_ptr: *c_void = transmute(box SignalHandler {
                        function: None,
                        function_widget: Some(function),
                        user_data: if user_data.is_some() { transmute(user_data.unwrap()) } else { ptr::null() }
                    });

                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer,
                                                    c_str,
                                                    Some(redirect_callback_widget),
                                                    tmp_sighandler_ptr)
                    });

                    self.signal_handlers.push(transmute(tmp_sighandler_ptr));
                }
            }

        }
    );
)*/