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
    test_vec_multi_array();
}

fn test_better_read() {
    let file_path: &str = "src/percolation/input/input.txt";
    let file: File = File::open(file_path).expect("File wasn't found");
    let reader: BufReader<File> = BufReader::new(file);

    let vec_lines: Vec<String> = reader
        .lines()
        .map(|line: Result<String, std::io::Error>| line.unwrap().parse::<String>().unwrap())
        .collect();

    // let vec_nums: Vec<Vec<usize>> = Vec::new();

    // vec_lines.iter().map(|line| {
    //     line.split(' ').filter(|l| );
    // });
}

fn test_percolation() {
    let contents: String = fs::read_to_string("src/percolation/input/input.txt").unwrap();

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

    println!("{}", perc);

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

    println!("{}", perc);
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
