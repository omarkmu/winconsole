/// A 2-dimensional vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
