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

        return QuickFindUF { array: (v) };
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        return self.array[p] == self.array[q];
    }

    // union is too expensive
    // Takes N^2 (quadratic) array accesses
    // to process a sequence of N union commands
    // of N objects
    pub fn union(&mut self, p: usize, q: usize) -> () {
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
        let vec = &self.array;

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

// Lazy Approach/Algorithm
#[derive(Debug)]
pub struct QuickUnionUF {
    pub array: Vec<i32>,
}

impl QuickUnionUF {
    // Set id of each object to itself (N array accesses)
    pub fn new(n: i32) -> Self {
        let mut v = Vec::new();

        for i in 0..n {
            v.push(i);
        }

        return QuickUnionUF { array: (v) };
    }

    // Chase parent pointers until root is reached
    // (depth of i array accesses)
    pub fn root(&mut self, mut i: i32) -> i32 {
        while i != self.array[i as usize] {
            i = self.array[i as usize];
        }
        return i as i32;
    }

    // Check if p and q has the same root
    // (depth of p and q array accesses)
    pub fn connected(&mut self, p: i32, q: i32) -> bool {
        return self.root(p) == self.root(q);
    }

    // Change root of p to point to root of q
    // (depth of p and q array accesses)
    pub fn union(&mut self, p: i32, q: i32) -> () {
        let i: i32 = self.root(p);
        let j: i32 = self.root(q);

        self.array[i as usize] = j;
    }
}

impl Display for QuickUnionUF {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec = &self.array;

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
