use std::{fmt, fmt::{Display, Formatter}};

mod argument_error;
mod console_color;
mod console_font;
mod console_state;
mod input_settings;
mod output_settings;
mod selection_info;

pub use self::argument_error::ArgumentError;
pub use self::console_color::ConsoleColor;
pub use self::console_font::ConsoleFont;
pub use self::console_state::ConsoleState;
pub use self::input_settings::InputSettings;
pub use self::output_settings::OutputSettings;
pub use self::selection_info::SelectionInfo;
