use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct WidthString {
    string: String,
    width: usize,
}

impl WidthString {
    pub fn new<T: Display>(thing: T) -> Self {
        let string = thing.to_string();
        #[cfg(feature = "unicode-width")]
        let width  = ::unicode_width::UnicodeWidthStr::width(string.as_str());
        #[cfg(not(feature = "unicode-width"))]
        let width  = string.len();
        WidthString { string, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }
}

impl Debug for WidthString {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.string)
    }
}

