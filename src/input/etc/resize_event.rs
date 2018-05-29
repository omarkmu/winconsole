use super::*;

/// Represents an input event which occurred as a result of a buffer resize.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizeEvent {
	/// The size of the screen buffer.
	pub size: Vector2<u16>
}

impl ResizeEvent {
	/**
	 Returns an empty ResizeEvent.
	 */
	pub fn new() -> ResizeEvent {
		ResizeEvent {
			size: Vector2::new(0, 0)
		}
	}
}

impl Into<InputEvent> for ResizeEvent {
	fn into(self) -> InputEvent {
		InputEvent::Resize(self)
	}
}
