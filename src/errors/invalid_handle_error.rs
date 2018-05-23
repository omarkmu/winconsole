/// Describes an error related to an invalid handle.
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidHandleError;

impl InvalidHandleError {
	/**
	 Creates a new InvalidHandleError.
	 */
	pub fn new() -> InvalidHandleError {
		InvalidHandleError {}
	}
}

impl_err!(InvalidHandleError, "invalid handle", "attempt to use an invalid handle");
