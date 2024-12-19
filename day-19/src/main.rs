#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::fs::File;
use rayon::prelude::*;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut input = BufReader::new(File::open("input")?).lines();

    let binding = input.next().unwrap()?;
    let towels = binding
        .split(", ")
        .map(|t| t.chars().collect_vec())
        .collect_vec();

    let patterns = input
        .skip(1)
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();

    println!("towels: {:?}\npatterns: {:?}", towels, patterns);
    let p1 = patterns
        .iter()
        .map(|p| (p, patternize(&towels, p)))
        .inspect(|e| println!("{e:?}"))
        .filter(|(_, b)| *b)
        .count();
    println!("p1: {}", p1);

    let p2 = patterns
        .par_iter()
        .map(|p| (p, patternize_p2(&towels, p)))
        .inspect(|e| println!("{e:?}"))
        .map(|p_s| p_s.1)
        .sum::<usize>();

    println!("p2: {}", p2);


    Ok(())
}

fn patternize_p2(towels: &[Vec<char>], pattern: &[char]) -> usize {
    let max_towel_size = towels
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    patternize_rec_p2(towels, pattern, max_towel_size, 0, vec![])
}

fn patternize(towels: &[Vec<char>], pattern: &[char]) -> bool {
    // println!("patternize: {pattern:?}");
    let max_towel_size = towels
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    patternize_rec(towels, pattern, max_towel_size, 0, vec![])
}

fn patternize_rec_p2(
    towels: &[Vec<char>],
    pattern: &[char],
    max_towel_size: usize,
    pos: usize,
    acc: Vec<char>,
) -> usize {
    // println!(
    //     "New recursion: pos: {pos}, pattern len: {} acc: {acc:?}",
    //     pattern.len()
    // );

    if acc == pattern {
        return 1
    }

    if pos == pattern.len() {
        // println!("reached the end: {acc:?}");
        return 0
    }

    (1..=max_towel_size).map(|size| {
        let extract = pattern.iter().skip(pos).take(size);
        // println!("extract: {:?}", extract.clone().collect_vec());

        let matching_towel = towels.iter().filter(|towel| towel.len() == size).find(|towel| {
            // println!("towel: {:?}", towel);
            extract
                .clone()
                .zip(towel.iter())
                .take_while(|(l, r)| l == r)
                .count()
                == towel.len()
        });

        // println!("Found match: {:?}", matching_towel);

        if let Some(towel) = matching_towel {
            let mut acc_ = acc.clone();
            acc_.extend(towel);
            let res = patternize_rec_p2(towels, pattern, max_towel_size, pos + towel.len(), acc_);
            // println!("res: {}", res);
            res

        } else {
            // println!("leaving the life");
            0
        }
    }).sum()
}

fn patternize_rec(
    towels: &[Vec<char>],
    pattern: &[char],
    max_towel_size: usize,
    pos: usize,
    acc: Vec<char>,
) -> bool {
    // println!(
    //     "New recursion: pos: {pos}, pattern len: {} acc: {acc:?}",
    //     pattern.len()
    // );

    if acc == pattern {
        return true;
    }

    if pos == pattern.len() {
        // println!("reached the end: {acc:?}");
        return false;
    }

    (1..=max_towel_size).any(|size| {
        let extract = pattern.iter().skip(pos).take(size);
        // println!("extract: {:?}", extract.clone().collect_vec());

        let matching_towel = towels.iter().filter(|towel| towel.len() == size).find(|towel| {
            // println!("towel: {:?}", towel);
            extract
                .clone()
                .zip(towel.iter())
                .take_while(|(l, r)| l == r)
                .count()
                == towel.len()
        });

        // println!("Found match: {:?}", matching_towel);

        if let Some(towel) = matching_towel {
            let mut acc_ = acc.clone();
            acc_.extend(towel);
            let res = patternize_rec(towels, pattern, max_towel_size, pos + towel.len(), acc_);
            // println!("res: {}", res);
            res

        } else {
            // println!("leaving the life");
            false
        }
    })
}
