use std::fmt::{self, Display, Formatter};

// Lazy Approach/Algorithm
#[derive(Debug)]
pub struct QuickUnionUF {
    pub array: Vec<i32>,
}

impl QuickUnionUF {
    // Set id of each object to itself (N array accesses)
    pub fn new(n: i32) -> Self {
        let mut v: Vec<i32> = Vec::new();

        for i in 0..n {
            v.push(i);
        }

        QuickUnionUF { array: (v) }
    }

    // Chase parent pointers until root is reached
    // (depth of i array accesses)
    pub fn root(&mut self, mut i: i32) -> i32 {
        while i != self.array[i as usize] {
            i = self.array[i as usize];
        }

        i
    }

    // Check if p and q has the same root
    // (depth of p and q array accesses)
    pub fn connected(&mut self, p: i32, q: i32) -> bool {
        self.root(p) == self.root(q)
    }

    // Change root of p to point to root of q
    // (depth of p and q array accesses)
    pub fn union(&mut self, p: i32, q: i32) {
        let i: i32 = self.root(p);
        let j: i32 = self.root(q);

        self.array[i as usize] = j;
    }
}

impl Display for QuickUnionUF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec: &Vec<i32> = &self.array;

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
