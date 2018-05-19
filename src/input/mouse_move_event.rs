use super::*;

/// Represents an input event which occurred as a result of mouse movement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MouseMoveEvent {
	/// A ControlKeyState object describing the state of control keys.
	pub modifiers: ControlKeyState,
	/// The character cell the event occurred on.
	pub position: Vector2<u16>
}

impl MouseMoveEvent {
	/**
	 Returns an empty MouseMoveEvent.
	 */
	pub fn new() -> MouseMoveEvent {
		MouseMoveEvent {
			modifiers: ControlKeyState::new(),
			position: Vector2::new(0, 0),
		}
	}
}
