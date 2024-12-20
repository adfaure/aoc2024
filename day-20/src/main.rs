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
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .take_while(|l| l.is_ok())
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();

    let start: (i32, i32) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let end: (i32, i32) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'E' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let base_time = shortest_path_p1(&grid, start, end);

    let times = grid
        .par_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| {
                    if grid[y][x] == '#' {
                        let mut cheat = grid.clone();
                        cheat[y][x] = '.';
                        base_time - shortest_path_p1(&cheat, start, end)
                    } else {
                        0
                    }
                })
                .collect_vec()
        })
        .filter(|saved| *saved >= 100)
        // .inspect(|e| println!("{e:?}"))
        .count();

    println!("p1: {:?}", times);

    Ok(())
}

fn shortest_path_p1(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) -> usize {
    let dims = (grid[0].len() as i32, grid.len() as i32);
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
                    && grid[new_pos.1 as usize][new_pos.0 as usize] != '#'
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
