use std::slice::{Iter, SliceIndex};

pub struct Grid2D<'a, T: Copy> {
    width: usize,
    height: usize,
    items: Vec<&'a Box<T>>,
}

impl<'a, T: Copy> Grid2D<'a, T> {
    pub fn new(width: usize, height: usize, items: Vec<&'a Box<T>>) -> Self {
        Grid2D {
            width: width,
            height: height,
            items: items,
        }
    }

    #[inline]
    fn offset(&self, x: usize, y: usize) -> usize {
        y * self.height + x
    }

    #[inline]
    fn offset_rev(&self, offset: usize) -> (usize, usize) {
        (offset % self.width, offset / self.height)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Box<T>> {
        if x < self.width && y < self.height {
            let item = self.items[self.offset(x, y)];
            return Some(&item);
        }
        None
    }

    pub fn iter(&self) -> Iter<&'a Box<T>> {
        self.items.iter()
    }

    fn cells_for_offsets(&self, x: usize, y: usize, offsets: Vec<(isize, isize)>) -> Vec<&Box<T>> {
        offsets
            .iter()
            .map(|pair| (x as isize + pair.0, y as isize + pair.1))
            .filter(|pair| {
                pair.0 > 0
                    && pair.0 < self.width as isize
                    && pair.1 > 0
                    && pair.1 < self.height as isize
            })
            .map(|p| self.get(x + p.0 as usize, y + p.1 as usize))
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<&Box<T>>>()
    }

    pub fn lateral_neighbors(&self, x: usize, y: usize) -> Vec<&Box<T>> {
        let offsets: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.cells_for_offsets(x, y, offsets)
    }

    pub fn diagonal_neighbors(&self, x: usize, y: usize) -> Vec<&Box<T>> {
        let offsets: Vec<(isize, isize)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.cells_for_offsets(x, y, offsets)
    }

    pub fn all_neighbors(&self, x: usize, y: usize) -> Vec<&Box<T>> {
        let offsets: Vec<(isize, isize)> = vec![
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        self.cells_for_offsets(x, y, offsets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}