#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::iter::repeat;

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
        f.debug_struct("Robot").field("state", &self.state).finish() // Do not include the panel field
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

impl Robot {
    pub fn move_to(&mut self, dest: &char) -> Vec<char> {
        let sequence = shortest_path(&self.panel, &self.state, dest)
            .first()
            .unwrap()
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

    pub fn move_to_all(&mut self, dest: &char) -> Vec<Vec<char>> {
        let sequence = shortest_path(&self.panel, &self.state, dest)
            .iter()
            .map(|s| s.iter().skip(1).map(|s| s.dir).collect_vec())
            .collect_vec();

        self.state = *dest;

        sequence
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

    let robots = std::iter::repeat(prototype_robot).take(25).chain(once(Robot {
        state: 'A',
        panel: digits_keypad
    })).collect_vec();

    let p1 = codes
        .iter()
        .map(|code| (code, robots_solve(robots.as_ref(), &code)))
        .map(|(code, solution)| {
            let num_code = code
                .into_iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            num_code * solution.len() as u32
        })
        .sum::<u32>();

    println!("p1: {}", p1);

    Ok(())
}

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
        other
            .cost
            .cmp(&self.cost)
            // Im not sure here
            .then_with(|| self.output.len().cmp(&other.output.len()))
        // .then_with(|| other.composed.len().cmp(&self.composed.len()))
    }
}

impl PartialOrd for StateRobot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

        // if let Some(prev_cost) = guard.get(&state.output) {
        //     if *prev_cost <= state.cost {
        //         continue;
        //     }
        // };

        // guard.insert(state.output.clone(), state.cost);

        if !state.output.is_empty() {
            // println!("code typed: {:?}", state.output);
        }

        if state.output == goal {
            println!("cost: {}", state.cost);
            println!("{:?}", state.composed);
            return state.composed;
        }

        if seen.insert((
            state.robots.clone(),
            state.output.clone(),
            // state.composed.clone()
        )) {
            for input in &['<', '>', 'v', '^', 'A'] {
                let (new_robots, actions, output) =
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

fn manip(robots: &mut Vec<Robot>, code: &[char]) {
    for c in code {
        let actions = robots[0].move_to(c);
        println!("actions: {actions:?}");
    }
}

fn compose(robots: &mut Vec<Robot>, code: &[char], idx: usize) -> Vec<char> {
    if idx >= robots.len() {
        return code.to_vec();
    }

    let mut result = vec![];

    for c in code {
        let r = robots[idx]
            .move_to_all(c)
            .iter()
            .inspect(|e| println!("la: {e:?}"))
            .map(|path| {
                let mut robots_ = robots.clone();
                #[allow(unused_mut)]
                let mut sequence = compose(&mut robots_, path, idx + 1);
                #[allow(clippy::let_and_return)]
                // sequence.push('A');
                sequence
            })
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();

        result.extend(r);
        // result.push('A')
    }

    println!("result: {result:?}");
    result
}

fn shortest_path(grid: &[Vec<char>], current_char: &char, target_char: &char) -> Vec<Vec<State>> {
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

    // println!("starts at: {:?}, end at {:?}", start, end_pos);

    let mut min_found = usize::MAX;

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

    let mut results = vec![];

    while let Some(state) = heap.pop() {
        let State { cost, position, .. } = state;

        if cost > min_found {
            break;
        }

        if position == end_pos && cost <= min_found {
            if cost < min_found {
                min_found = cost;
            }

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

            results.push(path);
            // return path;
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

    results

    // unreachable!()
}
