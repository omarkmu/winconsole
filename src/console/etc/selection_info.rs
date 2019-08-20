use super::{Rect, Vector2};

/// Information about a console selection.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectionInfo {
    /// The console selection anchor.
    pub anchor: Vector2<u16>,
    /// Is the selection empty?
    pub empty: bool,
    /// Is the mouse down?
    pub mouse_down: bool,
    /// The selection rectangle.
    pub rect: Rect,
    /// Is a selection occurring?
    pub selecting: bool,
}

impl Default for SelectionInfo {
    /**
    Returns an empty SelectionInfo object.
    */
    fn default() -> Self {
        Self {
            anchor: Vector2::new(0, 0),
            empty: false,
            mouse_down: false,
            rect: Rect::new(0, 0, 0, 0),
            selecting: false,
        }
    }
}
