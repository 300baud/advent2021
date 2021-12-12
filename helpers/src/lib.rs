pub struct Grid2D<'a, 'b, T> {
    width: usize,
    height: usize,
    items: &'a Vec<&'b T>,
}

impl<'a, 'b, T> Grid2D<'a, 'b, T> {
    pub fn new(width: usize, height: usize, items: &'a Vec<&'b T>) -> Self {
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

    pub fn get(&self, x: usize, y: usize) -> Option<&'b T> {
        if x < self.width && y < self.height {
            return Some(self.items[self.offset(x, y)]);
        }
        None
    }

    fn cells_for_offsets(&self, x: usize, y: usize, offsets: Vec<(isize, isize)>) -> Vec<&'b T> {
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
            .collect::<Vec<&'b T>>()
    }

    pub fn lateral_neighbors(&self, x: usize, y: usize) -> Vec<&'b T> {
        let offsets: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        self.cells_for_offsets(x, y, offsets)
    }

    pub fn diagonal_neighbors(&self, x: usize, y: usize) -> Vec<&'b T> {
        let offsets: Vec<(isize, isize)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.cells_for_offsets(x, y, offsets)
    }

    pub fn all_neighbors(&self, x: usize, y: usize) -> Vec<&'b T> {
        [self.lateral_neighbors(x, y), self.diagonal_neighbors(x, y)].concat()
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
