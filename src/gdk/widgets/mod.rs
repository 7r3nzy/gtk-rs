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

pub use self::color::{Color, RGBA};
pub use self::cursor::Cursor;
pub use self::device::Device;
pub use self::device_manager::DeviceManager;
pub use self::display::Display;
pub use self::screen::Screen;
pub use self::types::{Atom, Rectangle, Key};
pub use self::visual::Visual;
pub use self::window::Window;

mod color;
mod cursor;
mod device;
mod device_manager;
mod display;
mod screen;
mod types;
mod visual;
mod window;