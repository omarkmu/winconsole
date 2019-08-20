use super::Vector2;

/// Represents a usable console font.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsoleFont {
    /**
    An integer which describes the font family.
    See the `tmPitchAndFamily` field
    [here](https://msdn.microsoft.com/en-us/library/windows/desktop/dd145132(v=vs.85).aspx).
    */
    pub family: u32,
    /// The index of the font in the console font table.
    pub index: u32,
    /// The name of the font.
    pub name: String,
    /// The font size.
    pub size: Vector2<u16>,
    /**
    The font weight. Accepts values which are multiples of 100, with
    400 representing normal weight and 700 representing bold.
    */
    pub weight: u32,
}

impl Default for ConsoleFont {
    /**
    Returns an empty ConsoleFont object.
    */
    fn default() -> Self {
        Self {
            name: String::new(),
            size: Vector2::new(0, 0),
            weight: 0,
            family: 0,
            index: 0,
        }
    }
}
