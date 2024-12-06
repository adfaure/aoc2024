use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|line| line != &"")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(
            |(y, line)| match line.iter().enumerate().find(|(_, col)| **col == '^') {
                Some((x, _)) => Some((x as i32, y as i32)),
                None => None,
            },
        )
        .unwrap();

    let size_x = grid[0].len() as i32;
    let size_y = grid.len() as i32;

    let mut dir = (0, -1);

    let mut parcours: HashSet<(i32, i32)> = HashSet::new();
    let mut cur_pos = Some(pos);

    while let Some((x, y)) = cur_pos {
        parcours.insert((x, y));

        let next_pos = (x + dir.0, y + dir.1);
        if next_pos.0 < 0 || next_pos.0 >= size_x || next_pos.1 < 0 || next_pos.1 >= size_y {
            cur_pos = None;
        } else {
            let cell = grid[next_pos.1 as usize][next_pos.0 as usize];
            if cell == '#' {
                dir = (-dir.1, dir.0);
            } else {
                cur_pos = Some(next_pos);
            }
        }
    }

    println!("p1: {:?}", parcours.len());


    let mut dir = (0, -1);
    let mut cur_pos = Some(pos);
    let mut parcours: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut total = 0;
    let mut added = HashSet::new();

    while let Some((x, y)) = cur_pos {
        parcours.insert((x, y, dir.0, dir.1));
        let next_pos = (x + dir.0, y + dir.1);

        if next_pos.0 < 0 || next_pos.0 >= size_x || next_pos.1 < 0 || next_pos.1 >= size_y {
            cur_pos = None;
        } else {
            let cell = grid[next_pos.1 as usize][next_pos.0 as usize];

            if cell != '#' &&  (next_pos != pos) {
                let mut add_obstacle = grid.clone();
                add_obstacle[next_pos.1 as usize][next_pos.0 as usize] = '#';

                if try_(add_obstacle, pos, (0, -1)) && !added.contains(&(next_pos.1, next_pos.0)) {
                    added.insert((next_pos.1, next_pos.0));
                    total += 1;
                }
            }

            if cell == '#' {
                dir = (-dir.1, dir.0);
            } else {
                cur_pos = Some(next_pos);
            }

        }
    }

    println!("p2: {}", total);

    Ok(())
}


fn try_(grid: Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let size_x = grid[0].len() as i32;
    let size_y = grid.len() as i32;

    let mut dir = dir;

    let mut cur_pos = Some(pos);
    let mut parcours: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    while let Some((x, y)) = cur_pos {
        if parcours.contains(&(x, y, dir.0, dir.1)) {
            return true;
        }

        parcours.insert((x, y, dir.0, dir.1));

        let next_pos = (x + dir.0, y + dir.1);

        if next_pos.0 < 0 || next_pos.0 >= size_x || next_pos.1 < 0 || next_pos.1 >= size_y {
            cur_pos = None;
        } else {
            let cell = grid[next_pos.1 as usize][next_pos.0 as usize];
            if cell == '#' {
                dir = (-dir.1, dir.0);
            } else {
                cur_pos = Some(next_pos);
            }
        }

    }

    false
}
