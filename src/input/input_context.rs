use super::*;

/// Used to obtain input events.
pub struct InputContext {
	/// Should repeated events be sent?
	pub repeat_enabled: bool,
	/// Should the context restore the original input mode when it is dropped?
	pub restore_on_drop: bool,

	pub(crate) button_status: [bool; 5],
	pub(crate) held_keys: Vec<KeyCode>,
	original_mode: InputSettings,
	queue: Vec<InputEvent>
}

impl InputContext {
	/**
	 * Returns all of the input events which are currently in the queue.
	 *
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::Input;
	 * # fn main() {
	 * let mut ctx = Input::start().unwrap();
	 * loop {
	 * 	let events = ctx.get().unwrap();
	 * 	for event in events {
	 * 		println!("{}", event);
	 * 	}
	 * }
	 * # }
	 */
	pub fn get(&mut self) -> IoResult<Vec<InputEvent>> {
		self.collect(false)?;
		let events = self.queue.clone();
		self.queue.clear();
		Ok(events)
	}
	/**
	 * Returns a single input event, or InputEvent::None if none are available.
	 *
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::{Input, InputContext, InputEvent};
	 * # fn main() {
	 * let mut ctx = Input::start().unwrap();
	 * loop {
	 * 	let event = ctx.poll().unwrap();
	 * 	if event != InputEvent::None {
	 * 		println!("{}", event);
	 * 	}
	 * }
	 * # }
	 * ```
	 */
	pub fn poll(&mut self) -> IoResult<InputEvent> {
		if self.queue.len() == 0 {
			self.collect(false)?;
			if self.queue.len() == 0 { return Ok(InputEvent::None); }
		}
		Ok(self.queue.remove(0))
	}
	/**
	 * Resets the internal state of the context, clearing data about which keys and buttons are
	 * currently held along with the event queue.
	 *
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::Input;
	 * # fn main() {
	 * let mut ctx = Input::start().unwrap();
	 * ctx.wait().unwrap();
	 * ctx.reset();
	 * let event = ctx.wait().unwrap();
	 * println!("{}", event);
	 * # }
	 * ```
	 */
	pub fn reset(&mut self) {
		self.held_keys.clear();
		self.queue.clear();
		for i in 0..5 {
			self.button_status[i] = Console::get_key_state(BUTTON_VIRTUAL[i] as u32);
		}
	}
	/**
	 * Adds an input event to the input queue.
	 *
	 * # Arguments
	 * * `event` - The InputEvent to add.
	 *
	 * # Examples
	 * ```
	 * # extern crate winconsole;
	 * # use winconsole::input::{Input, InputEvent, FocusEvent};
	 * # fn main() {
	 * let mut ctx = Input::start().unwrap();
	 * let focus_event = FocusEvent::new();
	 * ctx.simulate(InputEvent::FocusLost(focus_event));
	 *
	 * let event = ctx.wait().unwrap();
	 * println!("{}", event);
	 * # }
	 * ```
	 */
	pub fn simulate(&mut self, event: InputEvent) {
		self.queue.push(event);
	}
	/**
	 * Waits until an input event is available, and returns it.
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
	pub fn wait(&mut self) -> IoResult<InputEvent> {
		self.collect(true)?;
		if self.queue.len() == 0 { return Ok(InputEvent::None); }
		Ok(self.queue.remove(0))
	}

	pub(crate) fn new(original_mode: InputSettings, button_status: [bool; 5]) -> InputContext {
		InputContext {
			button_status,
			original_mode,
			repeat_enabled: true,
			restore_on_drop: true,
			held_keys: Vec::new(),
			queue: Vec::new()
		}
	}

	fn collect(&mut self, wait: bool) -> IoResult<()> {
		if !wait {
			if Console::num_input_events()? == 0 { return Ok(()); }
		}

		let records = Console::read_input(1000)?;
		let events = Input::convert_events(&records, self);
		for event in events {
			self.queue.push(event);
		}
		Ok(())
	}
}

impl Drop for InputContext {
	fn drop(&mut self) {
		if self.restore_on_drop {
			Console::set_input_mode(self.original_mode).unwrap();
		}
	}
}
