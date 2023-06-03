#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]
#![allow(unused)]

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::ops::Deref;

#[macro_use]
extern crate itertools;

use princeton_algorithms_course::algorithms::quick_find_uf::QuickFindUF;
use princeton_algorithms_course::algorithms::quick_union_uf::QuickUnionUF;
use princeton_algorithms_course::percolation::percolation_alg::Percolation;

fn main() {
    let input_files = vec!["input.txt", "percolates.txt"];

    for file in input_files {
        test_percolation(file);
        println!();
    }
}

fn test_percolation(filename: &str) {
    let file_path: String = format!("src/percolation/input/{}", filename);
    let contents: String = fs::read_to_string(file_path).unwrap();

    let mut split_lines: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| {
            let mut split_line: Vec<&str> = line.trim().split(' ').collect();
            if split_line.len() == 3 {
                split_line.remove(1);
            }
            split_line
        })
        .collect();

    let n: usize = split_lines[0][0].parse().unwrap();

    split_lines.drain(0..1); // Remove the first line from split_lines

    let mut perc = Percolation::new(n);

    for val in split_lines {
        let row: usize = val[0].parse().unwrap();
        let col: usize = val[1].parse().unwrap();

        if row != 0 && col != 0 {
            perc.open(row, col)
        }
    }

    println!("{}", perc);

    println!("\n{}", perc.percolates());
}

fn test_vec_multi_array() {
    let mut v: Vec<Vec<String>> = vec![vec![String::new(); 10]; 10];

    for (i, j) in iproduct!(0..10, 0..10) {
        v[i][j] = format!("({}, {})", i, j);
    }

    for i in 0..10 {
        for j in 0..10 {
            print!("  {}  ", v[i][j]);
        }
        println!();
    }
}
