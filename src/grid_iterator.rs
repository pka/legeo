//! Grid iterators

pub struct GridIterator {
    z: usize,
    x: usize,
    y: usize,
}

impl GridIterator {
    pub fn new() -> GridIterator {
        GridIterator { z: 0, x: 0, y: 0 }
    }
}

impl Iterator for GridIterator {
    type Item = (usize, usize, usize); // z, y, x

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;

        if self.x < 42 {
            Some((self.z, self.x, self.y))
        } else {
            None
        }
    }
}
