#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::boxed::Box;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
    dir: (i32, i32),
    prev: Option<Box<State>>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cost.hash(state);
        self.position.hash(state);
        self.dir.hash(state);
        // Skip hashing `prev`.
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.dir.cmp(&other.dir))
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
        .map_while(|l| {
            let line = l.ok();

            if line.is_some() && !line.clone().unwrap().is_empty() {
                return Some(line.unwrap().clone());
            }

            None
        })
        .map(|line| line.clone().chars().collect_vec())
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

    // Should print the p1 on its own
    let min = shortest_path_p1(&grid, start, end, (1, 0));

    let best = shortest_path_p2_recurse(
        &grid,
        start,
        (1, 0),
        0,
        end,
        HashSet::new(),
        HashSet::new(),
    );
    // show_grid(&grid, &best.clone().unwrap().1);

    println!("p2: {}", best.unwrap().1.len());
    Ok(())
}

fn shortest_path_p2_recurse(
    grid: &[Vec<char>],
    start: (i32, i32),
    start_dir: (i32, i32),
    start_cost: usize,
    goal: (i32, i32),
    path_to: HashSet<(i32, i32)>,
    mut seen: HashSet<((i32, i32), (i32, i32))>,
) -> Option<(i32, HashSet<(i32, i32)>)> {
    // println!("Starting with: {start:?} {start_dir:?} {goal:?}, {start_cost}");

    let mut heap = BinaryHeap::new();
    let mut results = HashSet::new();

    let dims = (grid[0].len() as i32, grid.len() as i32);

    let initial_state = State {
        cost: start_cost,
        position: start,
        prev: None,
        dir: start_dir,
    };

    heap.push(initial_state);

    let min = shortest_path_p1(grid, start, goal, start_dir);

    while let Some(state) = heap.pop() {
        let State {
            cost,
            position,
            dir,
            ref prev,
        } = state;

        if cost > min + start_cost {
            return None;
        }

        let mut path: Vec<State> = vec![state.clone()];
        let mut cur: Option<Box<State>> = state.prev.clone();
        let mut current_best_pos = HashSet::new();

        while let Some(prev) = cur {
            path.push(*prev.clone());
            cur = prev.prev;
        }

        current_best_pos.extend(path_to.clone().iter());
        current_best_pos.extend(path.iter().map(|state| state.position));

        if position == goal {
            // println!("found");
            results.extend(path_to.clone().iter());
            results.extend(path.iter().map(|state| state.position));

            return Some((cost as i32, results));
        }

        if seen.insert((position, dir)) {
            let mut todos = vec![];

            for vec in [dir, (dir.1, -dir.0), (-dir.1, dir.0)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                    let cell = grid[new_pos.1 as usize][new_pos.0 as usize];
                    if cell == '#' {
                        continue;
                    } else {
                        let score = if vec == dir {
                            cost + 1
                        } else {
                            cost + 1000 + 1 //FIXME: not sure about this +1
                        };

                        let next = State {
                            cost: score,
                            position: new_pos,
                            dir: vec,
                            prev: Some(Box::new(state.clone())),
                        };

                        // fifo.push_front(next);
                        if !path_to.contains(&next.position) {
                            todos.push(next);
                        }
                    }
                }
            }

            #[allow(clippy::comparison_chain)]
            if todos.len() == 1 {
                heap.push(todos[0].clone());
            } else if todos.len() > 1 {
                let mut sub_paths = vec![];
                for todo in todos {

                    // Check if path from here
                    let min_split = shortest_path_p1(grid, todo.position, goal, todo.dir);
                    // println!("min split: {}: min: {}", todo.cost + min_split, start_cost + min);

                    if todo.cost + min_split == start_cost + min {
                        let sub_result = shortest_path_p2_recurse(
                            grid,
                            todo.position,
                            todo.dir,
                            todo.cost,
                            goal,
                            current_best_pos.clone(),
                            seen.clone(),
                        );

                        if let Some(sub) = sub_result {
                            sub_paths.push(sub.clone());
                            if sub.0 == start_cost as i32 + min as i32 {
                                results.extend(sub.1);
                            }
                        }
                    }
                }

                // println!("save: {:?}", ((start), (start_dir), start_cost));
                return Some((start_cost as i32 + min as i32, results));
            }
        }
    }

    None
}

fn shortest_path_p2(
    grid: &[Vec<char>],
    start: (i32, i32),
    start_dir: (i32, i32),
    goal: (i32, i32),
    min: usize,
) {
    let mut fifo = VecDeque::new();
    let mut results = HashSet::new();

    let dims = (grid[0].len() as i32, grid.len() as i32);

    let initial_state = State {
        cost: 0,
        position: start,
        prev: None,
        dir: start_dir,
    };

    fifo.push_back(initial_state);

    while let Some(state) = fifo.pop_front() {
        let State {
            cost,
            position,
            dir,
            ref prev,
        } = state;

        if cost > min {
            // already greater than the min path
            continue;
        }

        let mut path: Vec<State> = vec![state.clone()];
        let mut cur: Option<Box<State>> = state.prev.clone();

        while let Some(prev) = cur {
            path.push(*prev.clone());
            cur = prev.prev;
        }

        let current_path: HashSet<(i32, i32)> =
            HashSet::from_iter(path.iter().map(|state| state.position));
        if current_path.len() < path.len() {
            // That means that we passe two time on the same cell
            continue;
        }

        let min_to_current = shortest_path_p1(grid, start, position, start_dir);
        let min_to_end = shortest_path_p1(grid, position, goal, dir);

        if cost > min_to_current + min_to_end {
            // println!("cut min chelou");
            continue;
        }

        if position == goal {
            results.extend(path.iter().map(|state| state.position));
        }

        for vec in [dir, (dir.1, -dir.0), (-dir.1, dir.0)] {
            let new_pos = (position.0 + vec.0, position.1 + vec.1);
            if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                let cell = grid[new_pos.1 as usize][new_pos.0 as usize];
                if cell == '#' {
                    continue;
                } else {
                    let score = if vec == dir {
                        cost + 1
                    } else {
                        cost + 1000 + 1 //FIXME: not sure about this +1
                    };

                    let next = State {
                        cost: score,
                        position: new_pos,
                        dir: vec,
                        prev: Some(Box::new(state.clone())),
                    };

                    fifo.push_front(next);
                }
            }
        }
    }

    println!("p2: {}", results.len());
}

fn shortest_path_p1(
    grid: &[Vec<char>],
    start: (i32, i32),
    goal: (i32, i32),
    dir: (i32, i32),
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let dims = (grid[0].len() as i32, grid.len() as i32);

    let initial_state = State {
        cost: 0,
        position: start,
        dir,
        prev: None,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        let State {
            cost,
            position,
            dir,
            prev,
        } = state;

        if position == goal {
            return cost;
        }

        if seen.insert((position, dir)) {
            for vec in [dir, (dir.1, -dir.0), (-dir.1, dir.0)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                    let cell = grid[new_pos.1 as usize][new_pos.0 as usize];
                    if cell == '#' {
                        continue;
                    } else {
                        let score = if vec == dir {
                            cost + 1
                        } else {
                            cost + 1000 + 1 //FIXME: not sure about this +1
                        };

                        let next = State {
                            cost: score,
                            position: new_pos,
                            dir: vec,
                            prev: None,
                        };

                        heap.push(next);
                    }
                }
            }
        }
    }
    0
}

fn show_grid(grid: &[Vec<char>], paths: &HashSet<(i32, i32)>) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if paths.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!();
    }
}
