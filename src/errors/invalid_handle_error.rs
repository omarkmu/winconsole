/// Describes an error related to an invalid handle.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InvalidHandleError;

impl Default for InvalidHandleError {
    /**
    Creates a new InvalidHandleError.
    */
    fn default() -> Self {
        Self {}
    }
}

impl_err!(
    InvalidHandleError,
    "invalid handle",
    "attempt to use an invalid handle"
);
