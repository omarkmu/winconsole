use super::*;

/// Input handler which acts as an interface with Windows console input-related methods.
pub struct Input {}

impl Input {
	/**
	 * Returns a boolean representing whether or not the key is currently pressed.
	 *
	 * # Arguments
	 * * `key_code` - The KeyCode to retrieve the status of.
	 * 
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::{Input, KeyCode};
	 * # fn main() {
	 * let pressed = Input::is_key_down(KeyCode::Return);
	 * println!("Is [ENTER] pressed? {}", pressed);
	 * # }
	 * ```
	 */
	pub fn is_key_down(key_code: KeyCode) -> bool {
		if key_code == KeyCode::None { return false; }
		Console::get_key_state(key_code as u8 as u32)
	}
	/**
	 * Creates and returns an InputContext, and initialises input.
	 *
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::Input;
	 * # fn main() {
	 * let mut ctx = Input::start().unwrap();
	 * let event = ctx.wait().unwrap();
	 * println!("{}", event);
	 * # }
	 * ```
	 */
	pub fn start() -> IoResult<InputContext> {
		let mut button_status = [false; 5];
		for i in 0..5 {
			button_status[i] = Console::get_key_state(BUTTON_VIRTUAL[i] as u32);
		}
		Console::flush_input().unwrap();
		Ok(InputContext::new(Console::get_input_mode().unwrap(), button_status))
	}

	pub(crate) fn convert_events(records: &Vec<INPUT_RECORD>, ctx: &mut InputContext) -> Vec<InputEvent> {
		let mut ret = Vec::new();
		let button_status = &mut ctx.button_status;
		let held_keys = &mut ctx.held_keys;
		let repeat_enabled = ctx.repeat_enabled;
		for record in records {
			let ev = record.Event;

			match record.EventType {
				MOUSE_EVENT => {
					let mer = unsafe { ev.MouseEvent() };
					let flags = mer.dwEventFlags;
					let position = Vector2::new(mer.dwMousePosition.X as u16, mer.dwMousePosition.Y as u16);
					let modifiers = ControlKeyState::from(mer.dwControlKeyState as u16);
					
					if flags == MOUSE_MOVED {
						let mut mmev = MouseMoveEvent::new();
						mmev.modifiers = modifiers;
						mmev.position = position;
						ret.push(InputEvent::MouseMove(mmev));
					} else if flags & (MOUSE_WHEELED | MOUSE_HWHEELED) != 0  {
						let mut mwev = MouseWheelEvent::new();
						mwev.delta = (mer.dwButtonState as i32) / 65536;
						mwev.horizontal = flags & MOUSE_HWHEELED != 0;
						mwev.modifiers = modifiers;
						mwev.position = position;
						ret.push(InputEvent::MouseWheel(mwev));
					} else {
						for i in 0..5 {
							let status = mer.dwButtonState & (0x1 << i) != 0;
							if status == button_status[i] { continue; }

							let mut mev = MouseEvent::new();
							mev.button = (i as u8) + 1;
							mev.modifiers = modifiers;
							mev.position = position;
							mev.pressed = status;
							mev.key_code = KeyCode::from(BUTTON_VIRTUAL[i]);

							button_status[i] = status;
							if status {
								ret.push(InputEvent::MouseDown(mev));
							} else {
								ret.push(InputEvent::MouseUp(mev));
							}
						}
					}
				},
				KEY_EVENT => {
					let ker = unsafe { ev.KeyEvent() };
					let virt = ker.wVirtualKeyCode as u8;
					if BUTTON_VIRTUAL.contains(&virt) { continue; }

					let key_code = KeyCode::from(virt);
					let character = unsafe { *(ker.uChar.AsciiChar()) };
					let status = ker.bKeyDown != 0;
					let mut kev = KeyEvent::new();
					kev.character = character as u8 as char;
					kev.key_code = key_code;
					kev.modifiers = ControlKeyState::from(ker.dwControlKeyState as u16);
					kev.pressed = status;
					kev.repeat_count = ker.wRepeatCount;
					kev.scan_code = ker.wVirtualScanCode;

					if status {
						if held_keys.contains(&key_code) {
							if repeat_enabled {
								ret.push(InputEvent::KeyHeld(kev));
							}
						} else {
							ret.push(InputEvent::KeyDown(kev));
							held_keys.push(key_code);
						}
					} else {
						if held_keys.contains(&key_code) {
							let index = held_keys.iter().position(|k| *k == key_code).unwrap();
							held_keys.remove(index);
						}
						ret.push(InputEvent::KeyUp(kev));
					}
				},
				FOCUS_EVENT => {
					let fer = unsafe { ev.FocusEvent() };
					let focused = fer.bSetFocus != 0;
					let mut fev = FocusEvent::new();
					fev.focused = focused;

					if focused {
						ret.push(InputEvent::Focused(fev));
					} else {
						ret.push(InputEvent::FocusLost(fev));
					}
				},
				WINDOW_BUFFER_SIZE_EVENT => {
					let wer = unsafe { ev.WindowBufferSizeEvent() };
					let mut rev = ResizeEvent::new();
					rev.size = Vector2::new(wer.dwSize.X as u16, wer.dwSize.Y as u16);
					ret.push(InputEvent::Resize(rev));
				},
				_ => ()
			}
		}
		ret
	}
}
