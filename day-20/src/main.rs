#![feature(iter_array_chunks)]
use itertools::Itertools;
use rayon::prelude::*;
use std::boxed::Box;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct State {
    cost: usize,
    position: (i32, i32),
    cheated: bool,
    landing: bool,
    prev: Option<Box<State>>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("\ncost", &self.cost)
            .field("\nposition", &self.position)
            .field("\ncheated", &self.cheated)
            .field("\nlanding", &self.landing)
            // Avoid deep recursion by only printing a summary of `prev`
            .field("\nprev", &self.prev.as_ref().map(|_| "State(...)"))
            .finish()
    }
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position && self.cheated == other.cheated
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.position.hash(state);
        self.cheated.hash(state);
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

    let base_times = shortest_path(&grid, start, end, 20);
    println!(
        "base time {:?}",
        base_times
            .iter()
            // .filter(|s| s.last().unwrap().cost == 12)
            // .inspect(|a| assert!(a.iter().unique_by(|s| s.position).count() == a.len()))
            .map(|s| 84 - s.last().unwrap().cost)
            .counts()
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1))
            .collect_vec()
    );

    Ok(())
}

fn shortest_path(
    grid: &[Vec<char>],
    start: (i32, i32),
    goal: (i32, i32),
    cheat_time: usize,
) -> Vec<Vec<State>> {
    let dims = (grid[0].len() as i32, grid.len() as i32);
    let mut fifo = VecDeque::new();
    // let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let initial_state = State {
        cost: 0,
        position: start,
        cheated: false,
        landing: false,
        prev: None,
    };

    fifo.push_back(initial_state);
    let mut results = HashSet::new();

    while let Some(state) = fifo.pop_front() {
        let State {
            cost,
            position,
            cheated,
            landing,
            ref prev,
        } = state.clone();

        if cost > 84 {
            break;
        }

        if seen.insert((position, cheated, landing, cost)) {
            // println!("state: {state:?}");

            if position == goal {
                let mut path = vec![];
                let mut cur: Option<Box<State>> = state.prev.clone();
                path.push(state.clone());

                while let Some(prev) = cur {
                    path.push(*prev.clone());
                    cur = prev.prev;
                }
                path.reverse();

                println!(
                    "({cost}) path: {:?}",
                    path.iter().map(|s| s.position).collect_vec()
                );
                show_grid(grid, &path);

                results.insert(path);
            }

            // println!("treat: {state:?}");
            let (x_min, x_max) = (0, dims.0);
            let (y_min, y_max) = (0, dims.0);

            if !cheated {
                for x in y_min..y_max {
                    for y in x_min..x_max {
                        if x == position.0 && y == position.1 {
                            continue;
                        }

                        let new_pos = (position.0 + x, position.1 + y);
                        let dist = (new_pos.0 - position.0).unsigned_abs() as usize
                            + (new_pos.1 - position.1).unsigned_abs() as usize;

                        // println!("dist: {:?}->{:?}=={}", position, new_pos, dist);

                        if new_pos.0 >= 0
                            && dist <= cheat_time
                            && new_pos.0 < dims.0
                            && new_pos.1 >= 0
                            && new_pos.1 < dims.1
                            && grid[new_pos.1 as usize][new_pos.0 as usize] != '#'
                        {
                            fifo.push_back(State {
                                cost: cost + dist,
                                position: new_pos,
                                cheated: true,
                                landing: true,
                                prev: Some(Box::new(state.clone())),
                            });
                        }
                    }
                }
            }

            for vec in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0
                    && new_pos.0 < dims.0
                    && new_pos.1 >= 0
                    && new_pos.1 < dims.1
                    && grid[new_pos.1 as usize][new_pos.0 as usize] != '#'
                {
                    fifo.push_back(State {
                        cost: cost + 1,
                        position: new_pos,
                        cheated,
                        landing: false,
                        prev: Some(Box::new(state.clone())),
                    });
                }
            }
        }
        // } else {
            // println!("{state:?} is already treated");
        // }
    }
    results.into_iter().collect_vec()
}

fn show_grid_path(grid: &[Vec<char>], path: &[(i32, i32)]) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(state) = path.iter().find(|e| **e == (x as i32, y as i32)) {
                print!("{}", "O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn show_grid(grid: &[Vec<char>], path: &[State]) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(state) = path.iter().find(|e| e.position == (x as i32, y as i32)) {
                if state.landing {
                    print!("{}", "-");
                } else {
                    print!("{}", "O");
                }
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}
