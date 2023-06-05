// via bard.google.com
use std::iter::Iterator;

struct Bresenham {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Bresenham {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    fn draw(&self) -> impl Iterator<Item = (i32, i32)> {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        let step = if dy > 0 { 1 } else { -1 };
        let y = self.y1;

        (self.x1..self.x2 + 1)
            .map(|x| (x, y))
            .chain(std::iter::repeat((self.x2, y)))
            .map(|(x, y)| {
                let err = (dy * x) - (dx * y);
                while err > 0 {
                    y += step;
                    err -= 2 * dy;
                }
                (x, y)
            })
    }
}

