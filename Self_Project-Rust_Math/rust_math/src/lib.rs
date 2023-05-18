pub struct Algorithms;

impl Algorithms {
    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest: &T = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        return largest;
    }
}

mod list;
pub mod data_structures;
pub use crate::list::*;
