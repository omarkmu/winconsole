/// Information about console history.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HistoryInfo {
    /// The number of commands kept in each history buffer.
    pub size: u32,
    /// The number of history buffers kept.
    pub number_of_buffers: u32,
    /// Should duplicate entries be stored in history buffers?
    pub duplicates_allowed: bool,
}

impl Default for HistoryInfo {
    /**
    Returns an empty HistoryInfo object.
    */
    fn default() -> Self {
        Self {
            size: 0,
            number_of_buffers: 0,
            duplicates_allowed: false,
        }
    }
}
