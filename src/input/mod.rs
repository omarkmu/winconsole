use super::console;
use super::console::*;
use super::errors::*;

use cgmath::Vector2;

use std::{
    fmt,
    fmt::{Display, Formatter},
    mem,
};

use winapi::um::wincon::{
    KEY_EVENT_RECORD_uChar, COORD, FOCUS_EVENT, FOCUS_EVENT_RECORD, INPUT_RECORD, KEY_EVENT,
    KEY_EVENT_RECORD, MOUSE_EVENT, MOUSE_EVENT_RECORD, MOUSE_HWHEELED, MOUSE_MOVED, MOUSE_WHEELED,
    WINDOW_BUFFER_SIZE_EVENT, WINDOW_BUFFER_SIZE_RECORD,
};

mod etc;
mod input_main;

pub use self::etc::*;
pub use self::input_main::*;

const BUTTON_VIRTUAL: [u8; 5] = [1, 2, 4, 5, 6];
