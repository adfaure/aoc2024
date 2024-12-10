use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn apply(numbers: &Vec<i64>, idx: usize, res: i64, results: &mut HashSet<i64>) {
    if numbers.len() == idx {
        results.insert(res);
        return;
    }

    apply(numbers, idx + 1, res + numbers[idx], results);
    apply(numbers, idx + 1, res * numbers[idx], results);
}

fn apply_p2(numbers: &Vec<i64>, idx: usize, res: i64, results: &mut HashSet<i64>) {
    if numbers.len() == idx {
        results.insert(res);
        return;
    }

    apply_p2(numbers, idx + 1, res + numbers[idx], results);
    apply_p2(numbers, idx + 1, res * numbers[idx], results);
    apply_p2(
        numbers,
        idx + 1,
        format!("{res}{}", numbers[idx]).parse::<i64>().unwrap(),
        results,
    );
}

fn main() -> std::io::Result<()> {
    let calibrations: i64 = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|line| {
            let (total, suit) = line.split_once(":").unwrap();
            let numbers = suit
                .split(" ")
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>();
            (total.parse::<i64>().unwrap(), numbers)
        })
        .filter(|(total, numbers)| {
            let mut leaves = HashSet::new();
            apply(numbers, 1, numbers[0], &mut leaves);

            leaves.contains(total)
        })
        .map(|(total, _)| total)
        .sum();

    println!("p1: {calibrations:?}");

    let calibrations: i64 = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|line| {
            let (total, suit) = line.split_once(":").unwrap();
            let numbers = suit
                .split(" ")
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>();
            (total.parse::<i64>().unwrap(), numbers)
        })
        .filter(|(total, numbers)| {
            let mut leaves = HashSet::new();
            apply_p2(numbers, 1, numbers[0], &mut leaves);

            leaves.contains(total)
        })
        .map(|(total, _)| total)
        .sum();

    println!("p1: {calibrations:?}");
    Ok(())
}
