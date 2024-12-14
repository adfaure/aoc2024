#![feature(iter_array_chunks)]

use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"p=(\d+),(\d+) v=([-]?\d+),([-]?\d+)").unwrap();

    let mut robots = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .filter_map(|line| {
            if re.is_match(&line) {
                let (_, [x, y, vx, vy]) = re.captures_iter(&line).next().unwrap().extract();
                return Some((
                    x.parse::<i64>().unwrap(),
                    y.parse::<i64>().unwrap(),
                    vx.parse::<i64>().unwrap(),
                    vy.parse::<i64>().unwrap(),
                ));
            }

            None
        })
        .collect_vec();

    let iters = 10_000;

    let dims = (101, 103);

    (0..iters).for_each(|i| {
        for robot in robots.iter_mut() {
            robot.0 += robot.2;
            robot.1 += robot.3;

            if robot.1 < 0 {
                robot.1 = robot.1 + dims.1;
            }

            if robot.0 < 0 {
                robot.0 = dims.0 + robot.0;
            }

            if robot.1 >= dims.1 {
                robot.1 = (robot.1 % dims.1);
            }

            if robot.0 >= dims.0 {
                robot.0 = (robot.0 % dims.0);
            }
        }

        let q = robots
            .iter()
            // .filter(|robot| robot.0 != dims.0 / 2 && robot.1 != dims.1 / 2)
            .fold(
                (vec![], vec![], vec![], vec![])
                    as (
                        Vec<(i64, i64, i64, i64)>,
                        Vec<(i64, i64, i64, i64)>,
                        Vec<(i64, i64, i64, i64)>,
                        Vec<(i64, i64, i64, i64)>,
                    ),
                |(mut a, mut b, mut c, mut d), robot| {
                    // right - up
                    if robot.0 > dims.0 / 2 && robot.1 > dims.1 / 2 {
                        a.push(*robot)
                    }

                    // right - down
                    if robot.0 > dims.0 / 2 && robot.1 < dims.1 / 2 {
                        b.push(*robot)
                    }

                    // left - up
                    if robot.0 < dims.0 / 2 && robot.1 > dims.1 / 2 {
                        c.push(*robot)
                    }

                    // left - down
                    if robot.0 < dims.0 / 2 && robot.1 < dims.1 / 2 {
                        d.push(*robot)
                    }

                    (a, b, c, d)
                },
            );

        let mut nb_sym = 0;
        for a in q.0 {
            for b in &q.2 {
                if a.1 ==  b.1 && (a.0 - (dims.0 / 2)).abs() == (b.0 - (dims.0 / 2)).abs() {
                    nb_sym += 1;
                }
            }
        }

        let mut nb_sym2 = 0;
        for a in q.1 {
            for b in &q.3 {
                if a.1 ==  b.1 && (a.0 - (dims.0 / 2)).abs() == (b.0 - (dims.0 / 2)).abs() {
                    nb_sym2 += 1;
                }
            }
        }

        if nb_sym >= 45 || nb_sym2 >= 45 {
            println!("iter: {} sym: {}", i, nb_sym);
            print_robots(dims.0 as usize, dims.1 as usize, &robots);
        }

    });

    let quadrants = robots
        .iter()
        // .filter(|robot| robot.0 != dims.0 / 2 && robot.1 != dims.1 / 2)
        .fold(
            (vec![], vec![], vec![], vec![])
                as (
                    Vec<(i64, i64, i64, i64)>,
                    Vec<(i64, i64, i64, i64)>,
                    Vec<(i64, i64, i64, i64)>,
                    Vec<(i64, i64, i64, i64)>,
                ),
            |(mut a, mut b, mut c, mut d), robot| {
                if robot.0 > dims.0 / 2 && robot.1 > dims.1 / 2 {
                    a.push(*robot)
                }

                if robot.0 > dims.0 / 2 && robot.1 < dims.1 / 2 {
                    b.push(*robot)
                }

                if robot.0 < dims.0 / 2 && robot.1 > dims.1 / 2 {
                    c.push(*robot)
                }

                if robot.0 < dims.0 / 2 && robot.1 < dims.1 / 2 {
                    d.push(*robot)
                }

                (a, b, c, d)
            },
        );

    // println!("{robots:?}");
    // println!("{} {} {} {}", quadrants.0.len(), quadrants.1.len(), quadrants.2.len(), quadrants.3.len());
    let p1 = quadrants.0.len() * quadrants.1.len() * quadrants.2.len() * quadrants.3.len();

    println!("p1: {}", p1);
    Ok(())
}

fn print_robots(sx: usize, sy: usize, robots: &[(i64, i64, i64, i64)]) {
    for y in 0..sy {
        for x in 0..sx {
            let total = robots
                .iter()
                .filter(|(rx, ry, _, _)| x as i64 == *rx && y as i64 == *ry)
                .count();

            if total == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}
