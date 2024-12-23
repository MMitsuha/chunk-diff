mod config;

pub use config::Config;

use crate::{
    frame::{changed::Changed, Frame},
    hasher::Hasher,
    util::{chunk::Chunk, point::Point},
};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub struct Encoder<T>
where
    T: Hasher,
{
    hasher: T,
    config: Config,
    chunks: Vec<Chunk>,
    hashes: Vec<(Chunk, Vec<u64>)>,
}

impl<T> Encoder<T>
where
    T: Hasher + Sync,
{
    pub fn new(config: Config) -> Self
    where
        T: Default,
    {
        Self::new_with_hasher(T::default(), config)
    }

    pub fn new_with_hasher(hasher: T, config: Config) -> Self {
        let full = config.full();
        let rect = config.rect();
        let chunks = full.divide(rect);
        let size = chunks.len();

        Self {
            hasher,
            config,
            chunks,
            hashes: Vec::with_capacity(size),
        }
    }

    pub fn encode(&mut self, data: &[u8]) -> Vec<Frame> {
        let full = self.config.full();

        if data.len() != full.area() {
            panic!("Data length does not match full rect area");
        }

        let hashes = self
            .chunks
            .par_iter()
            .map(|chunk| (*chunk, self.hasher.hash_rect(data, chunk, full)))
            .collect::<Vec<_>>();
        let frames = if self.hashes.len() != hashes.len() {
            let full_chunk = Chunk::new(Point::new(0, 0), *full);
            vec![Frame::Changed(Changed::new(
                full_chunk,
                full_chunk.extract(data, full),
            ))]
        } else {
            hashes
                .par_iter()
                .zip(self.hashes.par_iter())
                .filter(|((_, new_hash), (_, old_hashes))| new_hash != old_hashes)
                .map(|((chunk, _), (_, _))| {
                    Frame::Changed(Changed::new(*chunk, chunk.extract(data, full)))
                })
                .collect::<Vec<_>>()
        };

        self.hashes = hashes;

        frames
    }
}
