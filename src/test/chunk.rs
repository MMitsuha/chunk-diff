use crate::util::Rect;

#[test]
fn divide_chunk_exact() {
    let full = Rect::new(320, 180);
    let rect = Rect::new(160, 90);
    let chunks = full.divide(&rect);

    assert_eq!(chunks.len(), 4);

    assert_eq!(chunks[0].point().x(), 0);
    assert_eq!(chunks[0].point().y(), 0);

    assert_eq!(chunks[1].point().x(), 160);
    assert_eq!(chunks[1].point().y(), 0);

    assert_eq!(chunks[2].point().x(), 0);
    assert_eq!(chunks[2].point().y(), 90);

    assert_eq!(chunks[3].point().x(), 160);
    assert_eq!(chunks[3].point().y(), 90);

    for i in 0..4 {
        assert_eq!(chunks[i].rect().width(), 160);
        assert_eq!(chunks[i].rect().height(), 90);
    }
}

#[test]
fn divide_chunk_remainder() {
    let full = Rect::new(170, 100);
    let rect = Rect::new(80, 45);
    let chunks = full.divide(&rect);

    assert_eq!(chunks.len(), 9);

    assert_eq!(chunks[0].point().x(), 0);
    assert_eq!(chunks[0].point().y(), 0);

    assert_eq!(chunks[1].point().x(), 80);
    assert_eq!(chunks[1].point().y(), 0);

    assert_eq!(chunks[2].point().x(), 0);
    assert_eq!(chunks[2].point().y(), 45);

    assert_eq!(chunks[3].point().x(), 80);
    assert_eq!(chunks[3].point().y(), 45);

    // Remainder chunks
    assert_eq!(chunks[4].point().x(), 160);
    assert_eq!(chunks[4].point().y(), 0);

    assert_eq!(chunks[5].point().x(), 160);
    assert_eq!(chunks[5].point().y(), 45);

    assert_eq!(chunks[6].point().x(), 0);
    assert_eq!(chunks[6].point().y(), 90);

    assert_eq!(chunks[7].point().x(), 80);
    assert_eq!(chunks[7].point().y(), 90);

    assert_eq!(chunks[8].point().x(), 160);
    assert_eq!(chunks[8].point().y(), 90);

    for i in 0..4 {
        assert_eq!(chunks[i].rect().width(), 80);
        assert_eq!(chunks[i].rect().height(), 45);
    }

    for i in 4..6 {
        assert_eq!(chunks[i].rect().width(), 10);
        assert_eq!(chunks[i].rect().height(), 45);
    }

    for i in 6..8 {
        assert_eq!(chunks[i].rect().width(), 80);
        assert_eq!(chunks[i].rect().height(), 10);
    }

    assert_eq!(chunks[8].rect().width(), 10);
    assert_eq!(chunks[8].rect().height(), 10);
}
