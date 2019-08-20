use super::*;

/// Represents an input event which occurred on a mouse button.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseEvent {
    /// The mouse button the event occurred on.
    pub button: u8,
    /// The KeyCode of the mouse button which the event occurred on.
    pub key_code: KeyCode,
    /// A ControlKeyState object describing the state of control keys.
    pub modifiers: ControlKeyState,
    /// The character cell the event occurred on.
    pub position: Vector2<u16>,
    /// Is the mouse button pressed?
    pub pressed: bool,
}

impl MouseEvent {
    /**
    Returns an empty MouseEvent.
    */
    pub fn new() -> MouseEvent {
        MouseEvent {
            button: 0,
            key_code: KeyCode::None,
            modifiers: ControlKeyState::default(),
            position: Vector2::new(0, 0),
            pressed: false,
        }
    }
}

impl Into<InputEvent> for MouseEvent {
    fn into(self) -> InputEvent {
        if self.pressed {
            InputEvent::MouseDown(self)
        } else {
            InputEvent::MouseUp(self)
        }
    }
}
