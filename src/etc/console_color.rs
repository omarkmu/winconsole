use super::*;

/// Represents supported console colors.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConsoleColor {
	/// The color black; defaults to #000000.
	Black = 0x0,
	/// The color dark blue; defaults to #000080.
	DarkBlue = 0x1,
	/// The color dark green; defaults to #008000.
	DarkGreen = 0x2,
	/// The color teal; defaults to #008080.
	Teal = 0x3,
	/// The color dark red; defaults to #800000.
	DarkRed = 0x4,
	/// The color magenta; defaults to #800080.
	Magenta = 0x5,
	/// The color dark yellow; defaults to #808000.
	DarkYellow = 0x6,
	/// The color gray; defaults to #C0C0C0.
	Gray = 0x7,
	/// The color dark gray; defaults to #808080.
	DarkGray = 0x8,
	/// The color blue; defaults to #0000FF.
	Blue = 0x9,
	/// The color green; defaults to #00FF00.
	Green = 0xA,
	/// The color aqua; defaults to #00FFFF.
	Aqua = 0xB,
	/// The color red; defaults to #FF0000.
	Red = 0xC,
	/// The color pink; defaults to #FF00FF.
	Pink = 0xD,
	/// The color yellow; defaults to #FFFF00.
	Yellow = 0xE,
	/// The color white; defaults to #FFFFFF.
	White = 0xF
}

impl ConsoleColor {
	/**
	Returns the integral value of the ConsoleColor.
	*/
	pub fn get_value(&self) -> u8 {
		*self as u8
	}
}

impl From<u16> for ConsoleColor {
	fn from(value: u16) -> ConsoleColor {
		match value {
			0x0 => ConsoleColor::Black,
			0x1 => ConsoleColor::DarkBlue,
			0x2 => ConsoleColor::DarkGreen,
			0x3 => ConsoleColor::Teal,
			0x4 => ConsoleColor::DarkRed,
			0x5 => ConsoleColor::Magenta,
			0x6 => ConsoleColor::DarkYellow,
			0x7 => ConsoleColor::Gray,
			0x8 => ConsoleColor::DarkGray,
			0x9 => ConsoleColor::Blue,
			0xA => ConsoleColor::Green,
			0xB => ConsoleColor::Aqua,
			0xC => ConsoleColor::Red,
			0xD => ConsoleColor::Pink,
			0xE => ConsoleColor::Yellow,
			_ => ConsoleColor::White
		}
	}
}

impl Into<u16> for ConsoleColor {
	fn into(self) -> u16 {
		self as u16
	}
}

impl Display for ConsoleColor {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let name = match *self {
			ConsoleColor::Black => "Black",
			ConsoleColor::DarkBlue => "DarkBlue",
			ConsoleColor::DarkGreen => "DarkGreen",
			ConsoleColor::Teal => "Teal",
			ConsoleColor::DarkRed => "DarkRed",
			ConsoleColor::Magenta => "Magenta",
			ConsoleColor::DarkYellow => "DarkYellow",
			ConsoleColor::Gray => "Gray",
			ConsoleColor::DarkGray => "DarkGray",
			ConsoleColor::Blue => "Blue",
			ConsoleColor::Green => "Green",
			ConsoleColor::Aqua => "Aqua",
			ConsoleColor::Red => "Red",
			ConsoleColor::Pink => "Pink",
			ConsoleColor::Yellow => "Yellow",
			ConsoleColor::White => "White"
		};
		write!(f, "ConsoleColor::{}", name)
	}
}
