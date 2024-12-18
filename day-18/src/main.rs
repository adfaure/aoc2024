#![feature(iter_array_chunks)]
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
    // prev: Option<Box<State>>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.position.hash(state);
        // Skip hashing `prev`.
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> std::io::Result<()> {
    let re_corrupt = Regex::new(r"(\d+),(\d+)").unwrap();

    let time = 12;
    let dim = (71, 71);

    let blocks = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| {
            let line = l.ok().unwrap();

            if let Some(captures) = re_corrupt.captures_iter(&line).map(|c| c.extract()).next() {
                let (_, [x, y]) = captures;
                return Some((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
            }

            None
        })
        .take(time)
        .collect::<HashSet<_>>();

    // show_grid(dim, &blocks);

    println!(
        "p1: {:?}",
        shortest_path_p1(&blocks, dim, (0, 0), (dim.0 - 1, dim.1 - 1))
    );

    let p2 = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| {
            let line = l.ok().unwrap();

            if let Some(captures) = re_corrupt.captures_iter(&line).map(|c| c.extract()).next() {
                let (_, [x, y]) = captures;
                return Some((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
            }

            None
        })
        .fold_while(vec![], |mut blocks, new_block| {
            blocks.push(new_block);
            let hash_blocks = HashSet::from_iter(blocks.clone().into_iter());

            if shortest_path_p1(&hash_blocks, dim, (0, 0), (dim.0 - 1, dim.1 - 1)) > 0 {
                Continue(blocks)
            } else {
                Done(blocks)
            }
        }).into_inner();

    println!("p2: {:?}", p2.last());

    Ok(())
}

fn shortest_path_p1(
    blocks: &HashSet<(i32, i32)>,
    dims: (i32, i32),
    start: (i32, i32),
    goal: (i32, i32),
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let initial_state = State {
        cost: 0,
        position: start,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        let State { cost, position } = state;

        if position == goal {
            return cost;
        }

        if seen.insert(position) {
            for vec in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0
                    && new_pos.0 < dims.0
                    && new_pos.1 >= 0
                    && new_pos.1 < dims.1
                    && !blocks.contains(&new_pos)
                {
                    let next = State {
                        cost: cost + 1,
                        position: new_pos,
                    };

                    heap.push(next);
                }
            }
        }
    }
    0
}

fn show_grid(dim: (i32, i32), blocks: &HashSet<(i32, i32)>) {
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if blocks.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
