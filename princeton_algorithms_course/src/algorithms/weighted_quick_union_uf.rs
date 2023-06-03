use std::fmt::{self, Display, Formatter};

pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    pub fn new(n: usize) -> Self {
        let count: usize = n;
        let parent: Vec<usize> = (0..n).collect();
        let size: Vec<usize> = vec![1; n];

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
    use super::*;

    #[test]
    fn test_count() {
        let mut uf: WeightedQuickUnionUF = WeightedQuickUnionUF::new(10);
        assert_eq!(uf.count(), 10);

        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);

        assert_eq!(uf.count(), 7);

        uf.union(9, 4);
        uf.union(2, 1);

        assert_eq!(uf.count(), 5);
    }

    #[test]
    fn test_connected() {
        let mut uf: WeightedQuickUnionUF = WeightedQuickUnionUF::new(10);

        assert!(!uf.connected(0, 1));
        assert!(!uf.connected(2, 3));

        uf.union(0, 1);
        uf.union(2, 3);

        assert!(uf.connected(0, 1));
        assert!(uf.connected(2, 3));

        uf.union(1, 3);

        assert!(uf.connected(0, 2));
        assert!(uf.connected(1, 3));
    }

    #[test]
    fn test_union() {
        let mut uf: WeightedQuickUnionUF = WeightedQuickUnionUF::new(10);

        assert!(!uf.connected(4, 8));

        uf.union(4, 8);

        assert!(uf.connected(4, 8));
        assert_eq!(uf.count(), 9);

        uf.union(2, 9);
        uf.union(1, 7);
        uf.union(2, 7); // Connect a different pair

        assert!(uf.connected(2, 9));
        assert!(uf.connected(1, 7));
        assert!(uf.connected(2, 7)); // Verify the new connection
        assert_eq!(uf.count(), 6);
    }
}
