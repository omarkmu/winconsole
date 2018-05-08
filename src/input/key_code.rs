use super::*;

/// Represents a virtual key code.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyCode {
	/// Default value.
	None = 0x0,
	/// Left mouse button.
	LButton = 0x01,
	/// Right mouse button.
	RButton = 0x02,
	/// Control-break processing.
	Cancel = 0x03,
	/// Middle mouse button (three-button mouse).
	MButton = 0x04,
	/// X1 mouse button.
	XButton1 = 0x05,
	/// X2 mouse button.
	XButton2 = 0x06,
	/// Backspace key.
	Backspace = 0x08,
	/// Tab key.
	Tab = 0x09,
	/// Clear key.
	Clear = 0x0c,
	/// Enter key.
	Return = 0x0d,
	/// Shift key.
	Shift = 0x10,
	/// Ctrl key.
	Control = 0x11,
	/// Alt key.
	Menu = 0x12,
	/// Pause key.
	Pause = 0x13,
	/// Caps Lock key.
	Capital = 0x14,
	/// Ime Kana/Hangul Mode.
	KanaHangul = 0x15,
	/// Ime Junja Mode.
	Junja = 0x17,
	/// Ime Final Mode.
	Final = 0x18,
	/// Ime Hanja/Kanji Mode.
	HanjaKanji = 0x19,
	/// Escape key.
	Escape = 0x1b,
	/// Ime Convert.
	Convert = 0x1c,
	/// Ime Nonconvert.
	NonConvert = 0x1d,
	/// Ime Accept.
	Accept = 0x1e,
	/// Ime Mode Change Request.
	ModeChange = 0x1f,
	/// Spacebar.
	Space = 0x20,
	/// Page Up key.
	PageUp = 0x21,
	/// Page Down key.
	PageDown = 0x22,
	/// End key.
	End = 0x23,
	/// Home key.
	Home = 0x24,
	/// Left Arrow key.
	Left = 0x25,
	/// Up Arrow key.
	Up = 0x26,
	/// Right Arrow key.
	Right = 0x27,
	/// Down Arrow key.
	Down = 0x28,
	/// Select key.
	Select = 0x29,
	/// Print key.
	Print = 0x2a,
	/// Execute key.
	Execute = 0x2b,
	/// Print Screen key.
	Snapshot = 0x2c,
	/// Ins key.
	Insert = 0x2d,
	/// Del key.
	Delete = 0x2e,
	/// Help key.
	Help = 0x2f,
	/// 0 key.
	Zero = 0x30,
	/// 1 key.
	One = 0x31,
	/// 2 key.
	Two = 0x32,
	/// 3 key.
	Three = 0x33,
	/// 4 key.
	Four = 0x34,
	/// 5 key.
	Five = 0x35,
	/// 6 key.
	Six = 0x36,
	/// 7 key.
	Seven = 0x37,
	/// 8 key.
	Eight = 0x38,
	/// 9 key.
	Nine = 0x39,
	/// A key.
	A = 0x41,
	/// B key.
	B = 0x42,
	/// C key.
	C = 0x43,
	/// D key.
	D = 0x44,
	/// E key.
	E = 0x45,
	/// F key.
	F = 0x46,
	/// G key.
	G = 0x47,
	/// H key.
	H = 0x48,
	/// I key.
	I = 0x49,
	/// J key.
	J = 0x4a,
	/// K key.
	K = 0x4b,
	/// L key.
	L = 0x4c,
	/// M key.
	M = 0x4d,
	/// N key.
	N = 0x4e,
	/// O key.
	O = 0x4f,
	/// P key.
	P = 0x50,
	/// Q key.
	Q = 0x51,
	/// R key.
	R = 0x52,
	/// S key.
	S = 0x53,
	/// T key.
	T = 0x54,
	/// U key.
	U = 0x55,
	/// V key.
	V = 0x56,
	/// W key.
	W = 0x57,
	/// X key.
	X = 0x58,
	/// Y key.
	Y = 0x59,
	/// Z key.
	Z = 0x5a,
	/// Left Windows Key (natural Keyboard).
	LWin = 0x5b,
	/// Right Windows Key (natural Keyboard).
	RWin = 0x5c,
	/// Applications Key (natural Keyboard).
	Apps = 0x5d,
	/// Computer Sleep key.
	Sleep = 0x5f,
	/// Numeric Keypad 0 key.
	Numpad0 = 0x60,
	/// Numeric Keypad 1 key.
	Numpad1 = 0x61,
	/// Numeric Keypad 2 key.
	Numpad2 = 0x62,
	/// Numeric Keypad 3 key.
	Numpad3 = 0x63,
	/// Numeric Keypad 4 key.
	Numpad4 = 0x64,
	/// Numeric Keypad 5 key.
	Numpad5 = 0x65,
	/// Numeric Keypad 6 key.
	Numpad6 = 0x66,
	/// Numeric Keypad 7 key.
	Numpad7 = 0x67,
	/// Numeric Keypad 8 key.
	Numpad8 = 0x68,
	/// Numeric Keypad 9 key.
	Numpad9 = 0x69,
	/// Multiply key.
	Multiply = 0x6a,
	/// Add key.
	Add = 0x6b,
	/// Separator key.
	Separator = 0x6c,
	/// Subtract key.
	Subtract = 0x6d,
	/// Decimal key.
	Decimal = 0x6e,
	/// Divide key.
	Divide = 0x6f,
	/// F1 key.
	F1 = 0x70,
	/// F2 key.
	F2 = 0x71,
	/// F3 key.
	F3 = 0x72,
	/// F4 key.
	F4 = 0x73,
	/// F5 key.
	F5 = 0x74,
	/// F6 key.
	F6 = 0x75,
	/// F7 key.
	F7 = 0x76,
	/// F8 key.
	F8 = 0x77,
	/// F9 key.
	F9 = 0x78,
	/// F10 key.
	F10 = 0x79,
	/// F11 key.
	F11 = 0x7a,
	/// F12 key.
	F12 = 0x7b,
	/// F13 key.
	F13 = 0x7c,
	/// F14 key.
	F14 = 0x7d,
	/// F15 key.
	F15 = 0x7e,
	/// F16 key.
	F16 = 0x7f,
	/// F17 key.
	F17 = 0x80,
	/// F18 key.
	F18 = 0x81,
	/// F19 key.
	F19 = 0x82,
	/// F20 key.
	F20 = 0x83,
	/// F21 key.
	F21 = 0x84,
	/// F22 key.
	F22 = 0x85,
	/// F23 key.
	F23 = 0x86,
	/// F24 key.
	F24 = 0x87,
	/// Num Lock key.
	NumLock = 0x90,
	/// Scroll Lock key.
	Scroll = 0x91,
	/// Left Shift key.
	LShift = 0xa0,
	/// Right Shift key.
	RShift = 0xa1,
	/// Left Control key.
	LControl = 0xa2,
	/// Right Control key.
	RControl = 0xa3,
	/// Left Menu key.
	LMenu = 0xa4,
	/// Right Menu key.
	RMenu = 0xa5,
	/// Browser Back key.
	BrowserBack = 0xa6,
	/// Browser Forward key.
	BrowserForward = 0xa7,
	/// Browser Refresh key.
	BrowserRefresh = 0xa8,
	/// Browser Stop key.
	BrowserStop = 0xa9,
	/// Browser Search key.
	BrowserSearch = 0xaa,
	/// Browser Favorites key.
	BrowserFavorites = 0xab,
	/// Browser Start And Home key.
	BrowserHome = 0xac,
	/// Volume Mute key.
	VolumeMute = 0xad,
	/// Volume Down key.
	VolumeDown = 0xae,
	/// Volume Up key.
	VolumeUp = 0xaf,
	/// Next Track key.
	MediaNextTrack = 0xb0,
	/// Previous Track key.
	MediaPrevTrack = 0xb1,
	/// Stop Media key.
	MediaStop = 0xb2,
	/// Play/pause Media key.
	MediaPlayPause = 0xb3,
	/// Start Mail key.
	LaunchMail = 0xb4,
	/// Select Media key.
	LaunchMediaSelect = 0xb5,
	/// Start Application 1 key.
	LaunchApp1 = 0xb6,
	/// Start Application 2 key.
	LaunchApp2 = 0xb7,
	/// Used for miscellaneous characters; it can vary by keyboard.
	Oem1 = 0xba,
	/// The '+' key.
	Plus = 0xbb,
	/// The ',' key.
	Comma = 0xbc,
	/// The '-' key.
	Minus = 0xbd,
	/// The '.' key.
	Period = 0xbe,
	/// Used for miscellaneous characters; it can vary by keyboard.
	Oem2 = 0xbf,
	/// Used for miscellaneous characters; it can vary by keyboard.
	Oem3 = 0xc0,
	/// For the U.S. standard keyboard, the '[{' key.
	Oem4 = 0xdb,
	/// For the U.S. standard keyboard, the '\|' key.
	Oem5 = 0xdc,
	/// For the U.S. standard keyboard, the ']}' key.
	Oem6 = 0xdd,
	/// For the U.S. standard keyboard, the 'single-quote/double-quote' key.
	Oem7 = 0xde,
	/// Used for miscellaneous characters; it can vary by keyboard.
	Oem8 = 0xdf,
	/// Either the angle bracket key or the backslash key on the RT 102-key keyboard.
	Oem102 = 0xe2,
	/// Ime Process key.
	ProcessKey = 0xe5,
	/// Attn key.
	Attn = 0xf6,
	/// Crsel key.
	CrSel = 0xf7,
	/// Exsel key.
	ExSel = 0xf8,
	/// Erase Eof key.
	ErEOF = 0xf9,
	/// Play key.
	Play = 0xfa,
	/// Zoom key.
	Zoom = 0xfb,
	/// Reserved.
	NoName = 0xfc,
	/// Pa1 key.
	PA1 = 0xfd,
	/// Clear key.
	OEMClear = 0xfe,
	/// Returned from keys with no mapping.
	NoMapping = 0xff
}

impl KeyCode {
	/**
	Returns the integral value of the KeyCode.
	*/
	pub fn get_value(&self) -> u32 {
		*self as u32
	}
}

impl From<u8> for KeyCode {
	fn from(value: u8) -> KeyCode {
		match value {
			0x0 => KeyCode::None,
			0x01 => KeyCode::LButton,
			0x02 => KeyCode::RButton,
			0x03 => KeyCode::Cancel,
			0x04 => KeyCode::MButton,
			0x05 => KeyCode::XButton1,
			0x06 => KeyCode::XButton2,
			0x08 => KeyCode::Backspace,
			0x09 => KeyCode::Tab,
			0x0c => KeyCode::Clear,
			0x0d => KeyCode::Return,
			0x10 => KeyCode::Shift,
			0x11 => KeyCode::Control,
			0x12 => KeyCode::Menu,
			0x13 => KeyCode::Pause,
			0x14 => KeyCode::Capital,
			0x15 => KeyCode::KanaHangul,
			0x17 => KeyCode::Junja,
			0x18 => KeyCode::Final,
			0x19 => KeyCode::HanjaKanji,
			0x1b => KeyCode::Escape,
			0x1c => KeyCode::Convert,
			0x1d => KeyCode::NonConvert,
			0x1e => KeyCode::Accept,
			0x1f => KeyCode::ModeChange,
			0x20 => KeyCode::Space,
			0x21 => KeyCode::PageUp,
			0x22 => KeyCode::PageDown,
			0x23 => KeyCode::End,
			0x24 => KeyCode::Home,
			0x25 => KeyCode::Left,
			0x26 => KeyCode::Up,
			0x27 => KeyCode::Right,
			0x28 => KeyCode::Down,
			0x29 => KeyCode::Select,
			0x2a => KeyCode::Print,
			0x2b => KeyCode::Execute,
			0x2c => KeyCode::Snapshot,
			0x2d => KeyCode::Insert,
			0x2e => KeyCode::Delete,
			0x2f => KeyCode::Help,
			0x30 => KeyCode::Zero,
			0x31 => KeyCode::One,
			0x32 => KeyCode::Two,
			0x33 => KeyCode::Three,
			0x34 => KeyCode::Four,
			0x35 => KeyCode::Five,
			0x36 => KeyCode::Six,
			0x37 => KeyCode::Seven,
			0x38 => KeyCode::Eight,
			0x39 => KeyCode::Nine,
			0x41 => KeyCode::A,
			0x42 => KeyCode::B,
			0x43 => KeyCode::C,
			0x44 => KeyCode::D,
			0x45 => KeyCode::E,
			0x46 => KeyCode::F,
			0x47 => KeyCode::G,
			0x48 => KeyCode::H,
			0x49 => KeyCode::I,
			0x4a => KeyCode::J,
			0x4b => KeyCode::K,
			0x4c => KeyCode::L,
			0x4d => KeyCode::M,
			0x4e => KeyCode::N,
			0x4f => KeyCode::O,
			0x50 => KeyCode::P,
			0x51 => KeyCode::Q,
			0x52 => KeyCode::R,
			0x53 => KeyCode::S,
			0x54 => KeyCode::T,
			0x55 => KeyCode::U,
			0x56 => KeyCode::V,
			0x57 => KeyCode::W,
			0x58 => KeyCode::X,
			0x59 => KeyCode::Y,
			0x5a => KeyCode::Z,
			0x5b => KeyCode::LWin,
			0x5c => KeyCode::RWin,
			0x5d => KeyCode::Apps,
			0x5f => KeyCode::Sleep,
			0x60 => KeyCode::Numpad0,
			0x61 => KeyCode::Numpad1,
			0x62 => KeyCode::Numpad2,
			0x63 => KeyCode::Numpad3,
			0x64 => KeyCode::Numpad4,
			0x65 => KeyCode::Numpad5,
			0x66 => KeyCode::Numpad6,
			0x67 => KeyCode::Numpad7,
			0x68 => KeyCode::Numpad8,
			0x69 => KeyCode::Numpad9,
			0x6a => KeyCode::Multiply,
			0x6b => KeyCode::Add,
			0x6c => KeyCode::Separator,
			0x6d => KeyCode::Subtract,
			0x6e => KeyCode::Decimal,
			0x6f => KeyCode::Divide,
			0x70 => KeyCode::F1,
			0x71 => KeyCode::F2,
			0x72 => KeyCode::F3,
			0x73 => KeyCode::F4,
			0x74 => KeyCode::F5,
			0x75 => KeyCode::F6,
			0x76 => KeyCode::F7,
			0x77 => KeyCode::F8,
			0x78 => KeyCode::F9,
			0x79 => KeyCode::F10,
			0x7a => KeyCode::F11,
			0x7b => KeyCode::F12,
			0x7c => KeyCode::F13,
			0x7d => KeyCode::F14,
			0x7e => KeyCode::F15,
			0x7f => KeyCode::F16,
			0x80 => KeyCode::F17,
			0x81 => KeyCode::F18,
			0x82 => KeyCode::F19,
			0x83 => KeyCode::F20,
			0x84 => KeyCode::F21,
			0x85 => KeyCode::F22,
			0x86 => KeyCode::F23,
			0x87 => KeyCode::F24,
			0x90 => KeyCode::NumLock,
			0x91 => KeyCode::Scroll,
			0xa0 => KeyCode::LShift,
			0xa1 => KeyCode::RShift,
			0xa2 => KeyCode::LControl,
			0xa3 => KeyCode::RControl,
			0xa4 => KeyCode::LMenu,
			0xa5 => KeyCode::RMenu,
			0xa6 => KeyCode::BrowserBack,
			0xa7 => KeyCode::BrowserForward,
			0xa8 => KeyCode::BrowserRefresh,
			0xa9 => KeyCode::BrowserStop,
			0xaa => KeyCode::BrowserSearch,
			0xab => KeyCode::BrowserFavorites,
			0xac => KeyCode::BrowserHome,
			0xad => KeyCode::VolumeMute,
			0xae => KeyCode::VolumeDown,
			0xaf => KeyCode::VolumeUp,
			0xb0 => KeyCode::MediaNextTrack,
			0xb1 => KeyCode::MediaPrevTrack,
			0xb2 => KeyCode::MediaStop,
			0xb3 => KeyCode::MediaPlayPause,
			0xb4 => KeyCode::LaunchMail,
			0xb5 => KeyCode::LaunchMediaSelect,
			0xb6 => KeyCode::LaunchApp1,
			0xb7 => KeyCode::LaunchApp2,
			0xba => KeyCode::Oem1,
			0xbb => KeyCode::Plus,
			0xbc => KeyCode::Comma,
			0xbd => KeyCode::Minus,
			0xbe => KeyCode::Period,
			0xbf => KeyCode::Oem2,
			0xc0 => KeyCode::Oem3,
			0xdb => KeyCode::Oem4,
			0xdc => KeyCode::Oem5,
			0xdd => KeyCode::Oem6,
			0xde => KeyCode::Oem7,
			0xdf => KeyCode::Oem8,
			0xe2 => KeyCode::Oem102,
			0xe5 => KeyCode::ProcessKey,
			0xf6 => KeyCode::Attn,
			0xf7 => KeyCode::CrSel,
			0xf8 => KeyCode::ExSel,
			0xf9 => KeyCode::ErEOF,
			0xfa => KeyCode::Play,
			0xfb => KeyCode::Zoom,
			0xfc => KeyCode::NoName,
			0xfd => KeyCode::PA1,
			0xfe => KeyCode::OEMClear,
			_ => KeyCode::NoMapping
		}
	}
}

impl Into<u8> for KeyCode {
	fn into(self) -> u8 {
		self as u8
	}
}

impl Display for KeyCode {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let name = match *self {
			KeyCode::None => "None",
			KeyCode::LButton => "LButton",
			KeyCode::RButton => "RButton",
			KeyCode::Cancel => "Cancel",
			KeyCode::MButton => "MButton",
			KeyCode::XButton1 => "XButton1",
			KeyCode::XButton2 => "XButton2",
			KeyCode::Backspace => "Backspace",
			KeyCode::Tab => "Tab",
			KeyCode::Clear => "Clear",
			KeyCode::Return => "Return",
			KeyCode::Shift => "Shift",
			KeyCode::Control => "Control",
			KeyCode::Menu => "Menu",
			KeyCode::Pause => "Pause",
			KeyCode::Capital => "Capital",
			KeyCode::KanaHangul => "KanaHangul",
			KeyCode::Junja => "Junja",
			KeyCode::Final => "Final",
			KeyCode::HanjaKanji => "HanjaKanji",
			KeyCode::Escape => "Escape",
			KeyCode::Convert => "Convert",
			KeyCode::NonConvert => "NonConvert",
			KeyCode::Accept => "Accept",
			KeyCode::ModeChange => "ModeChange",
			KeyCode::Space => "Space",
			KeyCode::PageUp => "PageUp",
			KeyCode::PageDown => "PageDown",
			KeyCode::End => "End",
			KeyCode::Home => "Home",
			KeyCode::Left => "Left",
			KeyCode::Up => "Up",
			KeyCode::Right => "Right",
			KeyCode::Down => "Down",
			KeyCode::Select => "Select",
			KeyCode::Print => "Print",
			KeyCode::Execute => "Execute",
			KeyCode::Snapshot => "Snapshot",
			KeyCode::Insert => "Insert",
			KeyCode::Delete => "Delete",
			KeyCode::Help => "Help",
			KeyCode::Zero => "Zero",
			KeyCode::One => "One",
			KeyCode::Two => "Two",
			KeyCode::Three => "Three",
			KeyCode::Four => "Four",
			KeyCode::Five => "Five",
			KeyCode::Six => "Six",
			KeyCode::Seven => "Seven",
			KeyCode::Eight => "Eight",
			KeyCode::Nine => "Nine",
			KeyCode::A => "A",
			KeyCode::B => "B",
			KeyCode::C => "C",
			KeyCode::D => "D",
			KeyCode::E => "E",
			KeyCode::F => "F",
			KeyCode::G => "G",
			KeyCode::H => "H",
			KeyCode::I => "I",
			KeyCode::J => "J",
			KeyCode::K => "K",
			KeyCode::L => "L",
			KeyCode::M => "M",
			KeyCode::N => "N",
			KeyCode::O => "O",
			KeyCode::P => "P",
			KeyCode::Q => "Q",
			KeyCode::R => "R",
			KeyCode::S => "S",
			KeyCode::T => "T",
			KeyCode::U => "U",
			KeyCode::V => "V",
			KeyCode::W => "W",
			KeyCode::X => "X",
			KeyCode::Y => "Y",
			KeyCode::Z => "Z",
			KeyCode::LWin => "LWin",
			KeyCode::RWin => "RWin",
			KeyCode::Apps => "Apps",
			KeyCode::Sleep => "Sleep",
			KeyCode::Numpad0 => "Numpad0",
			KeyCode::Numpad1 => "Numpad1",
			KeyCode::Numpad2 => "Numpad2",
			KeyCode::Numpad3 => "Numpad3",
			KeyCode::Numpad4 => "Numpad4",
			KeyCode::Numpad5 => "Numpad5",
			KeyCode::Numpad6 => "Numpad6",
			KeyCode::Numpad7 => "Numpad7",
			KeyCode::Numpad8 => "Numpad8",
			KeyCode::Numpad9 => "Numpad9",
			KeyCode::Multiply => "Multiply",
			KeyCode::Add => "Add",
			KeyCode::Separator => "Separator",
			KeyCode::Subtract => "Subtract",
			KeyCode::Decimal => "Decimal",
			KeyCode::Divide => "Divide",
			KeyCode::F1 => "F1",
			KeyCode::F2 => "F2",
			KeyCode::F3 => "F3",
			KeyCode::F4 => "F4",
			KeyCode::F5 => "F5",
			KeyCode::F6 => "F6",
			KeyCode::F7 => "F7",
			KeyCode::F8 => "F8",
			KeyCode::F9 => "F9",
			KeyCode::F10 => "F10",
			KeyCode::F11 => "F11",
			KeyCode::F12 => "F12",
			KeyCode::F13 => "F13",
			KeyCode::F14 => "F14",
			KeyCode::F15 => "F15",
			KeyCode::F16 => "F16",
			KeyCode::F17 => "F17",
			KeyCode::F18 => "F18",
			KeyCode::F19 => "F19",
			KeyCode::F20 => "F20",
			KeyCode::F21 => "F21",
			KeyCode::F22 => "F22",
			KeyCode::F23 => "F23",
			KeyCode::F24 => "F24",
			KeyCode::NumLock => "NumLock",
			KeyCode::Scroll => "Scroll",
			KeyCode::LShift => "LShift",
			KeyCode::RShift => "RShift",
			KeyCode::LControl => "LControl",
			KeyCode::RControl => "RControl",
			KeyCode::LMenu => "LMenu",
			KeyCode::RMenu => "RMenu",
			KeyCode::BrowserBack => "BrowserBack",
			KeyCode::BrowserForward => "BrowserForward",
			KeyCode::BrowserRefresh => "BrowserRefresh",
			KeyCode::BrowserStop => "BrowserStop",
			KeyCode::BrowserSearch => "BrowserSearch",
			KeyCode::BrowserFavorites => "BrowserFavorites",
			KeyCode::BrowserHome => "BrowserHome",
			KeyCode::VolumeMute => "VolumeMute",
			KeyCode::VolumeDown => "VolumeDown",
			KeyCode::VolumeUp => "VolumeUp",
			KeyCode::MediaNextTrack => "MediaNextTrack",
			KeyCode::MediaPrevTrack => "MediaPrevTrack",
			KeyCode::MediaStop => "MediaStop",
			KeyCode::MediaPlayPause => "MediaPlayPause",
			KeyCode::LaunchMail => "LaunchMail",
			KeyCode::LaunchMediaSelect => "LaunchMediaSelect",
			KeyCode::LaunchApp1 => "LaunchApp1",
			KeyCode::LaunchApp2 => "LaunchApp2",
			KeyCode::Oem1 => "Oem1",
			KeyCode::Plus => "Plus",
			KeyCode::Comma => "Comma",
			KeyCode::Minus => "Minus",
			KeyCode::Period => "Period",
			KeyCode::Oem2 => "Oem2",
			KeyCode::Oem3 => "Oem3",
			KeyCode::Oem4 => "Oem4",
			KeyCode::Oem5 => "Oem5",
			KeyCode::Oem6 => "Oem6",
			KeyCode::Oem7 => "Oem7",
			KeyCode::Oem8 => "Oem8",
			KeyCode::Oem102 => "Oem102",
			KeyCode::ProcessKey => "ProcessKey",
			KeyCode::Attn => "Attn",
			KeyCode::CrSel => "CrSel",
			KeyCode::ExSel => "ExSel",
			KeyCode::ErEOF => "ErEOF",
			KeyCode::Play => "Play",
			KeyCode::Zoom => "Zoom",
			KeyCode::NoName => "NoName",
			KeyCode::PA1 => "PA1",
			KeyCode::OEMClear => "OEMClear",
			_ => "NoMapping"
		};
		write!(f, "KeyCode::{}", name)
	}
}
