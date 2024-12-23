use super::{chunk::Chunk, point::Point};
use num_integer::Integer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn divide(&self, rect: &Rect) -> Vec<Chunk> {
        let (x_count, x_remainder) = self.width.div_rem(&rect.width);
        let (y_count, y_remainder) = self.height.div_rem(&rect.height);
        let count = match (x_remainder, y_remainder) {
            (0, 0) => x_count * y_count,
            (0, _) => x_count * y_count + x_count,
            (_, 0) => x_count * y_count + y_count,
            (_, _) => x_count * y_count + x_count + y_count + 1,
        };
        let mut buffer = Vec::with_capacity(count);

        for y in 0..y_count {
            for x in 0..x_count {
                buffer.push(Chunk::new(
                    Point::new(x * rect.width, y * rect.height),
                    Self::new(rect.width, rect.height),
                ));
            }
        }
        if x_remainder != 0 {
            for y in 0..y_count {
                buffer.push(Chunk::new(
                    Point::new(x_count * rect.width, y * rect.height),
                    Self::new(x_remainder, rect.height),
                ));
            }
        }
        if y_remainder != 0 {
            for x in 0..x_count {
                buffer.push(Chunk::new(
                    Point::new(x * rect.width, y_count * rect.height),
                    Self::new(rect.width, y_remainder),
                ));
            }
        }
        if x_remainder != 0 && y_remainder != 0 {
            buffer.push(Chunk::new(
                Point::new(x_count * rect.width, y_count * rect.height),
                Self::new(x_remainder, y_remainder),
            ));
        }

        buffer
    }
}
