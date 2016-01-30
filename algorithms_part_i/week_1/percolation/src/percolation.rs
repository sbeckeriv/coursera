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
        let width = self.width as usize;
        for (index, location) in self.chart.ids.iter().enumerate() {
            if index > 0 && index % width == 0 {
                write!(f, "\n");
            }
            write!(f, "{:>5}", *location);
        }
        write!(f, "\n")
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
        let uf_size = offset_height * width;
        let mut chart = UF::new(&uf_size);
        for i in 0..width * 2 {
            chart.union(i, 0);
            chart.union(uf_size - 1 - i, uf_size - 1);
        }
        println!("{:?}", chart);
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
        let isize_row = row as isize;
        let base_index = self.index_for(isize_col, isize_row).unwrap();
        if !self.grid[base_index] {
            self.grid[base_index] = true;
            // make a match?
            let directions = [(isize_col + 1, isize_row),
                              (isize_col - 1, isize_row),
                              (isize_col, isize_row + 1),
                              (isize_col, isize_row - 1)];
            println!("col: {}, row: {}", isize_col, isize_row);
            // println!("{:?}", directions);

            for direction in &directions {
                let index = self.index_for(direction.0, direction.1);
                let chart_index = self.index_for_chart(direction.0, direction.1 + 1);
                if index.is_some() {
                    let index = index.unwrap();
                    // println!("index:{} directions:{:?}", index, direction);
                    if self.grid[index] {
                        if chart_index.is_some() {
                            self.chart.union(chart_index.unwrap(), base_index);
                        }
                    }
                } else if chart_index.is_some() {
                    self.chart.union(chart_index.unwrap(), base_index);
                }

            }

            println!("{}", self);
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
    // includes the space offset for top and bottom
    fn index_for_chart(&self, col: isize, row: isize) -> Option<usize> {
        if col >= 0 && row >= 0 && self.width > col && self.height + 2 > row {
            let index = (self.width * row + col) as usize;
            Some(index)
        } else {
            None
        }
    }

    pub fn is_percolated(&mut self) -> bool {
        let end = self.chart.len() - 1;
        self.chart.connected(0, end)
    }
}
