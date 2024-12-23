use twox_hash::XxHash64;

const DEFAULT_SEED: u64 = 114514;

pub struct XxHasher {
    seed: u64,
}

impl super::Hasher for XxHasher {
    fn hash(&self, data: &[u8]) -> u64 {
        XxHash64::oneshot(self.seed, data)
    }
}

impl Default for XxHasher {
    fn default() -> Self {
        Self { seed: DEFAULT_SEED }
    }
}

impl XxHasher {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}
