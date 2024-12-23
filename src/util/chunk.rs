use super::{point::Point, rect::Rect};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator, ParallelSlice,
    ParallelSliceMut,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Chunk {
    point: Point,
    rect: Rect,
}

impl Chunk {
    pub fn new(point: Point, rect: Rect) -> Self {
        Self { point, rect }
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn extract<'a>(&self, data: &'a [u8], full: &Rect) -> Vec<Vec<u8>> {
        data.par_chunks(full.width())
            .skip(self.point.y())
            .take(self.rect.height())
            .map(|slice| slice[self.point.x()..self.point.x() + self.rect.width()].to_vec())
            .collect::<Vec<_>>()
    }

    pub fn apply(&self, data: &mut [u8], slices: &Vec<Vec<u8>>, full: &Rect) {
        data.par_chunks_mut(full.width())
            .skip(self.point.y())
            .take(self.rect.height())
            .map(|slice| &mut slice[self.point.x()..self.point.x() + self.rect.width()])
            .zip(slices.par_iter())
            .map(|(dst, src)| dst.copy_from_slice(src))
            .collect::<Vec<_>>();
    }
}
