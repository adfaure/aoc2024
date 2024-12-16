#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
    dir: (i32, i32),
}
struct Edge {
    node: usize,
    cost: usize,
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
    shortest_path_p1(&grid, start, end);

    let mut lol = shortest_path_p2(&grid, start, 0, (1, 0), end, HashSet::new());

    lol.sort_by(|a, b| a.0.cmp(&b.0));

    for (cost, path) in lol.iter().take(4) {
        println!("cost: {cost}");
        show_grid(&grid, path);
    }

    let min = lol[0].0;

    println!("min: {}", min);
    let best_places = lol
        .into_iter()
        .take_while(|e| e.0 == min)
        .flat_map(|(cost, path)| path)
        .collect::<HashSet<_>>();

    show_grid(&grid, &best_places);

    Ok(())
}

fn shortest_path_p2(
    grid: &[Vec<char>],
    start: (i32, i32),
    start_cost: usize,
    init_dir: (i32, i32),
    goal: (i32, i32),
    mut seen: HashSet<((i32, i32), (i32, i32))>,
) -> Vec<(i32, HashSet<(i32, i32)>)> {
    let mut heap = BinaryHeap::new();

    // to retrieve the path
    let mut paths: HashMap<(i32, i32), Option<State>> = HashMap::new();

    let dims = (grid[0].len() as i32, grid.len() as i32);

    let initial_state = State {
        cost: start_cost,
        position: start,
        dir: init_dir,
    };

    paths.insert(start, None);


    // println!("At: {:?} dir: {:?}", start, init_dir);

    heap.push(initial_state);

    let mut best_position = HashSet::new();
    let mut answers = vec![];

    while let Some(state) = heap.pop() {
        let cost = state.cost;
        let current_dir = state.dir;
        let position = state.position;

        if position == goal {
            // println!("found goal from: {start:?} {current_dir:?}");
            // That means that we reach the goal
            // We are likely to be in a subpath
            let mut traceback = position;
            let mut path = vec![];

            while let Some(Some(prev)) = paths.get(&traceback) {
                path.push(prev);
                traceback = prev.position;
                best_position.insert(prev.position);
            }

            answers.push(
                (cost as i32,
                HashSet::from_iter(path.iter().map(|s| s.position)))
            );
        }

        if seen.insert((position, current_dir)) {
            let mut todos = vec![];

            for vec in [current_dir, (current_dir.1, -current_dir.0), (-current_dir.1, current_dir.0)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                    let cell = grid[new_pos.1 as usize][new_pos.0 as usize];
                    if cell == '#' {
                        continue;
                    } else {
                        let score = if vec == current_dir {
                            cost + 1
                        } else {
                            cost + 1000 + 1
                        };

                        let next = State {
                            cost: score,
                            position: new_pos,
                            dir: vec,
                        };

                        match paths.get(&new_pos) {
                            Some(Some(current_state)) => {
                                if score < current_state.cost {
                                    paths.insert(new_pos, Some(state));
                                }
                            }
                            Some(_) => {
                                // unreachable!()
                            }
                            None => {
                                paths.insert(new_pos, Some(state));
                            }
                        };

                        todos.push(next);
                    }
                }
                // println!("todos: {todos:?}");

                #[allow(clippy::comparison_chain)]
                if todos.len() > 1 {
                    // We are likely to be in a subpath
                    let mut traceback = position;
                    let mut path = vec![];
                    path.push(&state);

                    while let Some(Some(prev)) = paths.get(&traceback) {
                        path.push(prev);
                        traceback = prev.position;
                        best_position.insert(prev.position);
                    }

                    let path_set: HashSet<(i32, i32)> =
                        HashSet::from_iter(path.iter().map(|s| s.position));

                    let mut all_subpaths = vec![];

                    for todo in &todos {
                        let results = shortest_path_p2(
                            grid,
                            todo.position,
                            todo.cost,
                            todo.dir,
                            goal,
                            seen.clone(),
                        )
                        .into_iter()
                        .filter(|e| e.0 > 0)
                        .map(|(cost, sub_path)| {
                            (
                                cost,
                                HashSet::from_iter(
                                    path_set
                                        .clone()
                                        .into_iter()
                                        .chain(sub_path.clone().into_iter()),
                                ),
                            )
                        });

                        all_subpaths.extend(results);
                    }
                    answers.extend(all_subpaths);
                } else if todos.len() == 1 {
                    heap.push(todos[0])
                }
            }
        } else {
            // println!("Already seen :{state:?}");
        }
    }

    // println!("could not reach goal from {start:?} with {init_dir:?}");
    answers
}

fn shortest_path_p1(grid: &[Vec<char>], start: (i32, i32), goal: (i32, i32)) {
    // dist[node] = current shortest distance from `start` to `node`
    let dir = (1, 0);

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let dims = (grid[0].len() as i32, grid.len() as i32);

    let initial_state = State {
        cost: 0,
        position: start,
        dir,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        let State {
            cost,
            position,
            dir,
        } = state;

        if position == goal {
            println!("p1: {}", cost);
            return;
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
                        };

                        heap.push(next);
                    }
                }
            }
        }
    }
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
