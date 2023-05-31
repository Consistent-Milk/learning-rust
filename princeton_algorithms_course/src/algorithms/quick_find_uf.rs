use std::fmt::{self, Display, Formatter};
// Eager Approach/Algorithm
#[derive(Debug)]
pub struct QuickFindUF {
    pub array: Vec<i32>,
}

impl QuickFindUF {
    pub fn new(n: i32) -> Self {
        let mut v: Vec<i32> = Vec::new();

        for i in 0..n {
            v.push(i);
        }

        QuickFindUF { array: (v) }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.array[p] == self.array[q]
    }

    // union is too expensive
    // Takes N^2 (quadratic) array accesses
    // to process a sequence of N union commands
    // of N objects
    pub fn union(&mut self, p: usize, q: usize) {
        let pid: i32 = self.array[p];
        let qid: i32 = self.array[q];

        for i in 0..self.array.len() {
            if self.array[i] == pid {
                self.array[i] = qid;
            }
        }
    }
}

impl Display for QuickFindUF {
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
