use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn ndigit(n: &u64) -> u64 {
    (*n as f64).log10().floor() as u64 + 1
}

fn recurse(rock: &u64, r: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    // println!("{rock} {r}");
    if memo.contains_key(&(*rock, r)) {
        return *memo.get(&(*rock, r)).unwrap();
    }

    if r == 0 {
        return 1;
    }

    let res;
    if *rock == 0 {
        res = recurse(&1, r - 1, memo);
    } else if ndigit(rock) % 2 == 0 {
        let s = format!("{rock}");
        let (l, right) = s.split_at((ndigit(rock) / 2) as usize);
        res = recurse(&(l.parse::<u64>().unwrap()), r - 1, memo)
            + recurse(&(right.parse::<u64>().unwrap()), r - 1, memo);
    } else {
        res = recurse(&(*rock * 2024), r - 1, memo);
    }

    memo.insert((*rock, r), res);
    res
}

fn main() -> std::io::Result<()> {
    let rocks = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let blinks = (0..25)
        .fold(rocks.clone(), |acc, _| {
            // println!("{acc:?}");
            acc.into_iter()
                .flat_map(|rock| {
                    if rock == 0 {
                        vec![1]
                    } else if ndigit(&rock) % 2 == 0 {
                        let s = format!("{rock}");
                        let (l, r) = s.split_at((ndigit(&rock) / 2) as usize);
                        return vec![l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()];
                    } else {
                        vec![rock * 2024]
                    }
                })
                .collect()
        })
        .len();

    println!("p1: {:?}", blinks);

    let mut memo = HashMap::new();
    let iters = 75;

    let blinks = rocks
        .iter()
        .map(|rock| recurse(rock, iters, &mut memo))
        .sum::<u64>();

    println!("p2: {:?}", blinks);

    Ok(())
}
