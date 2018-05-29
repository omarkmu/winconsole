macro_rules! bool_to_num {
	($x:expr) => (if $x { 1 } else { 0 });
}
macro_rules! buf {
	($size:expr) => {
		{
			let vec = vec![0; $size];
			vec.into_boxed_slice()
		}
	};
}
macro_rules! buf_to_str {
	($buf:expr) => {
		{
			let mut vec: Vec<u8> = vec![];
			for c in $buf.to_vec().iter() {
				if *c == 0 { break; }
				vec.push(*c as u8)
			}
			String::from_utf8(vec)?
		}
	}
}
#[cfg(feature = "input")]
macro_rules! buf_to_vec {
	($buf:expr, $len:expr) => {
		{
			let mut result = Vec::new();
			for i in 0usize..($len as usize) {
				let value = $buf[i];
				result.push(value);
			}
			result
		}
	}
}
/**
 Prints a colored message to the console.
 This has a side effect of flushing the console output.

 # Examples
 ```
 #[macro_use] extern crate winconsole;
 use winconsole::console::ConsoleColor;

 fn main() {
 	let thing = "world";
 	cprint!(ConsoleColor::Blue, "Hello, {}!", thing);
 	cprint!(ConsoleColor::Red, " Goodbye, world!");
 }
 ```
 */
#[macro_export]
macro_rules! cprint {
    ($color:expr, $($arg:tt)*) => {
		{
			use $crate::console;
			let old_color = console::get_foreground_color().unwrap();
			console::set_foreground_color($color).unwrap();
			print!($($arg)*);
			console::flush_output().unwrap();
			console::set_foreground_color(old_color).unwrap();
		}
	}
}
/**
 Prints a colored message to the console with a newline.
 This has a side effect of flushing the console output.

 # Examples
 ```
 #[macro_use] extern crate winconsole;
 use winconsole::console;
 use winconsole::console::ConsoleColor;

 fn main() {
 	let person = "Ada";
 	print!("Hello, ");
 	console::flush_output().unwrap();
 	cprintln!(ConsoleColor::Magenta, "{}.", person);
 	cprintln!(ConsoleColor::Blue, "How are you?");
 }
 ```
 */
#[macro_export]
macro_rules! cprintln {
    ($color: expr, $fmt:expr) => (cprint!($color, concat!($fmt, "\n")));
    ($color: expr, $fmt:expr, $($arg:tt)*) => (cprint!($color, concat!($fmt, "\n"), $($arg)*));
}
macro_rules! enumeration_internal {
	($(#[$attrs:meta])*
	$name:ident<$repr_type:ty, $type:ty> ($sname:expr) {
		@$default:expr,
        $($(#[$item_attrs:meta])* $member:ident = $value:expr,)+
    }) => (
		use std::fmt;
		use std::fmt::{Display, Formatter};
		$(#[$attrs])*
		#[derive(Clone, Copy, Debug, PartialEq)]
		#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
		pub enum $name {
			$(
				$(#[$item_attrs])*
				$member = $value,
			)+
		}
		impl $name {
			#[doc = "Returns the integral value of the"]
			#[doc = $sname]
			#[doc = "."]
			pub fn get_value(&self) -> $repr_type {
				*self as $repr_type
			}
		}
		impl From<$type> for $name {
			fn from(value: $type) -> $name {
				match value {
					$(
						$value => $name::$member,
					)+
					_ => $name::from($default)
				}
			}
		}
		impl Into<$type> for $name {
			fn into(self) -> $type {
				self as $type
			}
		}
		impl Display for $name {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				let name = match *self {
					$(
						$name::$member => stringify!($member),
					)+
				};
				write!(f, "{}::{}", $sname, name)
			}
		}
	);
}
macro_rules! enumeration {
	($(#[$attrs:meta])*
	$name:ident<$repr_type:ty, $type:ty> {
		__DEFAULT__ = $default:expr,
        $($(#[$item_attrs:meta])* $member:ident = $value:expr,)+
    }) => (enumeration_internal! {
		$(#[$attrs])*
		$name<$repr_type, $type> (stringify!($name)) {
			@$default,
			$($(#[$item_attrs])* $member = $value,)+
		}
	});
	($(#[$attrs:meta])*
	$name:ident<$type:ty> {
		__DEFAULT__ = $default:expr,
        $($(#[$item_attrs:meta])* $member:ident = $value:expr,)+
    }) => (enumeration_internal! {
		$(#[$attrs])*
		$name<$type, $type> (stringify!($name)) {
			@$default,
			$($(#[$item_attrs])* $member = $value,)+
		}
	})
}
macro_rules! flags_internal {
	($(#[$attrs:meta])*
	$name:ident<$type:ty> ($sname:expr) {
        $($(#[$flag_attrs:meta])* $member:ident = $value:expr,)+
    }) => (
		use std::fmt;
        $(#[$attrs])*
		#[derive(Clone, Copy, Debug, PartialEq)]
		#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
		#[allow(non_snake_case)]
        pub struct $name {
			$(
				$(#[$flag_attrs])*
				pub $member: bool,
			)+
		}
		impl $name {
			#[doc = "Creates a new"]
			#[doc = $sname]
			#[doc = "object with all fields set to false."]
			pub fn new() -> $name {
				$name {
					$($member: false,)+
				}
			}
		}
		impl From<$type> for $name {
			fn from(value: $type) -> $name {
				let mut flags = $name::new();
				$(flags.$member = value & $value != 0;)+
				flags
			}
		}
		impl Into<$type> for $name {
			fn into(self) -> $type {
				let mut value: $type = 0;
				$(if self.$member { value |= $value; })+
				value
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				let mut ret = String::new();
				$(
					if self.$member {
						if ret != "" { ret.push_str(" | "); }
						ret.push_str(stringify!($member));
					}
				)+
				write!(f, "{}({})", $sname, &ret)
			}
		}
	);
}
macro_rules! flags {
	($(#[$attrs:meta])*
	$name:ident<$type:ty> {
        $($(#[$flag_attrs:meta])* $member:ident = $value:expr,)+
    }) => (flags_internal! {
		$(#[$attrs])*
		$name<$type> (stringify!($name)) {
			$($(#[$flag_attrs])* $member = $value,)+
		}
	});
}
macro_rules! handle {
	($x:expr) => {
		{
			let handle = processenv::GetStdHandle($x);
			if handle as isize == -1 { throw_err!($crate::errors::InvalidHandleError::new()); }
			handle
		}
	};
}
macro_rules! make_colorref {
	($x:expr) => ($x.r as u32 | (($x.g as u32) << 8) | (($x.b as u32) << 16));
}
macro_rules! make_rgb {
	($x:expr) => {
		RGB8 {
			r: ($x & 0x0000ff) as u8,
			g: (($x >> 8) & 0x00ff) as u8,
			b: (($x >> 16) & 0xff) as u8
		}
	}
}
macro_rules! os_err {
	() => (
		{
			use std::io;
			use $crate::errors::*;
			let last_err = io::Error::last_os_error();
			let err = match last_err.raw_os_error().unwrap() {
				6 => WinError::from(InvalidHandleError::new()),
				_ => WinError::from(last_err)
			};
			Err(err)
		}
	);
	($i:expr) => {
		if $i == 0 {
			return os_err!();
		}
	};
	($i:expr, $x:expr) => {
		if $x {
			use std::io;
			let err = io::Error::last_os_error();
			if err.raw_os_error().unwrap() != 0 {
				os_err!($i);
			}
		} else {
			os_err!($i);
		}
	}
}
macro_rules! str_to_buf_internal {
	($s:expr, $type:ty) => {
		{
			let vec: Vec<$type> = String::from($s)
				.as_bytes()
				.iter()
				.map(|c| *c as $type)
				.collect();
			vec.into_boxed_slice()
		}
	};
	($s:expr, $size:expr, $type:ty) => {
		{
			let mut buffer: [$type; $size] = [0; $size];
			for (chr, val) in $s.as_bytes().iter().zip(buffer.iter_mut()) {
				*val = *chr as $type;
			}
			buffer
		}
	}
}
macro_rules! str_to_buf {
	($s:expr) => (str_to_buf_internal!($s, CHAR));
	($s:expr, $size:expr) => (str_to_buf_internal!($s, $size, CHAR));
}
macro_rules! str_to_buf_w {
	($s:expr) => (str_to_buf_internal!($s, WCHAR));
	($s:expr, $size:expr) => (str_to_buf_internal!($s, $size, WCHAR));
}
macro_rules! throw_err {
	($err:expr) => {
		Err($crate::errors::WinError::from($err))?;
	}
}
