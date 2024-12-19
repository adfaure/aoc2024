#![feature(iter_array_chunks)]
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
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

    println!("towels: {:?}\npatterns: \n{:?}", towels, patterns);

    // let p1 = patterns
    //     .iter()
    //     .map(|p| (p, patternize_p2(&towels, p)))
    //     .inspect(|e| println!("{e:?}"))
    //     .filter(|(_, b)| *b > 0)
    //     .count();
    // println!("p1: {}", p

    let max_towel_size = towels
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let patternized_pattern = towels
        .iter()
        .map(|p| {
            (
                p.clone(),
                patternize_rec_p2_no_cache(&towels, p, max_towel_size, 0, vec![]),
            )
        })
        .collect::<HashMap<_, _>>();

    println!(
        "patternized pattern: \n{}\n---------------\n",
        patternized_pattern
            .iter()
            .map(|e| format!("{e:?}"))
            .join("\n")
    );

    let p2 = patterns
        .iter()
        .map(|p| (p, patternize_p2(&towels, p)))
        // .inspect(|e| println!("{e:?}"))
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

    let patternized_pattern = towels
        .iter()
        .map(|p| {
            (
                p.clone(),
                patternize_rec_p2_no_cache(towels, p, max_towel_size, 0, vec![]),
            )
        })
        .collect::<HashMap<_, _>>();

    // println!(
    //     "patternized pattern: {}",
    //     patternized_pattern
    //         .iter()
    //         .map(|e| format!("{e:?}"))
    //         .join("\n")
    // );

    patternize_rec_p2(towels, pattern, max_towel_size, &mut HashMap::new())
}

fn patternize_rec_p2(
    towels: &[Vec<char>],
    pattern: &[char],
    max_towel_size: usize,
    memo: &mut HashMap<Vec<char>, usize>,
) -> usize {
    if let Some(res) = memo.get(pattern) {
        return *res;
    }

    if pattern.is_empty() {
        return 1;
    }

    let sum = (1..=max_towel_size)
        .rev()
        .filter_map(|size| {
            let matching_towel = towels
                .iter()
                .filter(|towel| towel.len() == size)
                .find(|towel| {
                    pattern
                        .iter()
                        .zip(towel.iter())
                        .take_while(|(l, r)| l == r)
                        .count()
                        == towel.len()
                });

            if let Some(towel) = matching_towel {
                let res = patternize_rec_p2(
                    towels,
                    &pattern.iter().skip(towel.len()).cloned().collect_vec(),
                    max_towel_size,
                    memo,
                );

                if res == 0 {
                    None
                } else {
                    Some(res)
                }
            } else {
                None
            }
        })
        .sum();

    memo.insert(pattern.clone().to_vec(), sum);
    sum
}

// fn patternize_rec_p2_no_cache(
//     towels: &[Vec<char>],
//     pattern: &[char],
//     max_towel_size: usize,
//     pos: usize,
//     acc: Vec<char>,
// ) -> usize {
//     if acc == pattern {
//         return 1;
//     }
//
//     if pos == pattern.len() {
//         return 0;
//     }
//
//     (1..=max_towel_size)
//         .rev()
//         .map(|size| {
//             let extract = pattern.iter().skip(pos).take(size);
//
//             let matching_towel = towels
//                 .iter()
//                 .filter(|towel| towel.len() == size)
//                 .find(|towel| {
//                     extract
//                         .clone()
//                         .zip(towel.iter())
//                         .take_while(|(l, r)| l == r)
//                         .count()
//                         == towel.len()
//                 });
//
//             if let Some(towel) = matching_towel {
//                 let mut acc_ = acc.clone();
//                 acc_.extend(towel);
//                 patternize_rec_p2_no_cache(towels, pattern, max_towel_size, pos + towel.len(), acc_)
//             } else {
//                 0
//             }
//         })
//         .sum()
// }
// fn patternize_rec_p2(
//     towels: &[Vec<char>],
//     pattern: &[char],
//     max_towel_size: usize,
//     towel_scores: &HashMap<Vec<char>, usize>,
//     pos: usize,
//     acc: Vec<char>,
//     towels_set: Vec<Vec<char>>,
// ) -> usize {
//     println!("{pos} {acc:?}");
//     if acc == pattern {
//         println!("towel set: {:?}", towels_set);
//         for towel in towels_set {
//             println!("towel: {} == {:?}", towel_scores.get(&towel).unwrap(), towel);
//         }
//         return 1;
//     }
//
//     if pos == pattern.len() {
//         return 0;
//     }
//
//     (1..=max_towel_size)
//         .rev()
//         .filter_map(|size| {
//             let extract = pattern.iter().skip(pos).take(size);
//
//             let matching_towel = towels
//                 .iter()
//                 .filter(|towel| towel.len() == size)
//                 .find(|towel| {
//                     extract
//                         .clone()
//                         .zip(towel.iter())
//                         .take_while(|(l, r)| l == r)
//                         .count()
//                         == towel.len()
//                 });
//
//             if let Some(towel) = matching_towel {
//                 let mut acc_ = acc.clone();
//                 let mut new_set = towels_set.clone();
//                 new_set.push(towel.clone());
//
//                 acc_.extend(towel);
//                 let res = patternize_rec_p2(
//                     towels,
//                     pattern,
//                     max_towel_size,
//                     towel_scores,
//                     pos + towel.len(),
//                     acc_,
//                     new_set
//                 );
//                 if res == 0 {
//                     None
//                 } else {
//                     Some(res)
//                 }
//             } else {
//                 None
//             }
//         })
//         .sum()
// }
//
fn patternize_rec_p2_no_cache(
    towels: &[Vec<char>],
    pattern: &[char],
    max_towel_size: usize,
    pos: usize,
    acc: Vec<char>,
) -> usize {
    if acc == pattern {
        return 1;
    }

    if pos == pattern.len() {
        return 0;
    }

    (1..=max_towel_size)
        .rev()
        .map(|size| {
            let extract = pattern.iter().skip(pos).take(size);

            let matching_towel = towels
                .iter()
                .filter(|towel| towel.len() == size)
                .find(|towel| {
                    extract
                        .clone()
                        .zip(towel.iter())
                        .take_while(|(l, r)| l == r)
                        .count()
                        == towel.len()
                });

            if let Some(towel) = matching_towel {
                let mut acc_ = acc.clone();
                acc_.extend(towel);
                patternize_rec_p2_no_cache(towels, pattern, max_towel_size, pos + towel.len(), acc_)
            } else {
                0
            }
        })
        .sum()
}
