#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
    prev: Option<Box<State>>,
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
    let (time, path) = shortest_path_p2(&grid, start, end);

    let cheat_time = 20;
    // let mut saves = HashSet::new();
    let mut saves = vec![];

    // tested:  903342 too low
    // to test: 1027164 // thats the good one
    // tested:  1107433 too high
    for cell in &path {
        for to_reach in &path {
            if cell.position == to_reach.position || cell.cost >= to_reach.cost {
                continue;
            }

            let dist = (cell.position.0 - to_reach.position.0).unsigned_abs()
                + (cell.position.1 - to_reach.position.1).unsigned_abs();

            let time_save = to_reach.cost - cell.cost - dist as usize;
            if time_save == 1 {
                continue;
            }

            if dist <= cheat_time && to_reach.cost > cell.cost {

                if (dist as usize) == time_save {
                    continue;
                }

                println!(
                     "leap (dist: {dist}) from {:?} -> {:?} would save {time_save} :: time:{} cell cost:{} toreach cost:{} = {:?}",
                    cell.position,
                    to_reach.position,
                    time,
                    cell.cost,
                    to_reach.cost,
                    time_save
                );

                saves.push((cell, to_reach, time_save));
            }
        }
    }

    // for (s, e, _) in &saves {
    //     println!("--------------");
    //     show_grid(
    //         &grid,
    //         &[(s.position, e.position)]
    //     );
    //     println!("--------------");
    // }

    println!(
        "p1': {:?}",
        saves
            .iter()
            .unique()
            .map(|(a, b, c)| c)
            .filter(|total| **total >= 100)
            .counts()
            .iter()
            .sorted_by(|a, b| a.cmp(b))
            .inspect(|e| println!("la: {:?}", e))
            .map(|e| e.1)
            .sum::<usize>()
    );

    Ok(())
}

fn shortest_path_p1(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) -> usize {
    let dims = (grid[0].len() as i32, grid.len() as i32);
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let initial_state = State {
        cost: 0,
        position: start,
        prev: None,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        let State { cost, position, .. } = state;

        if position == goal {
            return cost;
        }

        if seen.insert((position, cost)) {
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
                        prev: None,
                    };

                    heap.push(next);
                }
            }
        }
    }

    unreachable!()
}

fn shortest_path_p2(
    grid: &[Vec<char>],
    start: (i32, i32),
    goal: (i32, i32),
) -> (usize, Vec<State>) {
    let dims = (grid[0].len() as i32, grid.len() as i32);
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let initial_state = State {
        cost: 0,
        position: start,
        prev: None,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        let State { cost, position, .. } = state;

        if position == goal {
            let mut path = vec![];
            path.push(state.clone());
            let mut cur = state.prev;

            while let Some(prev) = cur {
                path.push(*prev.clone());
                cur = prev.prev;
            }

            return (cost, path);
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
                        prev: Some(Box::new(state.clone())),
                    };

                    heap.push(next);
                }
            }
        }
    }

    unreachable!()
}

fn show_grid(grid: &[Vec<char>], cheats: &[((i32, i32), (i32, i32))]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if let Some(_) = cheats
                .iter()
                .find(|((x1, y1), _)| *x1 as usize == x && *y1 as usize == y)
            {
                print!("1");
            } else if let Some(_) = cheats
                .iter()
                .find(|(_, (x1, y1))| *x1 as usize == x && *y1 as usize == y)
            {
                print!("2");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}
