use std::fmt::{self, Display, Formatter};
// Eager Approach/Algorithm
#[derive(Debug)]
pub struct QuickFindUF {
    pub array: Vec<usize>,
}

impl QuickFindUF {
    pub fn new(n: usize) -> Self {
        // As we know the size of the vec beforehand
        // we can allocate necessary memory during compile time
        let mut v: Vec<usize> = vec![0; n];

        for i in 0..n {
            v[i] = i;
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
        let pid: usize = self.array[p];
        let qid: usize = self.array[q];

        for i in 0..self.array.len() {
            if self.array[i] == pid {
                self.array[i] = qid;
            }
        }
    }
}

impl Display for QuickFindUF {
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
    use super::QuickFindUF;

    /// Check if union and connected works
    #[test]
    fn test_1() {
        let mut qf: QuickFindUF = QuickFindUF::new(10);
        qf.union(3, 4);

        assert!(qf.connected(3, 4));
    }

    /// Check if nested union and connected works
    #[test]
    fn test_2() {
        let mut qf: QuickFindUF = QuickFindUF::new(10);

        qf.union(0, 1);
        qf.union(1, 2);

        assert!(qf.connected(0, 2));
    }
}
