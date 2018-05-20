use cgmath::Vector2;
use rgb::RGB8;

use std;
use std::{io, mem, ptr};

use winapi::ctypes::c_void as VOID;
use winapi::shared::minwindef::{DWORD, MAX_PATH, UINT, WORD};
use winapi::um::{consoleapi, processenv, utilapiset, wincon};
use winapi::um::winbase::{
	STD_OUTPUT_HANDLE as STDOUT,
	STD_INPUT_HANDLE as STDIN
};
use winapi::um::wincon::{
	COORD,
	CONSOLE_CURSOR_INFO,
	CONSOLE_FONT_INFOEX,
	CONSOLE_READCONSOLE_CONTROL,
	CONSOLE_SCREEN_BUFFER_INFO,
	CONSOLE_SCREEN_BUFFER_INFOEX,
	CONSOLE_SELECTION_INFO
};
use winapi::um::winnt::{CHAR, WCHAR};

type IoResult<T> = io::Result<T>;
type BoxedResult<T> = Result<T, Box<std::error::Error>>;

mod console_main;
pub use self::console_main::Console;
pub use super::etc::*;

#[cfg(feature = "input")]
mod console_input;
