#![feature(iter_array_chunks)]
use itertools::Either;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat_n;

fn main() -> std::io::Result<()> {
    let mut lokcs_and_keys = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| l.chars().collect_vec())
        .fold(vec![vec![]], |mut acc, line| {
            if line.is_empty() {
                acc.push(vec![]);
            } else {
                acc.last_mut().expect("").push(line);
            }
            acc
        });

    let (locks, keys): (Vec<_>, Vec<_>) = lokcs_and_keys.into_iter().partition_map(|r| {
        if lock_or_key(&r) {
            Either::Left(r)
        } else {
            Either::Right(r)
        }
    });

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if count_column(key)
                .iter()
                .zip(count_column(lock))
                .map(|(l, k)| l + k)
                .all(|s| s <= 5)
            {
                count += 1;
            }
        }
    }
    println!("p1: {count}");

    Ok(())
}

fn count_column(grid: &[Vec<char>]) -> Vec<u32> {
    let res = if lock_or_key(grid) {
        grid.iter()
            .skip(1)
            .fold(vec![0; grid[0].len()], |mut acc, line| {
                for (col, cell) in line.iter().enumerate() {
                    if *cell == '#' {
                        acc[col] += 1;
                    }
                }
                acc
            })
    } else {
        grid.iter()
            .rev()
            .skip(1)
            .fold(vec![0; grid[0].len()], |mut acc, line| {
                for (col, cell) in line.iter().enumerate() {
                    if *cell == '#' {
                        acc[col] += 1;
                    }
                }
                acc
            })
    };

    // show_grid(grid);
    // println!("count: {res:?}");
    res
}

fn lock_or_key(grid: &[Vec<char>]) -> bool {
    let first_line = grid.first().unwrap();
    first_line.len() == first_line.iter().filter(|c| **c == '#').count()
}

fn show_grid(grid: &[Vec<char>]) {
    for line in grid.iter() {
        for x in line {
            print!("{}", x);
        }
        println!();
    }
}
