#![feature(iter_array_chunks)]
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Xor,
    Or,
    And,
    None,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XOR" => Ok(Op::Xor),
            "OR" => Ok(Op::Or),
            "AND" => Ok(Op::And),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    name: String,
    input_gates: (Option<String>, Option<String>),
    input: (Option<bool>, Option<bool>),
    op: Op,
    init_state: Option<bool>,
}

impl Gate {
    pub fn eval(&self) -> Option<bool> {
        if let Some(init) = self.init_state {
            return Some(init);
        };

        if let (Some(i1), Some(i2)) = self.input {
            return match self.op {
                Op::Xor => Some(i1 ^ i2),
                Op::Or => Some(i1 || i2),
                Op::And => Some(i1 && i2),
                _ => None,
            };
        }
        None
    }
}

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"(.*): ([01])").unwrap();
    let re2 = Regex::new(r"(.*) (.*) (.*) -> (.*)").unwrap();
    let mut input = BufReader::new(File::open("input")?)
        .lines()
        .take_while(|l| l.is_ok())
        .map_while(|l| l.ok());

    let init_gates = input
        .by_ref()
        .map_while(|l| {
            if let Some(captures) = re.captures_iter(&l).next() {
                let (_, [gate, state]) = captures.extract();
                return Some(Gate {
                    name: gate.to_string(),
                    op: Op::None,
                    input_gates: (None, None),
                    input: (None, None),
                    init_state: Some(state == "1"),
                });
            }
            None
        })
        .collect_vec();

    let gates = input
        .filter_map(|l| {
            if let Some(captures) = re2.captures_iter(&l).next() {
                let (_, [g1, op, g2, g3]) = captures.extract();
                return Some(Gate {
                    name: g3.to_string(),
                    op: Op::from_str(op).unwrap(),
                    input_gates: (Some(g1.to_string()), Some(g2.to_string())),
                    input: (None, None),
                    init_state: None,
                });
            }
            None
        })
        .collect_vec();

    let mut state = HashMap::<String, Gate>::new();
    for init_gate in init_gates {
        state.insert(init_gate.name.clone(), init_gate);
    }

    for gate in gates {
        state.insert(gate.name.clone(), gate);
    }

    // Lets find terminal gates
    let mut final_gates = state.keys().filter(|k| k.starts_with("z")).collect_vec();
    final_gates.sort();

    let final_results = final_gates.iter().map(|gate| {
        let action_plan = gate_action_plan(&state, state.get(*gate).unwrap());
        println!("plan for {} => {:?}", gate, action_plan.iter().map(|g| g.name.clone()).join(","));
        let new_state = execute_action_plan(state.clone(), &action_plan);
        let res = new_state.get(*gate).unwrap().eval().unwrap();

        res
    })
    .enumerate()
    .fold(0, |acc, (idx, b)| {
        if b {
            return acc + 2u64.pow(idx as u32);
        }
        acc
    });


    println!("p1: {:?}", final_results);

    Ok(())
}

fn execute_action_plan(
    states: HashMap<String, Gate>,
    execution_plan: &Vec<Gate>,
) -> HashMap<String, Gate> {
    let mut resulting_state = states.clone();

    for gate in execution_plan {
        if let (Some(i1), Some(i2)) = &gate.input_gates {
            let (g1, g2) = (
                resulting_state.get(i1).unwrap().clone(),
                resulting_state.get(i2).unwrap().clone(),
            );
            println!("compute for {:?}:\n\t{g1:?}\n\t{g2:?}", gate.name);

            let mut_gate = resulting_state.get_mut(&gate.name).unwrap();
            mut_gate.input = (g1.eval(), g2.eval());
            println!("\tcomputed {gate:?}");

        } else if let Some(init) = &gate.init_state {
        }
    }
    resulting_state
    
}

fn gate_action_plan(
    states: &HashMap<String, Gate>,
    entry_gate: &Gate,
) -> Vec<Gate> {
    let mut fifo = VecDeque::new();

    fifo.push_back(entry_gate);

    let mut result = vec![];

    while let Some(gate) = fifo.pop_front() {
        result.push(gate.clone());

        println!("unpack gate: {gate:?}");
        if let Some(_) = gate.init_state {
            // result.reverse();
            // return result;
        } else if let (Some(i1), Some(i2)) = &gate.input_gates {

            fifo.push_back(states.get(i1).unwrap());
            fifo.push_back(states.get(i2).unwrap());
        } else {
            unreachable!()
        }
    }

    result.reverse();
    return result;
    unreachable!()
}
