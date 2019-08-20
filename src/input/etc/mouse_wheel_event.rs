use super::*;

/// Represents an input event which occurred as a result of a mouse scroll wheel.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseWheelEvent {
    /// The direction and value of the scroll.
    pub delta: i16,
    /// Did the scroll event occur on a horizontal scroll wheel?
    pub horizontal: bool,
    /// A ControlKeyState object describing the state of control keys.
    pub modifiers: ControlKeyState,
    /// The character cell the event occurred on.
    pub position: Vector2<u16>,
}

impl MouseWheelEvent {
    /**
    Returns an empty MouseWheelEvent.
    */
    pub fn new() -> MouseWheelEvent {
        MouseWheelEvent {
            delta: 0,
            horizontal: false,
            modifiers: ControlKeyState::default(),
            position: Vector2::new(0, 0),
        }
    }
}

impl Into<InputEvent> for MouseWheelEvent {
    fn into(self) -> InputEvent {
        InputEvent::MouseWheel(self)
    }
}
