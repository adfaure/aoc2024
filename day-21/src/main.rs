#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .take_while(|l| l.is_ok())
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();
    Ok(())
}

