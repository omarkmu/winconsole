use super::*;
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
	static ref PAGES: Mutex<Vec<CodePage>> = Mutex::new(vec![]);
}

/// Console handler which acts as an interface with the Windows console API.
pub struct Console;

impl Console {
	/**
	 Generates a tone on the speaker.
	
	 # Arguments
	 * `frequency` - The frequency of the tone, in hertz.
	 * `duration` - The duration of the sound, in milliseconds.
	
	 # Examples
	 Plays a note of A4 frequency for 1 seconds.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::beep(440, 1000).unwrap();
	 # }
	 ```
	 */
	pub fn beep(frequency: u32, duration: u32) -> WinResult<()> {
		os_err!(unsafe { utilapiset::Beep(frequency, duration) });
		Ok(())
	}
	/**
	 Clears the console.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 println!("Hello, wolrd!");
	 println!("Wait, that's not right..");
	 Console::clear().unwrap();
	 println!("Hello, world!");
	 # }
	 ```
	 */
	pub fn clear() -> WinResult<()> {
		let size = Console::get_buffer_size()?;
		let length = size.x as DWORD * size.y as DWORD;
		Console::fill_char(32, length, COORD { X: 0, Y: 0 })?;
		Console::fill_attributes(Console::get_text_attributes()?, length, COORD { X: 0, Y: 0 })?;
		Console::set_cursor_position(0, 0)
	}
	/**
	 Clears the console history.
	 
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::clear_history().unwrap();
	 # }
	 ```
	 */
	pub fn clear_history() -> WinResult<()> {
		let old = Console::get_history_info()?;
		let mut empty = old.clone();
		empty.size = 0;
		Console::set_history_info(empty)?;
		Console::set_history_info(old)?;
		Ok(())
	}
	/**
	 Fills the console window with a specified character starting 
	 at a specified location, and returns the number of cells which were filled.  
	 Note that this only changes the character; the colors of each cell will remain the same.
	
	 # Arguments
	 * `chr` - The character to fill the console with.
	 * `column` - The column at which the fill should begin.
	 * `row` - The row at which the fill should begin.
	 * `max_length` - The maximum amount of cells to fill.
	 If None, fill the entirety of the console from the start position.
	
	 # Examples
	 Fills the entire console with 'Z'.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::fill_character('Z', 0, 0, None).unwrap();
	 # }
	 ```
	 Fills the first ten cells with 'G'.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::fill_character('G', 0, 0, 10).unwrap();
	 # }
	 ```
	 */
	pub fn fill_character<T: Into<Option<u32>>>(chr: char, column: u16, row: u16, max_length: T) -> WinResult<u32> {
		let coords = COORD { X: column as i16, Y: row as i16 };
		let length = match max_length.into() {
			Some(len) => len,
			None => {
				let size = Console::get_buffer_size()?;
				let con_length = size.x as DWORD * size.y as DWORD;
				let start_pos = column as DWORD * row as DWORD;
				if start_pos > con_length { return Ok(0); }
				con_length - start_pos
			}
		};
		Console::fill_char(chr as CHAR, length, coords)
	}
	/**
	 Fills the console window with a specified set of colors starting 
	 at a specified location, and returns the number of cells which were filled.  
	
	 # Arguments
	 * `colors` - The colors to fill the console with. The first item in the tuple is
	 the foreground color, and the second item is the background color.
	 * `column` - The column at which the fill should begin.
	 * `row` - The row at which the fill should begin.
	 * `max_length` - The maximum amount of cells to fill.
	 If None, fill the entirety of the console from the start position.
	
	 # Examples
	 Fills the entire console with a blue foreground and red background.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 Console::fill_colors(&(ConsoleColor::Blue, ConsoleColor::Red), 0, 0, None).unwrap();
	 # }
	 ```
	 */
	pub fn fill_colors<T: Into<Option<u32>>>(colors: &(ConsoleColor, ConsoleColor), column: u16, row: u16, max_length: T) -> WinResult<u32> {
		let coords = COORD { X: column as i16, Y: row as i16 };
		let length = match max_length.into() {
			Some(len) => len,
			None => {
				let size = Console::get_buffer_size()?;
				let con_length = size.x as DWORD * size.y as DWORD;
				let start_pos = column as DWORD * row as DWORD;
				if start_pos > con_length { return Ok(0); }
				con_length - start_pos
			}
		};
		let attrs = (colors.0.get_value() | ((colors.1.get_value()) << 4)) as WORD;
		Console::fill_attributes(attrs, length, coords)
	}
	/**
	 Flushes the console input buffer.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::flush_input().unwrap();
	 # }
	 ```
	 */
	pub fn flush_input() -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(STDIN);
			wincon::FlushConsoleInputBuffer(handle)
		});
		Ok(())
	}
	/**
	 Flushes the console output buffer.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::flush_output().unwrap();
	 # }
	 ```
	 */
	pub fn flush_output() -> WinResult<()> {
		io::stdout().flush()?;
		Ok(())
	}
	/**
	 Sends a ctrl signal to a process group which shares the console.

	 # Arguments
	 * `break_event`- Should a CTRL + BREAK signal be generated? Otherwise, a CTRL + C signal will be generated.
	 A CTRL + C signal cannot be generated for a process group.
	 * `process_group_id` - The ID of the process group to generate the event on. If None, generate the event on
	 processes which share the console.

	 # Examples
	 Generates a CTRL event.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::generate_ctrl_event(false, None).unwrap();
	 # }
	 ```
	 */
	pub fn generate_ctrl_event<T: Into<Option<u32>>>(break_event: bool, process_group_id: T) -> WinResult<()> {
		let id: u32 = match process_group_id.into() {
			None => 0,
			Some(id) => id
		};
		let event = bool_to_num!(break_event) as u32;
		os_err!(unsafe { wincon::GenerateConsoleCtrlEvent(event, id) });
		Ok(())
	}
	/**
	 Reads a single character from the input buffer.
	 Note that this will wait for input from the user, and will only accept certain characters;
	 this will not return from a control key press event.
	
	 # Arguments
	 * `suppress` - Should the character be returned without printing?
	
	 # Examples
	 Gets a character and prints it to the console.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::getch(false).unwrap();
	 # }
	 ```
	 */
	pub fn getch(suppress: bool) -> WinResult<char> {
		let old_mode = Console::get_input_mode()?;
		let mut mode = old_mode.clone();
		mode.EchoInput = false;
		mode.LineInput = false;
		Console::set_input_mode(mode)?;

		let mut res: CHAR = 0;
		os_err!(unsafe {
			let mut num: DWORD = 0;
			let handle = handle!(STDIN);
			let buffer_p = &mut res as *mut CHAR as *mut VOID;
			let control_p: *mut CONSOLE_READCONSOLE_CONTROL = ptr::null_mut();
			consoleapi::ReadConsoleA(handle, buffer_p, 1, &mut num, control_p)
		});
		let res = res as u8 as char;
		Console::set_input_mode(old_mode)?;

		if !suppress {
			print!("{}", res);
			Console::flush_output()?;
		}
		Ok(res)
	}
	/**
	 Returns the current background color of the console.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let background = Console::get_background_color().unwrap();
	 println!("Background color: {}", background);
	 # }
	 ```
	 */
	pub fn get_background_color() -> WinResult<ConsoleColor> {
		let attrs = Console::get_text_attributes()?;
		Ok(ConsoleColor::from((attrs & 0xF0) >> 4))
	}
	/**
	 Returns the size of the console output buffer.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let buffer_size = Console::get_buffer_size().unwrap();
	 Console::set_buffer_size(buffer_size.x + 1, buffer_size.y + 1);
	 # }
	 ```
	 */
	pub fn get_buffer_size() -> WinResult<Vector2<u16>> {
		let coords = Console::get_screen_buffer_info()?.dwSize;
		Ok(Vector2::new(coords.X as u16, coords.Y as u16))
	}
	/**
	 Returns a CodePageInfo object which contains information about the CodePage.

	 # Arguments
	 * `page` - The CodePage to retrieve information about.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, CodePage};
	 # fn main() {
	 let info = Console::get_code_page_info(CodePage::utf_8).unwrap();
	 println!("{}", info.name);
	 # }
	 ```
	 */
	pub fn get_code_page_info(page: CodePage) -> WinResult<CodePageInfo> {
		let mut info: CPINFOEXA = unsafe { mem::zeroed() };
		let identifier: u16 = page.into();
		os_err!(unsafe { winnls::GetCPInfoExA(identifier as u32, 0, &mut info) });

		let mut cpi = CodePageInfo::new();
		cpi.max_char_size = info.MaxCharSize as u8;
		cpi.default = buf_to_str!(info.DefaultChar);
		cpi.lead_byte = info.LeadByte;
		cpi.unicode_default = info.UnicodeDefaultChar;
		cpi.code_page = CodePage::from(info.CodePage as u16);
		cpi.name = buf_to_str!(info.CodePageName);

		Ok(cpi)
	}
	/**
	 Returns the RGB color value of a ConsoleColor.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 let black = Console::get_color(ConsoleColor::Black).unwrap();
	 println!("{:?}", black);
	 # }
	 ```
	 */
	pub fn get_color(color: ConsoleColor) -> WinResult<RGB8> {
		let mapping = Console::get_color_mapping()?;
		Ok(mapping[color.get_value() as usize])
	}
	/**
	 Returns the current color mapping for the console.
	 The indices of the returned array correspond with ConsoleColor values.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mapping = Console::get_color_mapping().unwrap();
	 let black = mapping[0];
	 println!("{:?}", black);
	 # }
	 ```
	 */
	pub fn get_color_mapping() -> WinResult<[RGB8; 16]> {
		let colors = Console::get_screen_buffer_info_ex()?.ColorTable;
		let mut ret = [RGB8{r: 0, g: 0, b: 0}; 16];
		for i in 0..16 {
			ret[i] = make_rgb!(colors[i]);
		}
		Ok(ret)
	}
	/**
	 Returns the current position of the console cursor.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let position = Console::get_cursor_position().unwrap();
	 println!("{:?}", position);
	 # }
	 ```
	 */
	pub fn get_cursor_position() -> WinResult<Vector2<u16>> {
		let pos = Console::get_screen_buffer_info()?.dwCursorPosition;
		Ok(Vector2::new(pos.X as u16, pos.Y as u16))
	}
	/**
	 Returns the size of the console cursor.  
	 The size of the console cursor will always be between 0 and 100 (inclusive).
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let cursor_size = Console::get_cursor_size().unwrap();
	 println!("{}", cursor_size);
	 # }
	 ```
	 */
	pub fn get_cursor_size() -> WinResult<u8> {
		let info = Console::get_cursor_info()?;
		Ok(info.dwSize as u8)
	}
	/**
	 Returns information about the current console font.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let font = Console::get_font().unwrap();
	 println!("{}", font.name);
	 # }
	 ```
	 */
	pub fn get_font() -> WinResult<ConsoleFont> {
		let info = Console::get_font_info_ex(false)?;
		let size = info.dwFontSize;
		Ok(ConsoleFont {
			family: info.FontFamily,
			index: info.nFont,
			name: buf_to_str!(info.FaceName),
			size: Vector2::new(size.X as u16, size.Y as u16),
			weight: info.FontWeight
		})
	}
	/**
	 Returns the current foreground color of the console.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let foreground = Console::get_foreground_color().unwrap();
	 println!("Foreground color: {}", foreground);
	 # }
	 ```
	 */
	pub fn get_foreground_color() -> WinResult<ConsoleColor> {
		let attrs = Console::get_text_attributes()?;
		Ok(ConsoleColor::from(attrs & 0xF))
	}
	/**
	 Returns a HistoryInfo object containing information about console history settings.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let history_settings = Console::get_history_info().unwrap();
	 println!("{:?}", history_settings);
	 # }
	 ```
	 */
	pub fn get_history_info() -> WinResult<HistoryInfo> {
		let mut info: CONSOLE_HISTORY_INFO = unsafe { mem::zeroed() };
		info.cbSize = mem::size_of::<CONSOLE_HISTORY_INFO>() as DWORD;
		os_err!(unsafe { wincon::GetConsoleHistoryInfo(&mut info) });

		let mut history = HistoryInfo::new();
		history.size = info.HistoryBufferSize;
		history.number_of_buffers = info.NumberOfHistoryBuffers;
		history.duplicates_allowed = info.dwFlags & 0x1 == 0;

		Ok(history)
	}
	/**
	 Returns the input code page used by the console.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let page = Console::get_input_code_page();
	 println!("{}", page);
	 # }
	 ```
	 */
	pub fn get_input_code_page() -> CodePage {
		CodePage::from(unsafe { consoleapi::GetConsoleCP() } as u16)
	}
	/**
	 Returns settings related to console input.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mode = Console::get_input_mode().unwrap();
	 println!("{}", mode);
	 # }
	 ```
	 */
	pub fn get_input_mode() -> WinResult<InputSettings> {
		let mode = Console::get_mode(STDIN)?;
		Ok(InputSettings::from(mode))
	}
	/**
	 Returns a list of installed code pages.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let pages = Console::get_installed_code_pages().unwrap();
	 for page in pages {
	 	println!("{}", page);
	 }
	 # }
	 ```
	 */
	pub fn get_installed_code_pages() -> WinResult<Vec<CodePage>> {
		Console::get_code_pages(1)
	}
	/**
	 Returns the original title of the console window.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let original_title = Console::get_original_title().unwrap();
	 println!("{}", original_title);
	 # }
	 ```
	 */
	pub fn get_original_title() -> WinResult<String> {
		let mut buffer: [CHAR; MAX_PATH] = [0; MAX_PATH];

		let length = unsafe {
			let buffer_p = &mut buffer[0] as *mut CHAR;
			wincon::GetConsoleOriginalTitleA(buffer_p, MAX_PATH as u32)
		};
		os_err!(length, true);
		Ok(buf_to_str!(buffer))
	}
	/**
	 Returns the input code page used by the console.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let page = Console::get_output_code_page();
	 println!("{}", page);
	 # }
	 ```
	 */
	pub fn get_output_code_page() -> CodePage {
		CodePage::from(unsafe { consoleapi::GetConsoleOutputCP() } as u16)
	}
	/**
	 Returns settings related to console output.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mode = Console::get_output_mode().unwrap();
	 println!("{}", mode);
	 # }
	 ```
	 */
	pub fn get_output_mode() -> WinResult<OutputSettings> {
		let mode = Console::get_mode(STDOUT)?;
		Ok(OutputSettings::from(mode))
	}
	/**
	 Returns a SelectionInfo object containing information about console selection.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let selection = Console::get_selection_info().unwrap();
	 println!("{:?}", selection);
	 # }
	 ```
	 */
	pub fn get_selection_info() -> WinResult<SelectionInfo> {
		let mut info: CONSOLE_SELECTION_INFO = unsafe { mem::zeroed() };
		os_err!(unsafe { wincon::GetConsoleSelectionInfo(&mut info) });

		let anchor = info.dwSelectionAnchor;
		let rect = info.srSelection;
		let flags = info.dwFlags;
		let rect = Rect::new(rect.Top as u16, rect.Left as u16, rect.Bottom as u16, rect.Top as u16);

		let mut selection = SelectionInfo::new();
		selection.anchor = Vector2::new(anchor.X as u16, anchor.Y as u16);
		selection.bottom_right = rect.bottom_right();
		selection.empty = flags & 0x2 == 0;
		selection.mouse_down = flags & 0x8 != 0;
		selection.rect = rect;
		selection.selecting = flags & 0x1 != 0;
		selection.top_left = rect.top_left();

		Ok(selection)
	}
	/**
	 Returns a ConsoleState object containing information about the current state of the console.
	
	 # Arguments
	 * `copy_output` - Should the state contain information about the output buffer?
	 * `copy_all` - Should the state copy all of the output buffer (as opposed to reading the buffer from
	 the start to the current cursor position)?
	
	 # Examples
	 Retrieves the state of the console, copying the output up to the location of the
	 console cursor.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let state = Console::get_state(true, false).unwrap();
	 println!("{}", state.output.len());
	 # }
	 ```
	 */
	pub fn get_state(copy_output: bool, copy_all: bool) -> WinResult<ConsoleState> {
		let mut state = ConsoleState::new();
		let buffer_size = Console::get_buffer_size()?;
		let cursor_position = Console::get_cursor_position()?;

		state.background_color = Console::get_background_color()?;
		state.buffer_size = buffer_size;
		state.color_mapping = Console::get_color_mapping()?;
		state.cursor_position = cursor_position;
		state.cursor_size = Console::get_cursor_size()?;
		state.cursor_visible = Console::is_cursor_visible()?;
		state.font = Console::get_font()?;
		state.foreground_color = Console::get_foreground_color()?;
		state.input_code_page = Console::get_input_code_page();
		state.input_mode = Console::get_input_mode()?;
		state.output_code_page = Console::get_output_code_page();
		state.output_mode = Console::get_output_mode()?;
		state.title = Console::get_title()?;

		if copy_output {
			let length: u32 = if copy_all {
				buffer_size.x as u32 * buffer_size.y as u32
			} else {
				buffer_size.x as u32 * cursor_position.y as u32 + cursor_position.x as u32
			};
			state.output = Console::read_output(0, 0, length)?;
			state.output_colors = Console::read_output_colors(0, 0, length)?;
		}
		Ok(state)
	}
	/**
	 Returns a list of supported code pages.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let pages = Console::get_supported_code_pages().unwrap();
	 for page in pages {
	 	println!("{}", page);
	 }
	 # }
	 ```
	 */
	pub fn get_supported_code_pages() -> WinResult<Vec<CodePage>> {
		Console::get_code_pages(2)
	}
	/**
	 Returns the title of the console window.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let title = Console::get_title().unwrap();
	 println!("{}", title);
	 # }
	 ```
	 */
	pub fn get_title() -> WinResult<String> {
		let mut buffer: [CHAR; MAX_PATH] = [0; MAX_PATH];

		let length = unsafe {
			let buffer_p = &mut buffer[0] as *mut CHAR;
			wincon::GetConsoleTitleA(buffer_p, MAX_PATH as u32)
		};
		os_err!(length, true);
		Ok(buf_to_str!(buffer))
	}
	/**
	 Returns the size of the window relative to the screen buffer.
	 These dimensions also serve as minimum values for the size of the buffer.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let size = Console::get_window_size().unwrap();
	 println!("Minimum columns: {}. Minimum rows: {}.", size.x, size.y);
	 # }
	 ```
	 */
	pub fn get_window_size() -> WinResult<Vector2<u16>> {
		let rect = Console::get_screen_buffer_info()?.srWindow;
		Ok(Vector2::new((rect.Right - rect.Left + 1) as u16, (rect.Bottom - rect.Top + 1) as u16))
	}
	/**
	 Returns a boolean representing whether or not the console cursor is visible.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let visible = Console::is_cursor_visible().unwrap();
	 println!("Is the cursor visible? {}", visible);
	 # }
	 ```
	 */
	pub fn is_cursor_visible() -> WinResult<bool> {
		let info = Console::get_cursor_info()?;
		Ok(info.bVisible == 1)
	}
	/**
	 Returns a boolean representing whether or not the supplied value is a valid code page.
	 A code page is considered valid if it is installed on the system.

	 # Arguments
	 * `identifier` - The code page identifier to check.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let valid = Console::is_valid_code_page(0);
	 assert_eq!(valid, false);
	 # }
	 ```
	 */
	pub fn is_valid_code_page(identifier: u16) -> bool {
		let valid = unsafe { winnls::IsValidCodePage(identifier as u32) };
		valid != 0
	}
	/**
	 Maps a ConsoleColor to an RGB8 value.
	
	 # Arguments
	 * `color` - The ConsoleColor to map.
	 * `rgb` - The RGB color which the ConsoleColor should represent.
	
	 # Examples
	 Maps `ConsoleColor::Black` to white.

	 ```
	 # extern crate winconsole;
	 # extern crate rgb;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 use rgb::RGB8;
	 Console::map_color(ConsoleColor::Black, RGB8 {r: 255, g: 255, b: 255}).unwrap();
	 # }
	 ```
	 */
	pub fn map_color(color: ConsoleColor, rgb: RGB8) -> WinResult<()> {
		let mut info = Console::get_screen_buffer_info_ex()?;
		info.ColorTable[color.get_value() as usize] = make_colorref!(rgb);
		info.srWindow.Bottom += 1;
		info.srWindow.Right += 1;
		Console::set_screen_buffer_info_ex(&mut info)
	}
	/**
	 Moves data from a rectangle of the console output to another point in the output.
	 The effects of the move can be limited by specifying a clipping rectangle,
	 so the contents of the console screen buffer outside the clipping rectangle are unchanged.

	 # Arguments
	 * `scroll` - The rectangle to be moved.
	 * `dest` - The upper-left corner of the new location of the contents.
	 * `clip` - An optional clipping rectangle.
	 * `fill_char` - A character to fill in spaces which were left empty as a result of the move.
	 * `fill_fg_color` - The foreground to fill in spaces which were left empty as a result of the move.
	 * `fill_bg_color` - The background to fill in spaces which were left empty as a result of the move.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # extern crate cgmath;
	 # use cgmath::Vector2;
	 # use winconsole::console::{Console, Rect};
	 # fn main() {
	 let scroll = Rect::new(0, 0, 10, 10);
	 let dest = Vector2::new(0, 3);
	 Console::move_contents(scroll, dest, None, None, None, None).unwrap();
	 # }
	 ```
	 */
	pub fn move_contents<T: Into<Option<Rect>>, U: Into<Option<char>>, V: Into<Option<ConsoleColor>>>(scroll: Rect, dest: Vector2<i16>,
		clip: T, fill_char: U, fill_fg_color: V, fill_bg_color: V) -> WinResult<()> {
		
		let fill_char = match fill_char.into() {
			Some(f) => f,
			None => ' '
		};
		let attrs = {
			let fg_color = match fill_fg_color.into() {
				Some(f) => f,
				None => Console::get_foreground_color()?
			};
			let bg_color = match fill_bg_color.into() {
				Some(f) => f,
				None => Console::get_background_color()?
			};
			((bg_color as WORD) << 4) | (fg_color as WORD)
		};
		let dest = COORD { X: dest.x, Y: dest.y};
		let scroll = SMALL_RECT {
			Top: scroll.top as i16,
			Bottom: scroll.bottom as i16,
			Left: scroll.left as i16,
			Right: scroll.right as i16
		};

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let scroll_p = &scroll as *const SMALL_RECT;
			let clip_p = match clip.into() {
				Some(c) => {
					let rect = SMALL_RECT {
						Top: c.top as i16,
						Bottom: c.bottom as i16,
						Left: c.left as i16,
						Right: c.right as i16
					};
					&rect as *const SMALL_RECT
				},
				None => ptr::null()
			};
			let info_p = {
				let mut char_info: CHAR_INFO = mem::zeroed();
				let mut chr: CHAR_INFO_Char = mem::zeroed();
				*chr.AsciiChar_mut() = fill_char as CHAR;

				char_info.Attributes = attrs;
				char_info.Char = chr;
				&char_info as *const CHAR_INFO
			};

			wincon::ScrollConsoleScreenBufferA(handle, scroll_p, clip_p, dest, info_p)
		});

		Ok(())
	}
	/**
	 Reads a string from the console output starting at a specified location.
	 Returns an error if the position is not within the buffer bounds.  
	 Note that this method reads the output buffer _directly_ (i.e., an empty end of a line will
	 be made up of multiple space characters rather than a newline character sequence).
	
	 # Arguments
	 * `column` - The column at which reading should begin.
	 * `row` - The row at which reading should begin.
	 * `max_length` - The maximum amount of characters to read. If None, the entire output buffer is read.
	
	 # Examples
	 Reads the entire console output buffer.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let output = Console::read_output(0, 0, None).unwrap();
	 println!("{}", output.len());
	 # }
	 ```
	 */
	pub fn read_output<T: Into<Option<u32>>>(column: u16, row: u16, max_length: T) -> WinResult<String> {
		let buffer_size = Console::get_buffer_size()?;
		if column >= buffer_size.x {
			throw_err!(ArgumentError::new("column", "column must be within the buffer"));
		} else if row >= buffer_size.y {
			throw_err!(ArgumentError::new("row", "row must be within the buffer"));
		}
		let max_length = match max_length.into() {
			Some(len) => len,
			None => {
				let size = Console::get_buffer_size()?;
				let con_length = size.x as DWORD * size.y as DWORD;
				let start_pos = column as DWORD * row as DWORD;
				if start_pos > con_length { return Ok(String::new()); }
				con_length - start_pos
			}
		};

		if max_length == 0 { return Ok(String::new()); }

		let mut num: DWORD = 0;
		let mut buffer: Box<[CHAR]> = buf!(max_length as usize);
		let coords = COORD { X: column as i16, Y: row as i16 };

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let buffer_p = &mut (*buffer)[0] as *mut CHAR;
			wincon::ReadConsoleOutputCharacterA(handle, buffer_p, max_length, coords, &mut num)
		});
		Ok(buf_to_str!(buffer))
	}
	/**
	 Reads colors from the console output starting at a specified location, and returns a vector of tuples.
	 The first item in each tuple is the foreground color, and the second is the background color.
	 Returns an error if the position is not within the buffer bounds.  
	
	 # Arguments
	 * `column` - The column at which reading should begin.
	 * `row` - The row at which reading should begin.
	 * `max_length` - The maximum amount of colors to read. If None, the entire output buffer is read.
	
	 # Examples
	 Prints the colors in the first cell.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let colors = Console::read_output_colors(0, 0, 1).unwrap();
	 println!("{} {}", colors[0].0, colors[0].1);
	 # }
	 ```
	 */
	pub fn read_output_colors<T: Into<Option<u32>>>(column: u16, row: u16, max_length: T) -> WinResult<Vec<(ConsoleColor, ConsoleColor)>> {
		let buffer_size = Console::get_buffer_size()?;
		if column >= buffer_size.x {
			throw_err!(ArgumentError::new("column", "column must be within the buffer"));
		} else if row >= buffer_size.y {
			throw_err!(ArgumentError::new("row", "row must be within the buffer"));
		}
		let max_length = match max_length.into() {
			Some(len) => len,
			None => {
				let size = Console::get_buffer_size()?;
				let con_length = size.x as DWORD * size.y as DWORD;
				let start_pos = column as DWORD * row as DWORD;
				if start_pos > con_length { return Ok(Vec::new()); }
				con_length - start_pos
			}
		};
		if max_length == 0 { return Ok(Vec::new()); }

		let mut num: DWORD = 0;
		let mut buffer: Box<[WORD]> = buf!(max_length as usize);
		let coords = COORD { X: column as i16, Y: row as i16 };

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let buffer_p = &mut (*buffer)[0] as *mut WORD;
			wincon::ReadConsoleOutputAttribute(handle, buffer_p, max_length, coords, &mut num)
		});
		let vec: Vec<(ConsoleColor, ConsoleColor)> = buffer.iter()
			.map(|attrs| (
				ConsoleColor::from(attrs & 0xF),
				ConsoleColor::from((attrs & 0xF0) >> 4)))
			.collect();
		Ok(vec)
	}
	/**
	 Sets the background color of the console.
	
	 # Arguments
	 * `color` - The color which will be assigned to the background.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 Console::set_background_color(ConsoleColor::DarkBlue).unwrap();
	 # }
	 ```
	 */
	pub fn set_background_color(color: ConsoleColor) -> WinResult<()> {
		let color = color as WORD;
		let current = Console::get_text_attributes()?;

		Console::set_text_attributes(color << 4 | (current & 0xF))
	}
	/**
	 Sets the size of the output buffer.
	 Returns an error if this size is smaller than the window's minimum amount of cells.
	
	 # Arguments
	 * `width` - The amount of columns the screen buffer should have.
	 * `height` - The amount of rows the screen buffer should have.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::set_buffer_size(200, 100).unwrap();
	 # }
	 ```
	 */
	pub fn set_buffer_size(width: u16, height: u16) -> WinResult<()> {
		let window_size = Console::get_window_size()?;

		if width < window_size.x {
			throw_err!(ArgumentError::new("width", "width must be more than window width"));
		} else if height < window_size.y {
			throw_err!(ArgumentError::new("height", "height must be more than window height"));
		}
		let coords = COORD { X: width as i16, Y: height as i16};

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::SetConsoleScreenBufferSize(handle, coords)
		});
		Ok(())
	}
	/**
	 Sets the color mapping of the console.
	 The indices of the array correspond with ConsoleColor values.
	
	 # Arguments
	 * `mapping` - The color mapping to set.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # extern crate rgb;
	 # use winconsole::console::Console;
	 # fn main() {
	 use rgb::RGB8;
	 let mut mapping = Console::get_color_mapping().unwrap();
	 mapping[0] = RGB8 {r: 255, g: 255, b: 255};
	 Console::set_color_mapping(&mapping).unwrap();
	 # }
	 ```
	 */
	pub fn set_color_mapping(mapping: &[RGB8; 16]) -> WinResult<()> {
		let mut info = Console::get_screen_buffer_info_ex()?;
		let mut colors = info.ColorTable;
		for i in 0..16 {
			let color = mapping[i];
			colors[i] = make_colorref!(color);
		}

		info.ColorTable = colors;
		info.srWindow.Bottom += 1;
    	info.srWindow.Right += 1;
		Console::set_screen_buffer_info_ex(&mut info)
	}
	/**
	 Adds or removes a handler routine from the console.

	 # Arguments
	 * `routine` - The callback function. If this is None, a value of `true` for `add`
	 will ignore CTRL + C input, and a value of `false` will restore normal processing.
	 * `add` - Should the routine be added or removed?

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 unsafe extern "system" fn handler(event_type: u32) -> i32 {
		 if event_type == 0 {
			 println!("CTRL + C pressed.");
			 return 1; // TRUE
		 }
		 return 0; // FALSE
	 }
	 Console::set_ctrl_handler(Some(handler), true).unwrap();
	 # }
	 ```

	 # See
	 [HandlerRoutine](https://docs.microsoft.com/en-us/windows/console/handlerroutine).
	 */
	pub fn set_ctrl_handler(handler: Option<HandlerRoutine>, add: bool) -> WinResult<()> {
		os_err!(unsafe{ consoleapi::SetConsoleCtrlHandler(handler, bool_to_num!(add)) });
		Ok(())
	}
	/**
	 Sets the position of the console cursor.
	 Returns an error if the position is not within the buffer bounds.
	
	 # Arguments
	 * `column` - The column of the new cursor position.
	 * `row` - The row of the new cursor position.
	
	 # Examples
	 Sets the cursor position to the start position of the console.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::set_cursor_position(0, 0).unwrap();
	 # }
	 ```
	 */
	pub fn set_cursor_position(column: u16, row: u16) -> WinResult<()> {
		let buffer_size = Console::get_buffer_size()?;

		if column >= buffer_size.x {
			throw_err!(ArgumentError::new("column", "column must be within the buffer bounds"));
		} else if row >= buffer_size.y {
			throw_err!(ArgumentError::new("row", "row must be within the buffer bounds"));
		}

		let coords = COORD {X: column as i16, Y: row as i16};
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::SetConsoleCursorPosition(handle, coords)
		});
		Ok(())
	}
	/**
	 Sets the size of the console cursor. Must be between 0 and 100 (inclusive).
	
	 # Arguments
	 * `size` - The new cursor size.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::set_cursor_size(50).unwrap();
	 # }
	 ```
	 */
	pub fn set_cursor_size(size: u8) -> WinResult<()> {
		if size > 100 {
			throw_err!(ArgumentError::new("size", "size must be in [0, 100] (inclusive)"));
		}

		let mut info = Console::get_cursor_info()?;
		info.dwSize = size as u32;
		Console::set_cursor_info(&info)?;
		Ok(())
	}
	/**
	 Sets the visibility of the console cursor.
	
	 # Arguments
	 * `visible` - True if the cursor should be visible, false otherwise.
	
	 # Examples
	 Hides the console cursor.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::set_cursor_visible(false).unwrap();
	 # }
	 ```
	 */
	pub fn set_cursor_visible(visible: bool) -> WinResult<()> {
		let mut info = Console::get_cursor_info()?;
		info.bVisible = bool_to_num!(visible);
		Console::set_cursor_info(&info)?;
		Ok(())
	}
	/**
	 Sets information about the console font.
	
	 # Arguments
	 * `font` - A ConsoleFont which contains the font information.
	
	 # Examples
	 Changes the console font to Consolas.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mut font = Console::get_font().unwrap();
	 font.name = "Consolas".to_string();
	 Console::set_font(&font).unwrap();
	 # }
	 ```
	 */
	pub fn set_font(font: &ConsoleFont) -> WinResult<()> {
		let mut info: CONSOLE_FONT_INFOEX = unsafe { mem::zeroed() };
		info.nFont = font.index as DWORD;
		info.dwFontSize = COORD { X: font.size.x as i16, Y: font.size.y as i16 };
		info.FontFamily = font.family as UINT;
		info.FontWeight = font.weight as UINT;
		info.FaceName = str_to_buf_w!(font.name, 32);
		Console::set_font_info_ex(&mut info, false)
	}
	/**
	 Sets the foreground color of the console.
	
	 # Arguments
	 * `color` - The color which will be assigned to the foreground.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 Console::set_foreground_color(ConsoleColor::Red).unwrap();
	 # }
	 ```
	 */
	pub fn set_foreground_color(color: ConsoleColor) -> WinResult<()> {
		let color = color as WORD;
		let current = Console::get_text_attributes()?;

		Console::set_text_attributes((current & 0xF0) | color)
	}
	/**
	 Sets information about console history settings.
	 
	 # Arguments
	 * `history` - The HistoryInfo to assign.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mut history_settings = Console::get_history_info().unwrap();
	 history_settings.duplicates_allowed = false;
	 Console::set_history_info(history_settings).unwrap();
	 # }
	 ```
	 */
	pub fn set_history_info(history: HistoryInfo) -> WinResult<()> {
		let mut info: CONSOLE_HISTORY_INFO = unsafe { mem::zeroed() };
		info.cbSize = mem::size_of::<CONSOLE_HISTORY_INFO>() as DWORD;
		info.HistoryBufferSize = history.size;
		info.NumberOfHistoryBuffers = history.number_of_buffers;
		info.dwFlags = bool_to_num!(!history.duplicates_allowed);

		os_err!(unsafe { wincon::SetConsoleHistoryInfo(&mut info) });
		Ok(())
	}
	/**
	 Sets the input code page to be used by the console.

	 # Arguments
	 * `page` - The code page to use.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, CodePage};
	 # fn main() {
	 Console::set_input_code_page(CodePage::utf_8).unwrap();
	 # }
	 ```
	 */
	pub fn set_input_code_page(page: CodePage) -> WinResult<()> {
		if page == CodePage::None || page == CodePage::Invalid {
			return Ok(()); // TODO: Maybe throw argument error instead?
		}
		let page: u16 = page.into();
		os_err!(unsafe { wincon::SetConsoleCP(page as u32) });
		Ok(())
	}
	/**
	 Sets settings related to console input.
	 Returns an error if the settings are invalid.
	
	 # Arguments
	 * `settings` - Settings to assign to the console input.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mut mode = Console::get_input_mode().unwrap();
	 mode.InsertMode = true;
	 Console::set_input_mode(mode).unwrap();
	 # }
	 ```
	 */
	pub fn set_input_mode(settings: InputSettings) -> WinResult<()> {
		if settings.EchoInput && !settings.LineInput {
			throw_err!(ArgumentError::new("settings", "disabling LineInput requires EchoInput to be disabled"));
		}
		let mode: u32 = settings.into();
		Console::set_mode(STDIN, mode)?;
		Ok(())
	}
	/**
	 Sets the output code page to be used by the console.

	 # Arguments
	 * `page` - The code page to use.

	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, CodePage};
	 # fn main() {
	 Console::set_output_code_page(CodePage::IBM437).unwrap();
	 # }
	 ```
	 */
	pub fn set_output_code_page(page: CodePage) -> WinResult<()> {
		if page == CodePage::None || page == CodePage::Invalid {
			return Ok(());
		}
		let page: u16 = page.into();
		os_err!(unsafe { wincon::SetConsoleOutputCP(page as u32) });
		Ok(())
	}
	/**
	 Sets settings related to console output.
	
	 # Arguments
	 * `settings` - Settings to assign to the console output.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 let mut mode = Console::get_output_mode().unwrap();
	 mode.WrapAtEol = false;
	 mode.DisableNewlineAutoReturn = true;
	 Console::set_output_mode(mode).unwrap();
	 # }
	 ```
	 */
	pub fn set_output_mode(settings: OutputSettings) -> WinResult<()> {
		let mode: u32 = settings.into();
		Console::set_mode(STDOUT, mode)
	}
	/**
	 Sets the state of the console to a ConsoleState.
	
	 # Arguments
	 * `state` - A ConsoleState containing state information.
	 * `clear` - Should the console be cleared before writing to the output?
	 * `write_output` - Should the stored text be written to the output?
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 println!("Message 1.");
	 let state = Console::get_state(true, false).unwrap();
	 println!("Message 2.");
	 Console::set_state(&state, true, true).unwrap();
	 println!("Great Scott!");
	 # }
	 ```
	 */
	pub fn set_state(state: &ConsoleState, clear: bool, write_output: bool) -> WinResult<()> {
		Console::set_background_color(state.background_color)?;
		Console::set_color_mapping(&state.color_mapping)?;
		Console::set_cursor_size(state.cursor_size)?;
		Console::set_foreground_color(state.foreground_color)?;
		Console::set_input_code_page(state.input_code_page)?;
		Console::set_output_code_page(state.output_code_page)?;
		Console::set_input_mode(state.input_mode)?;
		Console::set_output_mode(state.output_mode)?;
		Console::set_font(&state.font)?;
		Console::set_title(&state.title)?;

		if clear { Console::clear()?; }
		if write_output {
			Console::write_output_colors(&state.output_colors, 0, 0)?;
			Console::write_output(&state.output, 0, 0)?;
		}
		Console::set_cursor_position(state.cursor_position.x, state.cursor_position.y)?;
		Console::set_buffer_size(state.buffer_size.x, state.buffer_size.y)?;
		Ok(())
	}
	/**
	 Sets the title of the console window.
	
	 # Arguments
	 * `title` - The string to use as the title.
	
	 # Examples
	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::set_title("My Console").unwrap();
	 # }
	 ```
	 */
	pub fn set_title(title: &str) -> WinResult<()> {
		let mut buffer = str_to_buf!(title, MAX_PATH);
		os_err!(unsafe {
			let buffer_p = &mut buffer[0] as *mut CHAR;
			wincon::SetConsoleTitleA(buffer_p)
		});

		Ok(())
	}
	/**
	 Writes characters to the output at a specified position, and returns the
	 number of cells which were written to. Returns an error if the position is not within the buffer bounds.  
	 Note that this method writes characters  _directly_ to the output buffer
	 (i.e., newline characters do not move output to the next line,
	 but instead write the newline character).
	
	 # Arguments
	 * `string` - The string to write to the output.
	 * `column` - The column at which writing will begin.
	 * `row` - The row at which writing will begin.
	
	 # Examples
	 Writes `"Hello, world!"` on the 10th row starting at the 10th column.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::Console;
	 # fn main() {
	 Console::write_output("Hello, world!", 10, 10).unwrap();
	 # }
	 ```
	 */
	pub fn write_output(string: &str, column: u16, row: u16) -> WinResult<u32> {
		let buffer_size = Console::get_buffer_size()?;

		if column >= buffer_size.x {
			throw_err!(ArgumentError::new("column", "column must be within the buffer"));
		} else if row >= buffer_size.y {
			throw_err!(ArgumentError::new("row", "row must be within the buffer"));
		}

		let mut num: DWORD = 0;
		let coords = COORD { X: column as i16, Y: row as i16 };
		let chars: Box<[CHAR]> = str_to_buf!(string);
		let length = chars.len() as DWORD;
		if length == 0 { return Ok(0); }

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let chars_p = &(*chars)[0] as *const CHAR;
			wincon::WriteConsoleOutputCharacterA(handle, chars_p, length, coords, &mut num)
		});

		Ok(num)
	}
	/**
	 Changes the output colors starting at a specified position, and returns the
	 number of cells which were written to.
	 Returns an error if the position is not within the buffer bounds.
	
	 # Arguments
	 * `colors` - The colors to write to the console. The first item in each tuple is the foreground color,
	 and the second is the background color.
	 * `column` - The column at which writing will begin.
	 * `row` - The row at which writing will begin.
	
	 # Examples
	 Writes colors to the console starting at <0, 0>.

	 ```
	 # extern crate winconsole;
	 # use winconsole::console::{Console, ConsoleColor};
	 # fn main() {
	 let colors = vec![
		(ConsoleColor::Red, ConsoleColor::Blue),
		(ConsoleColor::Blue, ConsoleColor::Red),
		(ConsoleColor::Red, ConsoleColor::Blue),
		(ConsoleColor::Blue, ConsoleColor::Red)
	 ];
	 Console::write_output_colors(&colors, 0, 0).unwrap();
	 # }
	 ```
	 */
	pub fn write_output_colors(colors: &Vec<(ConsoleColor, ConsoleColor)>, column: u16, row: u16) -> WinResult<u32> {
		let buffer_size = Console::get_buffer_size()?;

		if column >= buffer_size.x {
			throw_err!(ArgumentError::new("column", "column must be within the buffer"));
		} else if row >= buffer_size.y {
			throw_err!(ArgumentError::new("row", "row must be within the buffer"));
		}

		let mut num: DWORD = 0;
		let coords = COORD { X: column as i16, Y: row as i16 };
		let length = colors.len() as DWORD;
		if length == 0 { return Ok(0); }

		let attrs: Box<[WORD]> = {
			let mut res: Vec<WORD> = colors.iter()
				.map(|&(ref fg, ref bg)| 
					(fg.get_value() | ((bg.get_value()) << 4)) as WORD)
				.collect();
			res.into_boxed_slice()
		};

		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let attrs_p = &(*attrs)[0] as *const WORD;
			wincon::WriteConsoleOutputAttribute(handle, attrs_p, length, coords, &mut num)
		});

		Ok(num)
	}

	fn fill_attributes(attributes: WORD, length: DWORD, coords: COORD) -> WinResult<DWORD> {
		let mut num: DWORD = 0;
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::FillConsoleOutputAttribute(handle, attributes, length, coords, &mut num)
		});
		Ok(num)
	}
	fn fill_char(character: CHAR, length: DWORD, coords: COORD) -> WinResult<DWORD> {
		let mut num: DWORD = 0;
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::FillConsoleOutputCharacterA(handle, character, length, coords, &mut num)
		});
		Ok(num)
	}
	fn get_code_pages(flags: u32) -> WinResult<Vec<CodePage>> {
		unsafe extern "system" fn enum_pages(ptr: *mut i8) -> i32 {
			let mut identifier = String::new();
			let mut offset = 0;
			loop {
				let chr = *ptr.offset(offset) as u8 as char;
				if chr == '\0' { break; }
				identifier.push(chr);
				offset += 1;
			}
			match identifier.parse::<u16>() {
				Ok(id) => {
					let cp = CodePage::from(id);
					if cp != CodePage::Invalid {
						match PAGES.lock() {
							Ok(mut pages) => pages.push(cp),
							Err(_) => return 0
						}
					}
				},
				Err(_) => ()
			}

			return 1;
		}

		os_err!(unsafe {
			winnls::EnumSystemCodePagesA(Some(enum_pages), flags)
		});

		match PAGES.lock() {
			Ok(mut pages) => {
				let ret = pages.clone();
				pages.clear();
				return Ok(ret);
			},
			Err(err) => {
				let mut pages = err.into_inner();
				let ret = pages.clone();
				pages.clear();
				return Ok(ret);
			}
		}
	}
	fn get_cursor_info() -> WinResult<CONSOLE_CURSOR_INFO> {
		let mut info = unsafe { mem::zeroed() };
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::GetConsoleCursorInfo(handle, &mut info)
		});
		Ok(info)
	}
	fn get_font_info_ex(maximum: bool) -> WinResult<CONSOLE_FONT_INFOEX> {
		let mut info: CONSOLE_FONT_INFOEX = unsafe { mem::zeroed() };
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			info.cbSize = mem::size_of::<CONSOLE_FONT_INFOEX>() as DWORD;
			wincon::GetCurrentConsoleFontEx(handle, bool_to_num!(maximum), &mut info)
		});
		Ok(info)
	}
	fn get_mode(handle_id: DWORD) -> WinResult<DWORD> {
		let mut num: DWORD = 0;
		os_err!(unsafe {
			let handle = handle!(handle_id);
			consoleapi::GetConsoleMode(handle, &mut num)
		});
		Ok(num)
	}
	fn get_screen_buffer_info() -> WinResult<CONSOLE_SCREEN_BUFFER_INFO> {
		let mut csbi = unsafe { mem::zeroed() };
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::GetConsoleScreenBufferInfo(handle, &mut csbi)
		});
		Ok(csbi)
	}
	fn get_screen_buffer_info_ex() -> WinResult<CONSOLE_SCREEN_BUFFER_INFOEX> {
		let mut csbi: CONSOLE_SCREEN_BUFFER_INFOEX = unsafe { mem::zeroed() };
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			csbi.cbSize = mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as DWORD;
			wincon::GetConsoleScreenBufferInfoEx(handle, &mut csbi)
		});
		Ok(csbi)
	}
	fn get_text_attributes() -> WinResult<WORD> {
		let csbi = Console::get_screen_buffer_info()?;
		Ok(csbi.wAttributes)
	}
	fn set_cursor_info(value: &CONSOLE_CURSOR_INFO) -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let value_p = value as *const CONSOLE_CURSOR_INFO;
			wincon::SetConsoleCursorInfo(handle, value_p)
		});
		Ok(())
	}
	fn set_font_info_ex(value: &mut CONSOLE_FONT_INFOEX, maximum: bool) -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			value.cbSize = mem::size_of::<CONSOLE_FONT_INFOEX>() as DWORD;
			let value_p = value as *mut CONSOLE_FONT_INFOEX;
			wincon::SetCurrentConsoleFontEx(handle, bool_to_num!(maximum), value_p)
		});
		Ok(())
	}
	fn set_mode(handle_id: DWORD, value: DWORD) -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(handle_id);
			consoleapi::SetConsoleMode(handle, value)
		});
		Ok(())
	}
	fn set_screen_buffer_info_ex(value: &mut CONSOLE_SCREEN_BUFFER_INFOEX) -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			let value_p = value as *mut CONSOLE_SCREEN_BUFFER_INFOEX;
			wincon::SetConsoleScreenBufferInfoEx(handle, value_p)
		});
		Ok(())
	}
	fn set_text_attributes(value: WORD) -> WinResult<()> {
		os_err!(unsafe {
			let handle = handle!(STDOUT);
			wincon::SetConsoleTextAttribute(handle, value)
		});
		Ok(())
	}
}
