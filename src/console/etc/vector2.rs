/// A 2-dimensional vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
	/// The x component.
	pub x: T,
	/// The y component.
	pub y: T
}

impl<T> Vector2<T> {
	/**
	 * Creates a new Vector2.
	 *
	 * # Arguments
	 * `x` - The x component of the vector.  
	 * `y` - The y component of the vector.
	 */
	pub fn new(x: T, y: T) -> Vector2<T> {
		Vector2 {
			x: x,
			y: y
		}
	}
}

#[cfg(feature = "cgmath")]
impl<T> Into<cgmath::Vector2<T>> for Vector2<T> {
	fn into(self) -> cgmath::Vector2<T> {
		cgmath::Vector2 {
			x: self.x,
			y: self.y
		}
	}
}
