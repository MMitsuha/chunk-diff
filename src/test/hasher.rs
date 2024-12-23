use crate::{
    hasher::{xxhash::XxHasher, Hasher},
    util::rect::Rect,
};

#[test]
fn hash_rect_eq() {
    let data = vec![0; 9 * 8];
    let rect = Rect::new(3, 2);
    let full = Rect::new(9, 8);
    let chunks = full.divide(&rect);
    let hasher = XxHasher::default();
    let hashes = chunks
        .iter()
        .map(|chunk| hasher.hash_rect(&data, &chunk, &full))
        .collect::<Box<_>>();

    assert_eq!(hashes.len(), 12);

    assert_eq!(hashes[0], hashes[1]);
    assert_eq!(hashes[1], hashes[2]);
    assert_eq!(hashes[2], hashes[3]);
}

#[test]
fn hash_rect_ne() {
    let data = vec![0, 0, 1, 1];
    let rect = Rect::new(2, 1);
    let full = Rect::new(4, 1);
    let chunks = full.divide(&rect);
    let hasher = XxHasher::default();
    let hashes = chunks
        .iter()
        .map(|chunk| hasher.hash_rect(&data, &chunk, &full))
        .collect::<Box<_>>();

    assert_eq!(hashes.len(), 2);

    assert_ne!(hashes[0], hashes[1]);
}
