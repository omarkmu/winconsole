use cgmath::Vector2;
use rgb::RGB8;
use super::{ConsoleColor, ConsoleFont, InputSettings, OutputSettings};

/// Represents the state of the console.
#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleState {
	/// The background color of the console.
	pub background_color: ConsoleColor,
	/// The console's buffer size.
	pub buffer_size: Vector2<u16>,
	/// The color mapping of the console.
	pub color_mapping: [RGB8; 16],
	/// The console cursor position.
	pub cursor_position: Vector2<u16>,
	/// The console cursor size.
	pub cursor_size: u8,
	/// The visibility of the console cursor.
	pub cursor_visible: bool,
	/// The console font information.
	pub font: ConsoleFont,
	/// The foreground color of the console.
	pub foreground_color: ConsoleColor,
	/// The console input mode.
	pub input_mode: InputSettings,
	/// The console's output contents.
	pub output: String,
	/// The colors of the console's output contents.
	pub output_colors: Vec<(ConsoleColor, ConsoleColor)>,
	/// The console output mode.
	pub output_mode: OutputSettings,
	/// The console window title.
	pub title: String
}

impl ConsoleState {
	/**
	 Returns an empty ConsoleState object.
	 */
	pub fn new() -> ConsoleState {
		ConsoleState {
			background_color: ConsoleColor::Black,
			buffer_size: Vector2::new(0, 0),
			color_mapping: [RGB8 {r: 0, g: 0, b: 0}; 16],
			cursor_position: Vector2::new(0, 0),
			cursor_size: 0,
			cursor_visible: false,
			font: ConsoleFont::new(),
			foreground_color: ConsoleColor::Black,
			input_mode: InputSettings::new(),
			output: String::new(),
			output_colors: Vec::new(),
			output_mode: OutputSettings::new(),
			title: String::new()
		}
	}
}
