use super::console::*;
use cgmath::Vector2;
use std::{io, fmt, fmt::{Display, Formatter}, mem};
use winapi::um::wincon::{
	COORD,
	FOCUS_EVENT,
	FOCUS_EVENT_RECORD,
	INPUT_RECORD,
	KEY_EVENT,
	KEY_EVENT_RECORD,
	KEY_EVENT_RECORD_uChar,
	MOUSE_EVENT,
	MOUSE_EVENT_RECORD,
	MOUSE_MOVED,
	MOUSE_WHEELED,
	MOUSE_HWHEELED,
	WINDOW_BUFFER_SIZE_EVENT,
	WINDOW_BUFFER_SIZE_RECORD,
};

mod control_key_state;
mod focus_event;
mod input_context;
mod input_event;
mod input_filter;
mod input_main;
mod key_code;
mod key_event;
mod mouse_event;
mod mouse_move_event;
mod mouse_wheel_event;
mod resize_event;

pub use self::control_key_state::ControlKeyState;
pub use self::focus_event::FocusEvent;
pub use self::input_context::InputContext;
pub use self::input_event::InputEvent;
pub use self::input_filter::InputFilter;
pub use self::input_main::Input;
pub use self::key_code::KeyCode;
pub use self::key_event::KeyEvent;
pub use self::mouse_event::MouseEvent;
pub use self::mouse_move_event::MouseMoveEvent;
pub use self::mouse_wheel_event::MouseWheelEvent;
pub use self::resize_event::ResizeEvent;

type IoResult<T> = io::Result<T>;

const BUTTON_VIRTUAL: [u8; 5] = [1, 2, 4, 5, 6];
