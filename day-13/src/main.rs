#![feature(iter_array_chunks)]

use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"Button ([AB]): X\+(\d+), Y\+(\d+)").unwrap();
    let re_b = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let p1 = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|line| {
            if re.is_match(&line) {
                let (_, [b, x, y]) = re.captures_iter(&line).next().unwrap().extract();
                return Some((
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                    Some(b.to_string()),
                ));
            }

            if re_b.is_match(&line) {
                let (_, [x, y]) = re_b.captures_iter(&line).next().unwrap().extract();
                return Some((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap(), None));
            }

            None
        })
        .array_chunks()
        .filter_map(|[a, b, p]| {
            // a.0 b.0 = p.0
            // a.1 b.1 = p.1
            let det = a.0 * b.1 - b.0 * a.1;

            let sol_a = (p.0 * b.1 - p.1 * b.0) / (a.0 * b.1 - b.0 * a.1);
            let sol_b = (p.1 * a.0 - p.0 * a.1) / (a.0 * b.1 - b.0 * a.1);

            let test_x = a.0 * sol_a + b.0 * sol_b;
            let test_y = a.1 * sol_a + b.1 * sol_b;

            if test_x == p.0 && test_y == p.1 {
                return Some((sol_a, sol_b));
            }
            None
        })
        .map(|(a, b)| 3*a + b)
        .sum::<i64>();

    let error = 10_000_000_000_000;
    let p2 = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|line| {
            if re.is_match(&line) {
                let (_, [b, x, y]) = re.captures_iter(&line).next().unwrap().extract();
                return Some((
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                    Some(b.to_string()),
                ));
            }

            if re_b.is_match(&line) {
                let (_, [x, y]) = re_b.captures_iter(&line).next().unwrap().extract();
                return Some((x.parse::<i64>().unwrap() + error, y.parse::<i64>().unwrap() + error, None));
            }

            None
        })
        .array_chunks()
        .filter_map(|[a, b, p]| {
            // a.0 b.0 = p.0
            // a.1 b.1 = p.1
            let det = a.0 * b.1 - b.0 * a.1;

            let sol_a = (p.0 * b.1 - p.1 * b.0) / (a.0 * b.1 - b.0 * a.1);
            let sol_b = (p.1 * a.0 - p.0 * a.1) / (a.0 * b.1 - b.0 * a.1);

            let test_x = a.0 * sol_a + b.0 * sol_b;
            let test_y = a.1 * sol_a + b.1 * sol_b;

            if test_x == p.0 && test_y == p.1 {
                return Some((sol_a, sol_b));
            }
            None

        })
        .map(|(a, b)| 3*a + b)
        .sum::<i64>();


    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    Ok(())
}
