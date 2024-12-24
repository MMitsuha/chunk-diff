use crate::{
    encoder::{Config, Encoder},
    frame::Frame,
    hasher::XxHasher,
    util::Rect,
};
use rand::Rng;

#[test]
fn encode() {
    let mut data = vec![0; 320 * 180];
    let rect = Rect::new(160, 90);
    let full = Rect::new(320, 180);
    let config = Config::new(rect, full);
    let mut encoder = Encoder::<XxHasher>::new(config);

    {
        let frames = encoder.encode(&data);
        assert_eq!(frames.len(), 1);
        match frames.first().unwrap() {
            Frame::Changed(changed) => {
                assert_eq!(changed.chunk().point().x(), 0);
                assert_eq!(changed.chunk().point().y(), 0);
                assert_eq!(
                    changed
                        .data()
                        .iter()
                        .map(|y_slice| y_slice.iter())
                        .flatten()
                        .filter(|b| **b != 0)
                        .count(),
                    0
                );
            }
        }
    }

    {
        data[161] = 1;
        let frames = encoder.encode(&data);
        assert_eq!(frames.len(), 1);
        match frames.first().unwrap() {
            Frame::Changed(changed) => {
                assert_eq!(changed.chunk().point().x(), 160);
                assert_eq!(changed.chunk().point().y(), 0);
                assert_eq!(
                    changed
                        .data()
                        .iter()
                        .map(|y_slice| y_slice.iter())
                        .flatten()
                        .filter(|b| **b != 0)
                        .count(),
                    1
                );
                data[161] = 2;
                changed.apply(&mut data, &full);
                assert_eq!(data[161], 1);
            }
        }
    }

    {
        let frames = encoder.encode(&data);
        assert_eq!(frames.len(), 0);
    }
}

#[test]
fn encode_and_decode_4k() {
    let mut data = vec![0; 3840 * 2160];
    let rect = Rect::new(320, 180);
    let full = Rect::new(3840, 2160);
    let config = Config::new(rect, full);
    let mut rng = rand::thread_rng();
    let mut encoder = Encoder::<XxHasher>::new(config);
    let mut old_data = data.clone();
    let _ = encoder.encode(&data);

    (0..10).for_each(|_| {
        let start = rng.gen_range(0..3840 * 2160 - 10);
        data[start..start + 10]
            .copy_from_slice(&(0..10).map(|_| rng.gen::<u8>()).collect::<Vec<_>>());
    });

    let frames = encoder.encode(&data);
    frames.iter().for_each(|frame| match frame {
        Frame::Changed(changed) => changed.apply(&mut old_data, &full),
    });
    assert_eq!(data, old_data);
}
