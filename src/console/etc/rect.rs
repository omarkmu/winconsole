use cgmath::Vector2;

/// Defines the coordinates of a the corners of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
	/// The top of the rectangle.
	pub top: u16,
	/// The bottom of the rectangle.
	pub bottom: u16,
	/// The left of the rectangle.
	pub left: u16,
	/// The right of the rectangle.
	pub right: u16
}

impl Rect {
	/**
	 Creates a new Rect.
	 */
	pub fn new(top: u16, left: u16, right: u16, bottom: u16) -> Rect {
		Rect {
			top,
			bottom,
			left,
			right
		}
	}

	/**
	 Returns a Vector representing the bottom-right corner of the rectangle.
	 */
	pub fn bottom_right(&self) -> Vector2<u16> {
		Vector2::new(self.right, self.bottom)
	}
	/**
	 Returns a Vector representing the top-left corner of the rectangle.
	 */
	pub fn top_left(&self) -> Vector2<u16> {
		Vector2::new(self.left, self.top)
	}
}
