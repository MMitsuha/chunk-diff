use crate::util::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Rect of the chunk to be hashed and compared.
    rect: Rect,
    /// Full rect of the data.
    full: Rect,
}

impl Config {
    pub fn new(rect: Rect, full: Rect) -> Self {
        Self { rect, full }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn full(&self) -> &Rect {
        &self.full
    }
}
