#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

#[allow(non_snake_case)]
#[derive(Eq, Hash, Debug, Clone, PartialEq)]
struct Program {
    is: usize,
    A: u64,
    B: u64,
    C: u64,
    instructions: Vec<u64>,
}

impl Iterator for Program {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.is < self.instructions.len() {
            let (instruction, operand) =
                (self.instructions[self.is], self.instructions[self.is + 1]);

            if let Some(output) = execute_instruction(self, &instruction, &operand) {
                return Some(output);
            };
        }

        None
    }
}

fn main() -> std::io::Result<()> {
    let re_registre = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
    let re_instructions = Regex::new(r"Program: (.*)").unwrap();

    let mut program = Program {
        is: 0,
        A: 0,
        B: 0,
        C: 0,
        instructions: vec![],
    };

    BufReader::new(File::open("input")?).lines().for_each(|l| {
        let line = l.ok().unwrap();

        for (_, [registre, value]) in re_registre.captures_iter(&line).map(|c| c.extract()) {
            if registre == "A" {
                program.A = value.parse::<u64>().unwrap();
            }

            if registre == "B" {
                program.B = value.parse::<u64>().unwrap();
            }

            if registre == "C" {
                program.C = value.parse::<u64>().unwrap();
            }
        }

        for (_, [values]) in re_instructions.captures_iter(&line).map(|c| c.extract()) {
            program.instructions = values
                .split(",")
                .map(|v| v.parse::<u64>().unwrap())
                .collect_vec();
        }
    });

    let out = run_program(&mut program.clone());
    println!("p1: {}", out.iter().join(","));

    let goal = program.instructions.clone();

    // let mut drange = u64::MAX;
    // let mut dic = drange / 2;

    // let mut found = false;

    // while !found {
    //     let mut p = program.clone();
    //     p.A = dic;

    //     let out = run_program(&mut p);
    //     // println!("{dic} -- {drange}");

    //     if out.len() < goal.len() {
    //         dic += drange;
    //     } else if out.len() > goal.len() {
    //         dic -= drange;
    //     } else if out[out.len() - 1] == goal[out.len() - 1] {
    //         found = true;
    //     }

    //     drange /= 2;
    // }

    // let sol = (0..100_000_000_000u64).into_par_iter().find_map_any(|i| {
    //     let base = to_base_b(i, 8);
    //     let mut p = program.clone();
    //     p.A = base;
    //     let out = run_program(&mut p.clone());

    //     println!("out {base}: {out:?}");

    //     if goal.iter().zip(p).take_while(|(a, b)| *a == b).count() == goal.len() {
    //         Some((base, i))
    //     } else {
    //         None
    //     }

    // });

    // let sol = (0..100_000_000_000u64).find_map(|i| {
    //     let base = to_base_b(i * 8, 8);
    //     let mut p = program.clone();
    //     p.A = base;

    //     if goal.iter().zip(p).take_while(|(a, b)| *a == b).count() == goal.len() {
    //         Some((base, i))
    //     } else {
    //         None
    //     }

    // });
    //

    // println!("p2 {:?}", sol);
    //
    // let mut increases = vec![];
    // let mut current_size = 1;
    // let mut i = 0;

    // while current_size <= goal.len() + 1 {
    //     let mut p = program.clone();
    //     p.A = i;

    //     let out = run_program(&mut p);
    //     if current_size < out.len() {
    //         increases.push((i, current_size));
    //         current_size += 1;
    //     }

    //     i += 1;
    // }

    // println!("increases: {increases:?}");

    let s = goal.len();

    let min_tier = 8u64.pow(s as u32 - 1);
    let max_tier = 8u64.pow(s as u32);
    let step = (max_tier - min_tier) / 8;

    println!("min: {} max {} stp: {}", min_tier, max_tier, step);

    let mut p = program.clone();
    p.A = min_tier;
    let out = run_program(&mut p);
    println!("lower: {out:?}");

    let mut p = program.clone();
    p.A = max_tier - 1;
    let out = run_program(&mut p);
    println!("upper: {out:?}");

    for i in 0..8 {
        let mut p_ = program.clone();
        let A = min_tier + (i * step);
        p_.A = A;

        let out = run_program(&mut p_.clone());
        println!("loop({A}): {out:?}");
    }

    let mut jit: HashMap<Program, Vec<u64>> = HashMap::new();
    let sol = (0..200_000u64).find_map(|i| {
        let base = i; // to_base_b(i * 8, 8);
        let mut p = program.clone();
        p.A = base;

        let mut p2 = p.clone();

        run_program_jit(&mut p2, &mut jit);

        if goal.iter().zip(p).take_while(|(a, b)| *a == b).count() == goal.len() {
            Some((base, i))
        } else {
            None
        }
    });

    println!("{sol:?}");

    let mut p = program.clone();
    let out = run_program(&mut p);
    println!("{out:?}");

    let mut p = program.clone();
    p.A = 90112 + (28672 / 8) * 7;
    let out = run_program(&mut p);
    println!("test: {out:?}");

    // recurse_find(program.clone());
    Ok(())
}


fn recurse_find_with(p: Program, acc: u64, s: usize) -> u64 {
    if s == 0 {
        return acc;
    }

    let mut t = p.clone();
    t.A = acc;

    println!("recurse with:({acc}) {:?}", run_program(&mut t));

    let goal = p.instructions.clone();

    let min_tier = 8u64.pow(s as u32 - 1);
    let max_tier = 8u64.pow(s as u32);

    let step = (max_tier - min_tier) / 8;

    for i in 0..8 {
        let mut p_ = p.clone();
        let A = acc + min_tier + (i * step);
        p_.A = A;

        let out = run_program(&mut p_.clone());
    }

    0
}

fn recurse_find(p: Program) {
    let goal = p.instructions.clone();
    let s = goal.len();
    let acc = 0;

    let res = recurse_find_with(p, acc, s);
    println!("res: {res:?}");
}

fn run_program_jit(program: &mut Program, jit: &mut HashMap<Program, (Program, Vec<u64>)>) -> Vec<u64> {
    let mut stdout = vec![];

    while program.is < program.instructions.len() {

        let (instruction, operand) = (
            program.instructions[program.is],
            program.instructions[program.is + 1],
        );

        if let Some(output) = execute_instruction(program, &instruction, &operand) {
            stdout.push(output);
        };

    }

    stdout
}

fn run_program(program: &mut Program) -> Vec<u64> {


    let mut stdout = vec![];

    while program.is < program.instructions.len() {
        let (instruction, operand) = (
            program.instructions[program.is],
            program.instructions[program.is + 1],
        );

        if let Some(output) = execute_instruction(program, &instruction, &operand) {
            stdout.push(output);
        };
    }

    stdout
}

fn dbg_program(program: &mut Program) -> Vec<u64> {
    println!("Start {program:?}");
    let mut stdout = vec![];

    while program.is < program.instructions.len() {
        let (instruction, operand) = (
            program.instructions[program.is],
            program.instructions[program.is + 1],
        );

        println!("{program:?}");

        if let Some(output) = execute_instruction(program, &instruction, &operand) {
            stdout.push(output);
        };
    }

    println!("\nout: {stdout:?}________________________________\n");
    stdout
}

fn to_base_b(mut n: u64, b: u64) -> u64 {
    let mut res = vec![];

    while n > 0 {
        res.push(n % b);
        n /= b;
    }

    res.iter()
        .enumerate()
        .rev()
        .fold(0, |acc, (i, n)| acc + (10u64.pow(i as u32) * n))
}

fn execute_instruction(program: &mut Program, instruction: &u64, operand: &u64) -> Option<u64> {
    match instruction {
        0 => {
            let res = program.A / 2u64.pow(combo(program, operand) as u32);
            program.A = res;

            program.is += 2;
            None
        }
        1 => {
            let res = program.B ^ *operand as u64;
            program.B = res;
            program.is += 2;
            None
        }
        2 => {
            let res = combo(program, operand) % 8;
            assert!(res < 8);

            program.B = res as u64;
            program.is += 2;
            None
        }
        3 => {
            if program.A != 0 {
                program.is = *operand as usize;
            } else {
                program.is += 2;
            }
            None
        }
        4 => {
            let res = program.B ^ program.C;
            program.B = res;
            program.is += 2;
            None
        }
        5 => {
            let res = combo(program, operand) % 8;
            assert!(res < 8);
            program.is += 2;
            Some(res)
        }
        6 => {
            let res = program.A / 2u64.pow(combo(program, operand) as u32);
            program.is += 2;
            program.B = res;
            None
        }
        7 => {
            let res = program.A / 2u64.pow(combo(program, operand) as u32);
            program.is += 2;
            program.C = res;
            None
        }
        _ => {
            unreachable!()
        }
    }
}

fn combo(program: &Program, value: &u64) -> u64 {
    match value {
        0..=3 => *value,
        4 => program.A,
        5 => program.B,
        6 => program.C,
        7 => process::exit(1),
        _ => unreachable!(),
    }
}
