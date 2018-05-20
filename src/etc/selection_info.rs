use cgmath::Vector2;

/// Information about a console selection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectionInfo {
	/// The console selection anchor.
	pub anchor: Vector2<u16>,
	/// The bottom-right corner of the selection.
	pub bottom_right: Vector2<u16>,
	/// Is the selection empty?
	pub empty: bool,
	/// Is the mouse down?
	pub mouse_down: bool,
	/// Is a selection occurring?
	pub selecting: bool,
	/// The top-left corner of the selection.
	pub top_left: Vector2<u16>
}

impl SelectionInfo {
	/**
	 Returns an empty SelectionInfo object.
	 */
	pub fn new() -> SelectionInfo {
		SelectionInfo {
			anchor: Vector2::new(0, 0),
			bottom_right: Vector2::new(0, 0),
			empty: false,
			mouse_down: false,
			selecting: false,
			top_left: Vector2::new(0, 0)
		}
	}
}
