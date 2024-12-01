use std::io::BufRead;
use std::collections::HashMap;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| {
            let mut iter = l.as_ref().unwrap().split_whitespace();

            let left = iter.next().unwrap().to_string().parse::<i32>().ok();
            let right = iter.next().unwrap().to_string().parse::<i32>().ok();

            (left.unwrap(), right.unwrap())
        }).unzip();


    left.sort();
    right.sort();

    let result : i32 = left.iter().zip(right.iter()).map(|(left, right)|{
        (left - right).abs()
    }).sum();

    println!("p1: {result}");

    let mut map = HashMap::new();
    for i in right {
        let value_ref: &mut i32 = map
            .entry(i)
            .or_insert(0);
        *value_ref += 1;
    }


    let p2 = left.iter().map(|value| {
        let occurences = map.get(value).unwrap_or(&0);
        value * occurences
    }).sum::<i32>();

    println!("p2: {p2}");
    Ok(())
}
