#![feature(iter_array_chunks)]
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{once, repeat, repeat_n};
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
    op: Op,
    init_state: Option<bool>,
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
                    init_state: None,
                });
            }
            None
        })
        .collect_vec();

    let mut state = HashMap::<String, Gate>::new();
    for init_gate in &init_gates {
        state.insert(init_gate.name.clone(), init_gate.clone());
    }
    for gate in &gates {
        state.insert(gate.name.clone(), gate.clone());
    }

    // swaps
    // I found them by looking at the graphs and finding the errors
    state = swap_gates(state, "z05", "bpf");
    state = swap_gates(state, "z11", "hcc");
    state = swap_gates(state, "hqc", "qcw");
    state = swap_gates(state, "fdw", "z35");

    println!(
        "p2: {}",
        ["z05", "bpf", "z11", "hcc", "hqc", "qcw", "fdw", "z35"]
            .iter()
            .sorted()
            .join(",")
    );

    for i in 0..46 {
        let state = inject(&state, 2u64.pow(i as u32), 2u64.pow(i as u32));
        print_state(&state);
    }

    let mut final_gates = state.keys().filter(|k| k.starts_with("z")).collect_vec();
    final_gates.sort();

    Ok(())
}

fn inject(state: &HashMap<String, Gate>, x: u64, y: u64) -> HashMap<String, Gate> {
    let mut result = state.clone();
    let mut x_gates = state.keys().filter(|k| k.starts_with("x")).collect_vec();
    x_gates.sort();
    // x_gates.reverse();

    let x_vec = to_bool_vec(x, &x_gates.len());
    for (idx, gate) in x_gates.iter().enumerate() {
        let g = result.get_mut(*gate).unwrap();
        g.init_state = Some(x_vec[idx]);
    }

    let mut y_gates = state.keys().filter(|k| k.starts_with("y")).collect_vec();
    y_gates.sort();
    // y_gates.reverse();

    let y_vec = to_bool_vec(y, &y_gates.len());
    for (idx, gate) in y_gates.iter().enumerate() {
        let g = result.get_mut(*gate).unwrap();
        g.init_state = Some(y_vec[idx]);
    }

    result
}
fn print_state(state: &HashMap<String, Gate>) {
    let mut x_bit = vec![];
    let mut y_bit = vec![];

    for (name, gate) in state.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
        if name.starts_with("x") {
            x_bit.push(gate.init_state.unwrap())
        } else if name.starts_with("y") {
            y_bit.push(gate.init_state.unwrap())
        }
    }

    let x = x_bit.iter().enumerate().fold(0, |acc, (idx, b)| {
        if *b {
            return acc + 2u64.pow(idx as u32);
        }
        acc
    });
    let y = y_bit.iter().enumerate().fold(0, |acc, (idx, b)| {
        if *b {
            return acc + 2u64.pow(idx as u32);
        }
        acc
    });

    let result_vec = to_bool_vec(x + y, &x_bit.len());
    let computed_vec = compute(state.clone());

    let x_repr = x_bit
        .iter()
        .map(|b| (if *b { "1" } else { "0" }).to_string())
        .join("");
    let y_repr = y_bit
        .iter()
        .map(|b| (if *b { "1" } else { "0" }).to_string())
        .join("");

    let expected = result_vec
        .iter()
        .map(|b| (if *b { "1" } else { "0" }).to_string())
        .join("");

    let computed_number = computed_vec
        .iter()
        .enumerate()
        .map(|(idx, b)| {
            if *b {
                return 2u64.pow(idx as u32);
            }
            0
        })
        .sum::<u64>();

    let computed = computed_vec
        .iter()
        .map(|b| (if *b { "1" } else { "0" }).to_string())
        .join("");

    if computed != expected {
        println!(
            "Init numbers: {x}({x_repr}) + {y}({y_repr}) == {} (==? {computed_number})\n\tExpecting: {}",
            x + y,
            expected
        );

        println!("\tComputed : {computed}");
        // println!("{}", to_mermaid(state, get_critical_path(state, "z36")));
        // assert!(false);
    }
}

fn backtrack_gate(state: &HashMap<String, Gate>, gate: &str, expected: bool) {
    let action_plan = get_critical_path(state, gate)
        .into_iter()
        .chain(get_critical_path(state, "z00"))
        .chain(once(state.get(gate).unwrap().clone()))
        .chain(once(state.get("z00").unwrap().clone()))
        .collect_vec();

    println!(
        "plan: {}",
        action_plan.iter().map(|g| g.name.clone()).join(",")
    );
    let before_as_str = compute(state.clone())
        .iter()
        .map(|b| (if *b { "1" } else { "0" }).to_string())
        .join("");

    action_plan
        .iter()
        .combinations(2)
        .map(|v| {
            let (g1, g2) = v.iter().collect_tuple().unwrap();
            println!("swap: {} {}", g1.name, g2.name);

            let mut new_state = state.clone();
            new_state = swap_gates(new_state, &g1.name, &g2.name);
            for (k, v) in &new_state {
                println!("{k} -> {v:?}");
            }

            let as_str = compute(new_state.clone())
                .iter()
                .map(|b| (if *b { "1" } else { "0" }).to_string())
                .join("");

            let gate = new_state.get(gate).unwrap();

            println!("gate: {gate:?}");
            println!("Computed: {as_str}\n        : {before_as_str}");
            if eval_gate(&new_state, gate) == expected {}
        })
        .for_each(|_| {});
}

fn get_critical_path(state: &HashMap<String, Gate>, gate: &str) -> Vec<Gate> {
    let gate = state.get(gate).unwrap();
    if gate.init_state.is_some() {
        return vec![gate.clone()];
    } else if let (Some(i1), Some(i2)) = &gate.input_gates {
        return get_critical_path(state, i1)
            .iter()
            .chain(get_critical_path(state, i2).iter())
            .chain(once(gate))
            .cloned()
            .collect_vec();
    }

    unreachable!()
}

fn to_bool_vec(mut number: u64, target_size: &usize) -> Vec<bool> {
    let mut r = vec![];

    while number != 0 {
        r.push(number % 2 != 0);
        number /= 2;
    }

    if r.len() < *target_size {
        r = r
            .iter()
            .chain(repeat(&false).take(target_size - r.len() + 1))
            .cloned()
            .collect_vec();
    }

    r
}

fn swap_gates(state: HashMap<String, Gate>, g1: &str, g2: &str) -> HashMap<String, Gate> {
    let mut result = state.clone();
    let mut g1 = state.get(g1).unwrap().clone();
    let mut g2 = state.get(g2).unwrap().clone();

    // if let (Some(i1), Some(i2)) = &g1.input_gates {
    //     if g2.name == *i1 || g2.name == *i2 {
    //         return result;
    //     }
    // }

    // if let (Some(i1), Some(i2)) = &g2.input_gates {
    //     if g2.name == *i1 || g1.name == *i2 {
    //         return result;
    //     }
    // }

    let tmp = g1.clone();
    g1.name = g2.name.clone();
    g2.name = tmp.name;

    // g1.input_gates = g2.input_gates.clone();
    // g1.init_state = g2.init_state;
    // g1.input = g2.input;
    // g1.op = g2.op;

    // g2.input_gates = tmp.input_gates.clone();
    // g2.init_state = tmp.init_state;
    // g2.op = tmp.op;
    // g2.input = tmp.input;

    result.insert(g2.name.clone(), g2.clone());
    result.insert(g1.name.clone(), g1.clone());

    result
}

fn compute(state: HashMap<String, Gate>) -> Vec<bool> {
    let mut final_gates = state.keys().filter(|k| k.starts_with("z")).collect_vec();
    final_gates.sort();

    final_gates
        .iter()
        .map(|gate| eval_gate(&state, state.get(*gate).unwrap()))
        .collect_vec()
}

fn to_mermaid(states: &HashMap<String, Gate>, rednodes: Vec<Gate>) -> String {
    let mut result = String::from("");
    let mut b_cluster = String::from("");
    let mut e_cluster = String::from("");

    let mut shapes = String::from("");

    let mut red = String::from("");
    for gate in rednodes {
        red = format!("{red} {} [style=filled, fillcolor=red];\n", gate.name)
    }

    for gate in states.values() {
        let txt = match gate.op {
            Op::Or => format!("{} [shape=circle]\n", gate.name),
            Op::And => format!("{} [shape=diamond]\n", gate.name),
            Op::Xor => format!("{} [shape=triangle]\n", gate.name),
            Op::None => format!(""),
        };
        shapes = format!("{shapes}{txt}");
    }

    for (k, gate) in states {
        if let (Some(i1), Some(i2)) = &gate.input_gates {
            if gate.name.starts_with("z") {
                e_cluster = format!("{e_cluster}\n        {k};");
            }
            result = format!("{result}\n    {i1} -> {k};\n    {i2} -> {k};");
        } else {
            b_cluster = format!("{b_cluster}\n        {k};");
        }
    }

    result = format!(
        "
digraph G {{
    {red}
    {shapes}
    {result}
}}
"
    );

    result
}

fn eval_gate(states: &HashMap<String, Gate>, gate: &Gate) -> bool {
    if gate.init_state.is_some() {
        return gate.init_state.unwrap();
    } else if let (Some(i1), Some(i2)) = &gate.input_gates {
        return match gate.op {
            Op::And => {
                eval_gate(states, states.get(i1).unwrap())
                    && eval_gate(states, states.get(i2).unwrap())
            }
            Op::Or => {
                eval_gate(states, states.get(i1).unwrap())
                    || eval_gate(states, states.get(i2).unwrap())
            }
            Op::Xor => {
                eval_gate(states, states.get(i1).unwrap())
                    ^ eval_gate(states, states.get(i2).unwrap())
            }
            Op::None => unreachable!(),
        };
    }
    unreachable!()
}
