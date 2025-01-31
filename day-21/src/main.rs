#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::once;

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

    let robots = std::iter::repeat(prototype_robot.clone())
        .take(2)
        .chain(once(Robot {
            state: 'A',
            panel: digits_keypad.clone(),
        }))
        .collect_vec();

    let p1 = codes
        .iter()
        .map(|code| {
            (
                code,
                robots_solve_p2(robots.as_ref(), code, &mut HashMap::new()).1,
            )
        })
        .map(|(code, solution)| {
            let num_code = code
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            num_code * solution as u64
        })
        .sum::<u64>();

    println!("p1: {}", p1);

    let robots = std::iter::repeat(prototype_robot.clone())
        .take(25)
        .chain(once(Robot {
            state: 'A',
            panel: digits_keypad,
        }))
        .collect_vec();

    let p2 = codes
        .iter()
        .map(|code| {
            (
                code,
                robots_solve_p2(robots.as_ref(), code, &mut HashMap::new()).1,
            )
        })
        .map(|(code, solution)| {
            let num_code = code
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            num_code * solution as u64
        })
        .sum::<u64>();

    println!("p2: {}", p2);

    Ok(())
}

fn robots_solve_p2(
    init_robots: &[Robot],
    goal: &[char],
    memo: &mut HashMap<(Vec<Robot>, Vec<char>), (Vec<Robot>, usize)>,
) -> (Vec<Robot>, usize) {
    if let Some(res) = memo.get(&(init_robots.to_vec(), goal.to_vec())) {
        return res.clone();
    }

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
        if !goal.starts_with(&state.output) {
            continue;
        }

        if state.output == goal {
            let result = (state.robots, state.cost);
            memo.insert((init_robots.to_vec(), goal.to_vec()), result.clone());

            return result;
        }

        if seen.insert((state.robots.clone(), state.output.clone())) {
            let mut last_robot = state.robots[state.robots.len() - 1].clone();
            let next_inputs = last_robot.adjacents();

            for (input, new_state) in next_inputs.into_iter().chain(once(('A', last_robot.state))) {
                let mut new_robots = state.robots.to_vec().clone();

                let (new_robots, move_cost) = if new_robots.len() == 1 {
                    // Do nothing if current state is A
                    new_robots[0].move_once(&input);
                    (new_robots, 1) // shortest_path(&robots[0].panel, &robots[0].state, &input.0).len()
                } else {
                    let mut robots_before = new_robots.clone();
                    robots_before.truncate(new_robots.len() - 1);
                    let mut robot_and_cost = robots_solve_p2(&robots_before, &[input], memo);

                    last_robot.state = new_state;
                    robot_and_cost.0.push(last_robot.clone());
                    (robot_and_cost.0, robot_and_cost.1)
                };

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

                heap.push(new_state);
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
