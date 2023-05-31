use crate::algorithms::weighted_quick_union_uf::WeightedQuickUnionUF;

pub struct Percolation {
    size: usize,
    grid: Vec<Vec<bool>>,
    union_find: WeightedQuickUnionUF,
    extra_union_find: WeightedQuickUnionUF,
    total_opened: usize,
}

impl Percolation {
    pub fn new(n: usize) -> Self {
        let union_find: WeightedQuickUnionUF = WeightedQuickUnionUF::new(n * n + 2);
        let extra_union_find: WeightedQuickUnionUF = WeightedQuickUnionUF::new(n * n + 1);

        Percolation {
            size: (n),
            grid: (vec![vec![false; n]; n]),
            union_find: (union_find),
            extra_union_find: (extra_union_find),
            total_opened: (0),
        }
    }

    fn validate(&self, row: usize, col: usize) {
        if (row < 1) | (row > self.size) | (col < 1) | (col > self.size) {
            panic!("Illegal value of row or column");
        }
    }

    fn get_id(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.size + (col - 1)
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.validate(row, col);
        self.grid[row - 1][col - 1]
    }

    pub fn is_full(&self, row: usize, col: usize) -> bool {
        self.validate(row, col);
        self.extra_union_find.find(self.get_id(row, col))
            == self.extra_union_find.find(self.size * self.size)
    }

    pub fn number_of_open_sites(&self) -> usize {
        self.total_opened
    }

    pub fn percolates(&self) -> bool {
        self.union_find.find(self.size * self.size)
            == self.union_find.find(self.size * self.size + 1)
    }

    pub fn open(&mut self, row: usize, col: usize) {
        if self.is_open(row, col) {}

        self.grid[row - 1][col - 1] = true;
        self.total_opened += 1;

        if row == 1 {
            self.union_find
                .union(self.size * self.size, self.get_id(row, col));
            self.extra_union_find
                .union(self.size * self.size, self.get_id(row, col));
        } else if self.is_open(row - 1, col) {
            self.union_find
                .union(self.get_id(row, col), self.get_id(row - 1, col));
            self.extra_union_find
                .union(self.get_id(row, col), self.get_id(row - 1, col));
        }

        if row == self.size {
            self.union_find
                .union(self.size * self.size + 1, self.get_id(row, col));
        } else if self.is_open(row + 1, col) {
            self.union_find
                .union(self.get_id(row, col), self.get_id(row + 1, col));
            self.union_find
                .union(self.get_id(row, col), self.get_id(row + 1, col));
        }

        if (col > 1) && self.is_open(row, col - 1) {
            self.union_find
                .union(self.get_id(row, col), self.get_id(row, col - 1));
            self.extra_union_find
                .union(self.get_id(row, col), self.get_id(row, col - 1));
        }

        if (col < self.size) && self.is_open(row, col + 1) {
            self.union_find
                .union(self.get_id(row, col), self.get_id(row, col + 1));
            self.extra_union_find
                .union(self.get_id(row, col), self.get_id(row, col + 1));
        }
    }
}
