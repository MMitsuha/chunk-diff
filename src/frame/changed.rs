use crate::util::{Chunk, Rect};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Changed {
    chunk: Chunk,
    data: Vec<Vec<u8>>,
}

impl Changed {
    pub(crate) fn new(chunk: Chunk, data: Vec<Vec<u8>>) -> Self {
        Self { chunk, data }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }

    pub fn apply(&self, data: &mut [u8], full: &Rect) {
        self.chunk.apply(data, &self.data, full);
    }
}
