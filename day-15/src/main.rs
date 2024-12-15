#![feature(iter_array_chunks)]

use itertools::EitherOrBoth;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::once;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut grid = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| {
            let line = l.ok();

            if line.is_some() && !line.clone().unwrap().is_empty() {
                return Some(line.unwrap().clone());
            }

            None
        })
        .map(|line| line.clone().chars().collect_vec())
        .collect_vec();

    let dirs = BufReader::new(File::open("input")?)
        .lines()
        .skip_while(|l| {
            let line = l.as_ref().clone().ok();

            if line.is_some() && !line.clone().unwrap().is_empty() {
                return true;
            }

            false
        })
        .skip(1)
        .flat_map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();

    // println!("dirs: {dirs:?}");

    let mut pos: (i32, i32) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == '@' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    dirs.iter().for_each(|d| {
        // println!("Grid ({d}):");
        // show_grid(&grid);

        let v: (i32, i32) = match d {
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => {
                unreachable!();
            }
        };

        let moves = check_moves(&grid, &pos, &v, vec![]);

        if !moves.is_empty() {
            pos = ((pos.0 + v.0), (pos.1 + v.1));
        }

        // println!("need to move: {:?}", moves);

        for to_move in moves.iter().rev() {
            let c = grid[to_move.1 as usize][to_move.0 as usize];
            let new_pos = ((to_move.0 + v.0) as usize, (to_move.1 + v.1) as usize);

            let swap = grid[new_pos.1][new_pos.0];
            grid[new_pos.1][new_pos.0] = c;
            grid[to_move.1 as usize][to_move.0 as usize] = swap;
        }
    });

    let p1 = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, c)| **c == 'O')
                .map(move |(x, c)| y * 100 + x)
        })
        .sum::<usize>();

    println!("p1: {p1:?}");

    let mut grid2 = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| {
            let line = l.ok();

            if line.is_some() && !line.clone().unwrap().is_empty() {
                return Some(line.unwrap().clone());
            }

            None
        })
        .map(|line| {
            line.clone()
                .chars()
                .flat_map(|c| {
                    if c == 'O' {
                        vec!['[', ']']
                    } else if c == '@' {
                        vec!['@', '.']
                    } else {
                        vec![c, c]
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut pos: (i32, i32) = grid2
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == '@' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    dirs.iter().for_each(|d| {
        println!("Grid ({d}):");
        show_grid(&grid2);

        let v: (i32, i32) = match d {
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => {
                unreachable!();
            }
        };

        let moves = check_moves_p2(&grid2, &pos, &v, vec![]);

        if !moves.is_empty() {
            pos = ((pos.0 + v.0), (pos.1 + v.1));
        }

        println!("need to move: {:?}", moves);
        for m in &moves {
            println!("need to move: {m:?} == {}", &grid2[m.1 as usize][m.0 as usize]);
        }

        let mut already_done = HashSet::new();

        for to_move in moves.iter().rev() {
            if !already_done.insert(to_move) {
                continue;
            }

            let c = grid2[to_move.1 as usize][to_move.0 as usize];
            let new_pos = ((to_move.0 + v.0) as usize, (to_move.1 + v.1) as usize);

            let swap = grid2[new_pos.1][new_pos.0];
            grid2[new_pos.1][new_pos.0] = c;
            grid2[to_move.1 as usize][to_move.0 as usize] = swap;
        }
    });

    let p2 = grid2
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(x, c)| **c == '[')
                .map(move |(x, c)| y * 100 + x)
        })
        .sum::<usize>();

    show_grid(&grid2);
    println!("p2: {p2:?}");
    Ok(())
}

fn check_moves(
    grid: &[Vec<char>],
    pos: &(i32, i32),
    v: &(i32, i32),
    mut to_move: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    // println!("pos: {pos:?} dir: {v:?} to_move: {to_move:?}");

    let new_pos = (pos.0 + v.0, pos.1 + v.1);
    let dimensions = (grid[0].len() as i32, grid.len() as i32);

    if new_pos.0 >= 0 && new_pos.0 < dimensions.0 && new_pos.1 >= 0 && new_pos.1 < dimensions.1 {
        // Mouvement ok
        let c = grid[new_pos.1 as usize][new_pos.0 as usize];
        if c == '#' {
            vec![]
        } else if c == 'O' {
            to_move.push(*pos);
            return check_moves(grid, &new_pos, v, to_move);
        } else if c == '.' {
            to_move.push(*pos);
            to_move
        } else {
            unreachable!("hit: {c}")
        }
    } else {
        unreachable!()
    }
}

fn check_moves_p2(
    grid: &[Vec<char>],
    pos: &(i32, i32),
    v: &(i32, i32),
    mut to_move: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    // println!("pos: {pos:?} dir: {v:?} to_move: {to_move:?}");

    let new_pos = (pos.0 + v.0, pos.1 + v.1);
    let dimensions = (grid[0].len() as i32, grid.len() as i32);

    if new_pos.0 >= 0 && new_pos.0 < dimensions.0 && new_pos.1 >= 0 && new_pos.1 < dimensions.1 {
        // Mouvement ok
        let c = grid[new_pos.1 as usize][new_pos.0 as usize];
        if c == '#' {
            vec![]
        } else if (c == '[' || c == ']') && v.1 != 0 {
            to_move.push(*pos);
            let next_to = if c == '[' {
                (new_pos.0 + 1, new_pos.1)
            } else {
                (new_pos.0 - 1, new_pos.1)
            };

            let cur = check_moves_p2(grid, &new_pos, v, to_move.clone());
            let next = check_moves_p2(grid, &next_to, v, to_move);

            if cur.is_empty() || next.is_empty() {
                return vec![];
            } else {
                cur.into_iter()
                    .zip_longest(next.into_iter())
                    .inspect(|e| println!("{e:?}"))
                    .flat_map(|elements| match elements {
                        EitherOrBoth::Both(l, r) => {
                            if l == r {
                                vec![r]
                            } else {
                                vec![r, l]
                            }
                        }
                        EitherOrBoth::Left(r) | EitherOrBoth::Right(r) => {
                            vec![r]
                        }
                    })
                    .collect_vec()
            }
        } else if c == '[' || c == ']' {
            to_move.push(*pos);
            return check_moves_p2(grid, &new_pos, v, to_move);
        } else if c == '.' {
            to_move.push(*pos);
            to_move
        } else {
            unreachable!("hit: {c}")
        }
    } else {
        unreachable!()
    }
}

fn show_grid(grid: &[Vec<char>]) {
    for line in grid.iter() {
        for x in line {
            print!("{}", x);
        }
        println!();
    }
}
