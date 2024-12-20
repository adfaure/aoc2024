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
use std::iter::once;

#[derive(Clone)]
struct State {
    cost: usize,
    position: (i32, i32),
    cheat: Vec<(i32, i32)>, // prev: Option<Box<State>>,
    cheated: bool,
    cheating: bool,
    start_cheat: bool,
    end_cheat: bool,
    trim: usize,
    prev: Option<Box<State>>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("\ncost", &self.cost)
            .field("\nposition", &self.position)
            .field("\ncheat", &self.cheat)
            .field("\ncheated", &self.cheated)
            .field("\nstart cheated", &self.start_cheat)
            .field("\ncheating", &self.cheating)
            .field("\nend cheated", &self.end_cheat)
            // Avoid deep recursion by only printing a summary of `prev`
            .field("\nprev", &self.prev.as_ref().map(|_| "State(...)"))
            .finish()
    }
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && self.position == other.position
            && self.cheat == other.cheat
            && self.cheated == other.cheated
            && self.end_cheat == other.end_cheat
            && self.start_cheat == other.start_cheat
            && self.cheating == other.cheating
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.position.hash(state);
        self.cheat.hash(state);
        self.cheated.hash(state);
        self.start_cheat.hash(state);
        self.end_cheat.hash(state);
        self.cheating.hash(state);
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
            .filter(|a| a.last().unwrap().cost == 10)
            .filter(|a| a.iter().filter(|e| e.position == end).count() == 1)
            .inspect(|a| assert!(a.iter().unique_by(|s| s.position).count() == a.len()))
            .enumerate()
            .unique_by(|(idx, a)| {
                let start = a.iter().find(|a| a.start_cheat);
                let end = a.iter().find(|a| a.end_cheat);

                // println!("-------------vvvvv----------------");
                // show_grid(&grid, a);
                // show_grid_path(&grid, &a.last().unwrap().cheat);

                let print_path = a.iter().map(|s| s.position).collect_vec();
                // println!("end path: {:?}", print_path);

                let rs = (
                    start.unwrap().position,
                    end.unwrap().position,
                    a.last().unwrap().cost,
                    a.last().unwrap().position,
                    a.last().unwrap().cheated,
                    // a.last().unwrap().start_cheat,
                    // a.last().unwrap().end_cheat,
                    // a.last().unwrap().cheating,
                );

                // println!("rs: {rs:?}");
                // println!("-------------^^^^^----------------");
                rs
            })
            .inspect(|(idx, a)| {
                let start = a.iter().find(|a| a.start_cheat);
                let end = a.iter().find(|a| a.end_cheat);

                println!("Should be unique by now");
                println!("-------------vvvvv----------------");
                show_grid(&grid, a);
                // show_grid_path(&grid, &a.last().unwrap().cheat);

                let print_path = a.iter().map(|s| s.position).collect_vec();
                println!("end path: {:?}", print_path);

                let rs = (
                    start.unwrap().position,
                    end.unwrap().position,
                    a.last().unwrap().cost,
                    a.last().unwrap().position,
                    a.last().unwrap().cheated,
                    // a.last().unwrap().start_cheat,
                    // a.last().unwrap().end_cheat,
                    // a.last().unwrap().cheating,
                );

                println!("rs: {rs:?}");
                println!("-------------^^^^^----------------");
            })
            .map(|(_, s)| 84 - s.last().unwrap().cost)
            .counts()
            .iter()
            .sorted_by(|a, b| a.1.cmp(b.1))
            .collect_vec() // .collect_vec()
    );

    // let test = vec![(1, 3), (1, 4), (1, 5), (1, 6), (2, 6), (2, 7), (2, 8), (3, 8), (3, 7), (4, 7)];

    // show_grid_path(&grid, &test);
    // println!("pat: \n{test:?}\n{:?}", trim_cheat_path(&grid, &test));
    // let times = shortest_path(&grid, start, end, 1).iter().filter()

    Ok(())
}

fn trim_cheat_path(grid: &[Vec<char>], path: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut res = path.to_vec();

    if let Some(first) = path.first() {
        // assert!(grid[first.1 as usize][first.0 as usize] == '#')
    }

    if let Some(last) = path.last() {
        // println!("yea");
        if grid[last.1 as usize][last.0 as usize] != '#' {
            // println!("yea yeah");
            let trim_end = path
                .iter()
                // .inspect(|e| println!("{e:?}"))
                .rev()
                .position(|pos| grid[pos.1 as usize][pos.0 as usize] == '#');

            if let Some(trim_idx) = trim_end {
                if trim_idx < path.len() {
                    res.resize(path.len() - trim_idx + 1, (0, 0));
                }
                let check = res.last().unwrap();
                assert!(grid[check.1 as usize][check.0 as usize] != '#')
            }
        }
    }

    res
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
        cheat: vec![],
        cheated: false,
        start_cheat: false,
        end_cheat: false,
        cheating: false,
        trim: 0,
        prev: None,
    };

    let initial_state_cheating = State {
        cost: 0,
        position: start,
        cheat: vec![start],
        cheated: false,
        start_cheat: true,
        end_cheat: false,
        cheating: true,
        trim: 0,
        prev: None,
    };
    fifo.push_back(initial_state_cheating);

    // fifo.push_back(initial_state);
    fifo.push_back(initial_state);

    let mut results = HashSet::new();

    while let Some(state) = fifo.pop_front() {
        // while let Some(state) = heap.pop() {
        let State {
            cost,
            position,
            cheat,
            cheated,
            start_cheat,
            end_cheat,
            cheating,
            trim,
            ref prev,
        } = state.clone();

        if cost > 16 {
            break;
        }

        if cheat.len() > cheat_time {
            // Nop
            continue;
        }

        let e;
        let s;
        if cheat.is_empty() {
            s = None;
            e = None;
        } else {
            s = cheat.first().cloned();
            e = cheat.last().cloned();
        }

        if position == goal {
            if cheating {
                continue;
            }

            let mut path = vec![];
            let mut cur: Option<Box<State>> = state.prev.clone();

            let mut final_state = state.clone();
            final_state.cheat = trim_cheat_path(grid, &cheat);

            path.push(final_state.clone());

            while let Some(prev) = cur {
                path.push(*prev.clone());
                cur = prev.prev;
            }

            path.reverse();

            // for s in &path {
            //     println!("{s:?}");
            // }

            let find_end_pos = path.iter().position(|s| s.end_cheat);

            if let Some(end) = find_end_pos {
                let trim_size = path
                    .iter()
                    .rev()
                    .skip_while(|s| !s.end_cheat)
                    .take_while(|s| grid[s.position.1 as usize][s.position.0 as usize] != '#')
                    .count();
            }
            results.insert(path);
        }

        if seen.insert((position, cheated, start_cheat, end_cheat, cheating, trim, s, e)) {
            for vec in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                    // Cheating time
                    if start_cheat {
                        if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                            let mut cheating_path = cheat.clone();
                            cheating_path.push(new_pos);

                            fifo.push_back(State {
                                cost: cost + 1,
                                position: new_pos,
                                cheat: cheating_path.clone(),
                                cheated: false,
                                start_cheat: false,
                                end_cheat: false,
                                cheating: true,
                                trim: 0,
                                prev: Some(Box::new(state.clone())),
                            });
                        }
                    } else if cheating {
                        if grid[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                            let mut cheating_path = cheat.clone();
                            cheating_path.push(new_pos);

                            fifo.push_back(State {
                                cost: cost + 1,
                                position: new_pos,
                                cheat: cheating_path.clone(),
                                cheated: false,
                                start_cheat: false,
                                end_cheat: false,
                                cheating: true,
                                trim: trim + 1,
                                prev: Some(Box::new(state.clone())),
                            });

                            if trim == 0 {
                                fifo.push_back(State {
                                    cost: cost + 1,
                                    position: new_pos,
                                    cheat: cheating_path.clone(),
                                    cheated: true,
                                    start_cheat: false,
                                    end_cheat: true,
                                    trim: 0,
                                    cheating: false,
                                    prev: Some(Box::new(state.clone())),
                                });
                            }
                        } else {
                            let mut cheating_path = cheat.clone();
                            cheating_path.push(new_pos);

                            fifo.push_back(State {
                                cost: cost + 1,
                                position: new_pos,
                                cheat: cheating_path,
                                cheated: false,
                                cheating: true,
                                start_cheat: false,
                                end_cheat: false,
                                trim: 0,
                                prev: Some(Box::new(state.clone())),
                            });
                        }
                    } else if grid[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                        fifo.push_back(State {
                            cost: cost + 1,
                            position: new_pos,
                            cheat: cheat.clone(),
                            cheated,
                            start_cheat: false,
                            end_cheat: false,
                            cheating: false,
                            trim: 0,
                            prev: Some(Box::new(state.clone())),
                        });

                        if !cheated {
                            fifo.push_back(State {
                                cost: cost + 1,
                                position: new_pos,
                                cheat: vec![new_pos],
                                cheated,
                                start_cheat: true,
                                end_cheat: false,
                                cheating: true,
                                trim: 0,
                                prev: Some(Box::new(state.clone())),
                            });
                        }
                    }
                }
            }
        }
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
                if state.start_cheat {
                    print!("+");
                } else if state.end_cheat {
                    print!("-");
                } else if state.cheating {
                    print!("@")
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
