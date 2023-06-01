#![allow(unused_imports)]
use std::fs;
use std::ops::Deref;

use princeton_algorithms_course::algorithms::quick_find_uf::QuickFindUF;
use princeton_algorithms_course::algorithms::quick_union_uf::QuickUnionUF;
use princeton_algorithms_course::percolation::percolation_alg::Percolation;

fn main() {
    let contents: String = fs::read_to_string("src/percolation/input/greeting57.txt").unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let mut split_lines: Vec<Vec<&str>> = Vec::new();

    for line in lines {
        let mut split_line: Vec<&str> = line.trim_start().split(' ').collect();

        if split_line.len() == 3 {
            split_line.remove(1);
        }

        split_lines.push(split_line);
    }

    let n: usize = split_lines[0][0].parse().unwrap();

    let mut perc: Percolation = Percolation::new(n);

    println!("{}", n);

    for val in split_lines.iter().skip(1) {
        let row: usize = val[0].deref().parse().unwrap();
        let col: usize = val[1].deref().parse().unwrap();

        println!("{} {}", row, col);

        if (row == 0) | (col == 0) {
            continue;
        }

        perc.open(row, col);
        println!("{} {} is open: {}", row, col, perc.is_open(row, col));
    }

    println!("Percolates: {}", perc.percolates());
}
