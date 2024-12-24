mod xxhash;

pub use xxhash::*;

use crate::util::{Chunk, Rect};
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};

pub trait Hasher {
    fn hash(&self, data: &[u8]) -> u64;

    fn hash_chunk(&self, data: &[u8], chunk: &Chunk, full: &Rect) -> Vec<u64>
    where
        Self: Sync,
    {
        let point = chunk.point();
        let rect = chunk.rect();

        data.par_chunks(full.width())
            .skip(point.y())
            .take(rect.height())
            .map(|slice| &slice[point.x()..point.x() + rect.width()])
            .map(|slice| self.hash(slice))
            .collect::<Vec<_>>()
    }
}
