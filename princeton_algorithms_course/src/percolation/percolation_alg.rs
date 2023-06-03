use std::fmt::{self, Display, Formatter};

use crate::algorithms::weighted_quick_union_uf::WeightedQuickUnionUF;

pub struct Percolation {
    size: usize,
    grid: Vec<Vec<bool>>,
    union_find: WeightedQuickUnionUF,
    total_opened: usize,
}

impl Percolation {
    pub fn new(n: usize) -> Self {
        let union_find = WeightedQuickUnionUF::new(n * n + 2);

        Percolation {
            size: n,
            grid: vec![vec![false; n]; n],
            union_find,
            total_opened: 0,
        }
    }

    fn validate(&self, row: usize, col: usize) {
        if row < 1 || row > self.size || col < 1 || col > self.size {
            panic!("Illegal arguments: row={}, col={}", row, col);
        }
    }

    fn get_site_index(&self, row: usize, col: usize) -> usize {
        self.validate(row, col);
        (row - 1) * self.size + (col - 1)
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.validate(row, col);
        self.grid[row - 1][col - 1]
    }

    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        self.validate(row, col);
        let site_index = self.get_site_index(row, col);
        self.union_find.connected(site_index, self.size * self.size)
    }

    pub fn number_of_open_sites(&self) -> usize {
        self.total_opened
    }

    pub fn percolates(&mut self) -> bool {
        self.union_find
            .connected(self.size * self.size, self.size * self.size + 1)
    }

    pub fn open(&mut self, row: usize, col: usize) {
        self.validate(row, col);
        let site_index = self.get_site_index(row, col);

        if !self.grid[row - 1][col - 1] {
            self.grid[row - 1][col - 1] = true;
            self.total_opened += 1;

            if row == 1 {
                self.union_find.union(site_index, self.size * self.size);
            }

            if row == self.size {
                self.union_find.union(site_index, self.size * self.size + 1);
            }

            if row > 1 && self.grid[row - 2][col - 1] {
                self.union_find
                    .union(site_index, self.get_site_index(row - 1, col));
            }

            if row < self.size && self.grid[row][col - 1] {
                self.union_find
                    .union(site_index, self.get_site_index(row + 1, col));
            }

            if col > 1 && self.grid[row - 1][col - 2] {
                self.union_find
                    .union(site_index, self.get_site_index(row, col - 1));
            }

            if col < self.size && self.grid[row - 1][col] {
                self.union_find
                    .union(site_index, self.get_site_index(row, col + 1));
            }
        }
    }
}

impl Display for Percolation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 1..=self.size {
            for j in 1..=self.size {
                let s = if self.is_open(i, j) { "T" } else { "F" };
                write!(f, " ({}, {}, {}) ", i, j, s)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percolation() {
        let mut percolation = Percolation::new(3);
        assert!(!percolation.percolates());

        percolation.open(1, 1);
        percolation.open(2, 1);
        percolation.open(2, 2);
        percolation.open(3, 3);

        assert!(percolation.is_open(1, 1));
        assert!(percolation.is_open(2, 1));
        assert!(percolation.is_open(2, 2));
        assert!(percolation.is_open(3, 3));
        assert!(!percolation.is_open(3, 2));
        assert!(!percolation.is_open(1, 3));

        assert!(percolation.is_full(1, 1));
        assert!(percolation.is_full(2, 1));
        assert!(percolation.is_full(2, 2));
        assert!(!percolation.is_full(3, 3));

        assert_eq!(percolation.number_of_open_sites(), 4);
        assert!(!percolation.percolates());
    }
}
