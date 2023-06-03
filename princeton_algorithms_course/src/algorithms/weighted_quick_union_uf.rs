use std::fmt::{self, Display, Formatter};

pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    pub fn new(n: usize) -> Self {
        let count = n;
        let parent = (0..n).collect();
        let size = vec![1; n];

        WeightedQuickUnionUF {
            parent,
            size,
            count,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&mut self, p: usize) -> usize {
        self.validate(p);
        let mut root = p;
        while root != self.parent[root] {
            root = self.parent[root];
        }

        // Perform path compression
        let mut current = p;
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        root
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return;
        }

        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }

        self.count -= 1;
    }

    fn validate(&self, p: usize) {
        let n = self.parent.len();
        if p >= n {
            panic!("Index out of bounds");
        }
    }
}

impl Display for WeightedQuickUnionUF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec: &Vec<usize> = &self.parent;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedQuickUnionUF;

    /// Check basic union and connected works
    #[test]
    fn test_1() {
        let mut wq = WeightedQuickUnionUF::new(10);
        wq.union(0, 1);

        assert!(wq.connected(0, 1));
    }
}
