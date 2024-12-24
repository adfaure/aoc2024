#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::once;

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
    dir: char,
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

#[derive(Clone)]
struct Robot {
    state: char,
    panel: Vec<Vec<char>>,
}

impl fmt::Debug for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = if self.panel.get(0).and_then(|row| row.get(0)) == Some(&'7') {
            "Numpad"
        } else {
            "Arrowpad"
        };

        // Write a custom debug format
        f.write_fmt(format_args!("{}Robot {{ state: {:?} }}", kind, self.state))
    }
}

impl Hash for Robot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
        // Skip hashing `prev`.
    }
}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for Robot {}

#[derive(Clone, Debug)]
struct StateRobot {
    cost: usize,
    composed: Vec<char>,
    output: Vec<char>,
    robots: Vec<Robot>,
}

// Implement PartialEq and Eq based on `state` only
impl PartialEq for StateRobot {
    fn eq(&self, other: &Self) -> bool {
        self.robots == other.robots
            && self.composed == other.composed
            && self.output == other.output
    }
}

impl Eq for StateRobot {}

impl Hash for StateRobot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.robots.hash(state);
        self.composed.hash(state);
        self.output.hash(state);
        // self.cost.hash(state);
        // Skip hashing `prev`.
    }
}

impl Ord for StateRobot {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // Im not sure here
        // .then_with(|| self.output.len().cmp(&other.output.len()))
        // .then_with(|| other.composed.len().cmp(&self.composed.len()))
    }
}

impl PartialOrd for StateRobot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Robot {
    pub fn adjacents(&self) -> Vec<(char, char)> {
        let dims = (self.panel[0].len() as i32, self.panel.len() as i32);
        let start = self
            .panel
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .position(|&c| c == self.state)
                    .map(|x| (x as i32, y as i32))
            })
            .unwrap();

        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .filter_map(|dir| {
                let new_pos = (start.0 + dir.0, start.1 + dir.1);
                if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
                    let new_state = self.panel[new_pos.1 as usize][new_pos.0 as usize];
                    if new_state != '.' {
                        return Some((vec_to_dir(*dir), new_state));
                    }
                }
                None
            })
            .collect_vec()
    }

    pub fn move_to(&mut self, dest: &char) -> Vec<char> {
        let sequence = shortest_path(&self.panel, &self.state, dest)
            .iter()
            .skip(1)
            .map(|s| s.dir)
            .collect_vec();

        self.state = *dest;

        sequence
    }

    pub fn move_once(&mut self, dir: &char) -> Option<char> {
        let vec = match dir {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            'A' => return None,
            _ => {
                unreachable!()
            }
        };

        let dims = (self.panel[0].len() as i32, self.panel.len() as i32);
        let start = self
            .panel
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .position(|&c| c == self.state)
                    .map(|x| (x as i32, y as i32))
            })
            .unwrap();

        let new_pos = (start.0 + vec.0, start.1 + vec.1);
        if new_pos.0 >= 0 && new_pos.0 < dims.0 && new_pos.1 >= 0 && new_pos.1 < dims.1 {
            let new_state = self.panel[new_pos.1 as usize][new_pos.0 as usize];
            if new_state != '.' {
                self.state = new_state;
                return Some(new_state);
            }
        }

        None
        // assert!(new_pos.0 >= 0 && new_pos.0 < dims.0, "new_pos: {new_pos:?}, dims: {dims:?}");
        // assert!(new_pos.1 >= 0 && new_pos.1 < dims.1);
        // assert!(new_state != '.');
    }

    pub fn press(&self, other: &mut Self) -> char {
        match self.state {
            'A' => return other.state,
            '>' | '<' | '^' | 'v' => {
                other.move_once(&self.state);
            }
            '0'..='9' => return self.state,
            _ => {
                unreachable!()
            }
        };

        '.'
    }

    pub fn move_to_all(&mut self, dest: &char) -> Vec<char> {
        let s = shortest_path(&self.panel, &self.state, dest)
            .iter()
            .skip(1)
            .map(|s| s.dir)
            .collect_vec();
        self.state = *dest;

        s
    }
}

// @me -> robot1 -> robot2 -> robot 3
fn main() -> std::io::Result<()> {
    let codes = BufReader::new(File::open("input")?)
        .lines()
        .take_while(|l| l.is_ok())
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();

    println!("codes: {:?}", codes);

    let arrow_keypad = vec![vec!['.', '^', 'A'], vec!['<', 'v', '>']];

    let digits_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['.', '0', 'A'],
    ];

    // Create a single prototype robot
    let prototype_robot = Robot {
        state: 'A',
        panel: arrow_keypad.clone(),
    };

    // let robots = std::iter::repeat(prototype_robot.clone()).take(2).chain(once(Robot {
    //     state: 'A',
    //     panel: digits_keypad.clone()
    // })).collect_vec();
    //
    // let p1 = codes
    //     .iter()
    //     .map(|code| (code, robots_solve_p2(robots.as_ref(), code)))
    //     .map(|(code, solution)| {
    //         let num_code = code
    //             .iter()
    //             .take_while(|c| c.is_ascii_digit())
    //             .collect::<String>()
    //             .parse::<u32>()
    //             .unwrap();

    //         num_code * solution.len() as u32
    //     })
    //     .sum::<u32>();

    // println!("p1: {}", p1);

    let robots = std::iter::repeat(prototype_robot.clone())
        .take(3)
        .chain(once(Robot {
            state: 'A',
            panel: digits_keypad,
        }))
        .collect_vec();

    println!("Try recurse");
    let test = robots_solve_p2(robots.as_ref(), &['0', '2', '9', 'A']);
    let test2 = robots_solve(robots.as_ref(), &['0', '2', '9', 'A']);
    // let test = robots_solve_p2(robots.as_ref(), &['0']);
    println!(
        "test : {} ({}) cost={}\ntest2: {} ({})",
        test.1.iter().join(""),
        test.1.len(),
        test.2,
        test2.iter().join(""),
        test2.len()
    );
    // println!("Try recurse");

    // let t = ['0', '2', '9', 'A']
    //     .iter()
    //     .fold((robots.to_vec(), 0), |acc, c| {
    //         let (r, l) = robots_solve_p2(&acc.0, &[*c]);
    //         (r, acc.1 + l.len())
    //     });

    // println!("test: {t:?}");

    // let p2 = codes
    //     .iter()
    //     .map(|code| (code, robots_solve_p2(robots.as_ref(), code)))
    //     .map(|(code, solution)| {
    //         let num_code = code
    //             .iter()
    //             .take_while(|c| c.is_ascii_digit())
    //             .collect::<String>()
    //             .parse::<u32>()
    //             .unwrap();

    //         num_code * solution.len() as u32
    //     })
    //     .sum::<u32>();

    // println!("p2: {}", p2);

    Ok(())
}

fn robots_solve_p2(init_robots: &[Robot], goal: &[char]) -> (Vec<Robot>, Vec<char>, usize) {
    println!(
        "Should solve: {:?} with robots: {:?}",
        goal.clone().into_iter().join(""),
        init_robots
    );
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let init = StateRobot {
        cost: 0,
        composed: vec![],
        output: vec![],
        robots: init_robots.to_vec(),
    };

    heap.push(init);
    while let Some(state) = heap.pop() {
        println!(
            "\tPoped ({}): robots: {:?} composed: {:?}: output: {:?}",
            state.cost, state.robots, state.composed, state.output
        );
        if !goal.starts_with(&state.output) {
            println!("\t\tCa dégage");
            continue;
        }

        if !state.output.is_empty() {
            // println!("code typed: {:?}", state.output);
        }

        if state.output == goal {
            println!("Found cost: {}", state.cost);
            println!("\tpath: {}", state.composed.clone().into_iter().join(""));
            let check = robots_solve(init_robots, goal);
            // assert!(check == state.composed);
            println!("\tpath: {} <- check p1", check.iter().join(""));
            return (state.robots, state.composed, state.cost);
        }

        if seen.insert((
            state.robots.clone(),
            state.output.clone(),
            // state.composed.clone()
        )) {
            let mut last_robot = state.robots[state.robots.len() - 1].clone();
            let next_inputs = last_robot.adjacents();

            println!("\tnext_inputs: {:?}", next_inputs);

            for (input, new_state) in next_inputs.into_iter().chain(once(('A', last_robot.state))) {
                let mut new_robots = state.robots.to_vec().clone();

                let (new_robots, move_cost) = if new_robots.len() == 1 {
                    // Do nothing if current state is A
                    new_robots[0].move_once(&input);
                    (new_robots, 1) // shortest_path(&robots[0].panel, &robots[0].state, &input.0).len()
                } else {
                    let mut robots_before = new_robots.clone();
                    robots_before.truncate(new_robots.len() - 1);
                    let mut robot_and_pash_and_cost = robots_solve_p2(&robots_before, &[input]);

                    last_robot.state = new_state;
                    robot_and_pash_and_cost.0.push(last_robot.clone());
                    (robot_and_pash_and_cost.0, robot_and_pash_and_cost.2)
                };

                println!(
                    "\t\t{input} leading to new state: {new_state} with a cost of {move_cost}"
                );
                println!("\t\t\t robots: {:?}", state);
                let mut new_output = state.output.clone();
                if input == 'A' {
                    new_output.push(new_state);
                }

                let mut new_composed = state.composed.clone();
                new_composed.push(input);

                let new_state = StateRobot {
                    cost: state.cost + move_cost,
                    robots: new_robots,
                    output: new_output,
                    composed: new_composed,
                };
                println!("\t\tPushing: {new_state:?}");

                heap.push(new_state);
            }

            continue;
            for input in &['<', '>', 'v', '^', 'A'] {
                let (new_robots, _, output) =
                    propagate_action(&state.robots, 0, input, vec![], None);

                // println!("result: {:?}", new_robots);
                // println!("result: {:?}", actions);
                let mut new_composed = state.composed.clone();
                // new_composed.extend(actions);
                new_composed.push(*input);

                let mut new_output = state.output.clone();
                if let Some(out) = output {
                    new_output.push(out);
                }

                let mut robots_before = new_robots.clone();
                robots_before.truncate(new_robots.len() - 1);

                let move_cost = if init_robots.len() == 1 {
                    1
                } else {
                    robots_solve_p2(&robots_before, &[*input]).1.len()
                };

                heap.push(StateRobot {
                    // A bit more A*ish
                    cost: move_cost,
                    robots: new_robots,
                    output: new_output,
                    composed: new_composed,
                });
            }
        } else {
            // println!("\tSeen state: {state:?}");
        }
    }
    println!(
        "Cannot solve: {:?} with robots: {:?}",
        goal.iter().join(""),
        init_robots
    );
    unreachable!()
}

// Handle case where first robot press A
fn propagate_action(
    robots: &[Robot],
    idx: usize,
    input: &char,
    // Out parameters for term recursivity
    mut acc: Vec<char>,
    output: Option<char>,
) -> (Vec<Robot>, Vec<char>, Option<char>) {
    if idx >= robots.len() {
        return (robots.to_vec(), acc, output);
    }

    match input {
        '<' | '>' | 'v' | '^' => {
            // println!("Moving ({input}) {:?}", robots[idx]);
            let mut new_robots = robots.to_vec();
            if new_robots[idx].move_once(input).is_some() {
                return (new_robots, acc, None);
            }

            (robots.to_vec(), acc, None)
        }
        'A' => {
            let mut out = None;
            if idx == robots.len() - 1 {
                out = Some(robots[idx].state);
            } else {
                acc.push(robots[idx].state);
            }
            propagate_action(robots, idx + 1, &robots[idx].state, acc, out)
        }
        _ => unreachable!(),
    }
}

fn dir_to_vec(dir: &char) -> (i32, i32) {
    match dir {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => {
            unreachable!()
        }
    }
}

fn vec_to_dir(vec: (i32, i32)) -> char {
    match vec {
        (1, 0) => '>',
        (-1, 0) => '<',
        (0, -1) => '^',
        (0, 1) => 'v',
        _ => {
            unreachable!()
        }
    }
}

fn shortest_path(grid: &[Vec<char>], current_char: &char, target_char: &char) -> Vec<State> {
    let start: (i32, i32) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == *current_char {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let end_pos: (i32, i32) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == *target_char {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let dims = (grid[0].len() as i32, grid.len() as i32);
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let initial_state = State {
        cost: 0,
        position: start,
        dir: '.',
        prev: None,
    };

    heap.push(initial_state);
    while let Some(state) = heap.pop() {
        let State { cost, position, .. } = state;

        if position == end_pos {
            let mut path = vec![];
            let mut cur = state.prev.clone();

            path.push(state.clone());

            while let Some(prev) = cur {
                path.push(*prev.clone());
                cur = prev.prev;
            }

            path.reverse();

            // more readable output
            for state in path.iter_mut() {
                state.prev = None
            }

            return path;
        }

        if seen.insert((position, cost)) {
            for vec in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let new_pos = (position.0 + vec.0, position.1 + vec.1);
                if new_pos.0 >= 0
                    && new_pos.0 < dims.0
                    && new_pos.1 >= 0
                    && new_pos.1 < dims.1
                    && grid[new_pos.1 as usize][new_pos.0 as usize] != '.'
                {
                    let dir_char = match vec {
                        (0, 1) => 'v',
                        (1, 0) => '>',
                        (0, -1) => '^',
                        (-1, 0) => '<',
                        _ => unreachable!(),
                    };

                    let next = State {
                        cost: cost + 1,
                        position: new_pos,
                        dir: dir_char,
                        prev: Some(Box::new(state.clone())),
                    };

                    heap.push(next);
                }
            }
        }
    }

    unreachable!()
}

fn robots_solve(robots: &[Robot], goal: &[char]) -> Vec<char> {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let init = StateRobot {
        cost: 0,
        composed: vec![],
        output: vec![],
        robots: robots.to_vec(),
    };

    heap.push(init);
    while let Some(state) = heap.pop() {
        // println!("cost: {}", state.cost);
        if !goal.starts_with(&state.output) {
            continue;
        }

        if !state.output.is_empty() {
            // println!("code typed: {:?}", state.output);
        }

        if state.output == goal {
            // println!("cost: {}", state.cost);
            // println!("{:?}", state.composed);
            return state.composed;
        }

        if seen.insert((
            state.robots.clone(),
            state.output.clone(),
            // state.composed.clone()
        )) {
            for input in &['<', '>', 'v', '^', 'A'] {
                let (new_robots, _, output) =
                    propagate_action(&state.robots, 0, input, vec![], None);

                // println!("result: {:?}", new_robots);
                // println!("result: {:?}", actions);
                let mut new_composed = state.composed.clone();
                // new_composed.extend(actions);
                new_composed.push(*input);

                let mut new_output = state.output.clone();
                if let Some(out) = output {
                    new_output.push(out);
                }

                heap.push(StateRobot {
                    cost: new_composed.len(),
                    robots: new_robots,
                    output: new_output,
                    composed: new_composed,
                });
            }
        }
    }

    unreachable!()
}
