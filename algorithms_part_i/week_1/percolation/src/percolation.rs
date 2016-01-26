use union_find::UF;
pub struct Percolator {
    chart: UF,
    grid: Vec<bool>,
    height: isize,
    width: isize,
}

impl Percolator {
    pub fn new(height: usize, width: usize) -> Self {
        let offset_height = height + 2;
        Percolator {
            chart: UF::new(&(offset_height * width)),
            grid: vec![false; width*height],
            height: height as isize,
            width: width as isize,
        }
    }
    pub fn open(&mut self, col: usize, row: usize) {
        let isize_col = col as isize;
        let isize_row = (row + 1) as isize;

        let index = self.index_for(isize_col, isize_row).unwrap();
        if !self.grid[index] {
            self.grid[index] = true;
            // make a match?
            let directions = [self.index_for(isize_col + 1, isize_row),
                              self.index_for(isize_col - 1, isize_row),
                              self.index_for(isize_col, isize_row + 1),
                              self.index_for(isize_col, isize_row - 1)];
        }
    }

    fn index_for(&self, col: isize, row: isize) -> Option<usize> {
        if col >= 0 && row >= 0 && self.width > col && self.height > row {
            let index = (self.width * row + col) as usize;
            Some(index)
        } else {
            None
        }
    }

    pub fn is_open(&self, col: usize, row: usize) -> bool {}
    pub fn is_full(&self, col: usize, row: usize) -> bool {}
    pub fn is_percolated(&self, col: usize, row: usize) -> bool {}
}
