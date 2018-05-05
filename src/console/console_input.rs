use super::*;
use winapi::um::{wincon::INPUT_RECORD, winuser};

impl Console {
	pub(crate) fn get_key_state(key: u32) -> bool {
		let num: i16;
		unsafe { num = winuser::GetAsyncKeyState(key as i32) }
		num != 0
	}
	pub(crate) fn num_input_events() -> IoResult<u32> {
		let mut num: DWORD = 0;
		os_err!(unsafe {
			let handle = handle!(STDIN);
			let num_p = &mut num as *mut DWORD;
			consoleapi::GetNumberOfConsoleInputEvents(handle, num_p)
		});
		Ok(num)
	}
	pub(crate) fn num_mouse_buttons() -> IoResult<u32> {
		let mut num: DWORD = 0;
		os_err!(unsafe {
			let num_p = &mut num as *mut DWORD;
			wincon::GetNumberOfConsoleMouseButtons(num_p)
		});
		Ok(num)
	}
    pub(crate) fn read_input(length: usize) -> IoResult<Vec<INPUT_RECORD>> {
		let mut num: DWORD = 0;
		let mut buffer: Box<[INPUT_RECORD]>;
        os_err!(unsafe {
            let handle = handle!(STDIN);
            buffer = buf_mem!(length);

			let length = length as DWORD;
			let buffer_p = &mut buffer[0] as *mut INPUT_RECORD;
            let num_p = &mut num as *mut DWORD;
			consoleapi::ReadConsoleInputA(handle, buffer_p, length, num_p)
        });
        Ok(buf_to_vec!(buffer, num))
    }
}
