use std::{mem, ptr};

use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::winuser::{FLASHWINFO, WINDOWPLACEMENT};
use winapi::um::{wincon, winuser};

use super::console::Vector2;
use super::errors::*;

mod etc;
mod window_main;

pub use self::etc::*;
pub use self::window_main::*;
