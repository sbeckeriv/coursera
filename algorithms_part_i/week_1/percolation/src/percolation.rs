use union_find::UF;
use std::fmt;

pub struct Percolator {
    chart: UF,
    grid: Vec<bool>,
    height: isize,
    width: isize,
}

impl fmt::Debug for Percolator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\n", self.chart)
    }
}

impl fmt::Display for Percolator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.width as usize;
        for (index, location) in self.grid.iter().enumerate() {

            if index > 0 && index % width == 0 {
                write!(f, "\n");
            }
            if *location {
                write!(f, "{:>2}", "x");
            } else {
                write!(f, "{:>2}", "-");
            }
        }
        write!(f, "\n")
    }
}
impl Percolator {
    pub fn new(height: usize, width: usize) -> Self {
        let offset_height = height + 2;
        let mut chart = UF::new(&(offset_height * width));
        for i in 1..width {
            chart.union(i, 0);
        }
        // map the top and bottom
        Percolator {
            chart: chart,
            grid: vec![false; width*height],
            height: height as isize,
            width: width as isize,
        }
    }
    pub fn open(&mut self, col: usize, row: usize) {
        let isize_col = col as isize;
        // offset only applies to the chart
        let isize_row = (row + 1) as isize;

        let grid_index = self.index_for(isize_col, isize_row - 1).unwrap();
        let index = self.index_for(isize_col, isize_row).unwrap();
        if !self.grid[grid_index] {
            self.grid[grid_index] = true;
            // make a match?
            let directions = [self.index_for(isize_col + 1, isize_row),
                              self.index_for(isize_col - 1, isize_row),
                              self.index_for(isize_col, isize_row + 1),
                              self.index_for(isize_col, isize_row - 1)];
            for direction in &directions {
                if direction.is_some() {
                    self.chart.union(direction.unwrap(), index);
                }
            }

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

    pub fn is_open(&self, col: usize, row: usize) -> bool {
        false
    }
    pub fn is_full(&self, col: usize, row: usize) -> bool {
        false
    }
    pub fn is_percolated(&self, col: usize, row: usize) -> bool {
        false
    }
}
