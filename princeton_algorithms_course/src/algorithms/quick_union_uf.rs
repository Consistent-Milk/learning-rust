use std::fmt::{self, Display, Formatter};

// Lazy Approach/Algorithm
#[derive(Debug)]
pub struct QuickUnionUF {
    pub array: Vec<usize>,
}

impl QuickUnionUF {
    // Set id of each object to itself (N array accesses)
    pub fn new(n: usize) -> Self {
        let mut v: Vec<usize> = vec![0; n];

        for i in 0..n {
            v[i] = i;
        }

        QuickUnionUF { array: (v) }
    }

    // Chase parent pointers until root is reached
    // (depth of i array accesses)
    pub fn root(&mut self, mut i: usize) -> usize {
        while i != self.array[i] {
            i = self.array[i];
        }
        i
    }

    // Check if p and q has the same root
    // (depth of p and q array accesses)
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    // Change root of p to point to root of q
    // (depth of p and q array accesses)
    pub fn union(&mut self, p: usize, q: usize) {
        let i: usize = self.root(p);
        let j: usize = self.root(q);

        self.array[i] = j;
    }
}

impl Display for QuickUnionUF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec: &Vec<usize> = &self.array;

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
    use super::QuickUnionUF;

    /// Check if union and connected works
    #[test]
    fn test_1() {
        let mut qu: QuickUnionUF = QuickUnionUF::new(10);

        qu.union(0, 1);
        
        assert!(qu.connected(0, 1));
    }

    /// Check if root works
    #[test]
    fn test_2() {
        let mut qu: QuickUnionUF = QuickUnionUF::new(10);
        
        // 1 <- 0
        qu.union(0, 1);
        
        // 2 <- 1 <- 0
        qu.union(1, 2);

        for i in 0..=2 {
            assert_eq!(qu.root(i), 2);
        }
    }
}