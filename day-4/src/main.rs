use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let grid: Vec<Vec<char>> = BufReader::new(File::open("input")?)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let size_x = grid[0].len() as i32;
    let size_y = grid.len() as i32;

    let xs = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, letter)| **letter == 'X')
                .map(move |(x, letter)| (x as i32, y as i32, letter))
        })
        .collect::<Vec<_>>();

    let dirs = (-1..=1)
        .flat_map(|x| {
            (-1..=1)
                .filter(move |y| !(x == 0 && *y == 0))
                .map(move |y| (x, y))
        })
        .collect::<Vec<_>>();

    let XMAS = "XMAS";

    let res: i32 = xs
        .iter()
        // .take(1)
        .map(|(xx, yx, _)| {
            dirs.iter()
                .filter(|(dir_x, dir_y)| {
                    let m = (0..XMAS.len())
                        .map(|i| ((i as i32 * *dir_x) + *xx, (i as i32 * *dir_y) + *yx))
                        .filter(move |(x, y)| *x >= 0 && *x < size_x && *y >= 0 && *y < size_y)
                        .map(|(x, y)| (x, y, grid[y as usize][x as usize]))
                        .zip(XMAS.chars())
                        .filter(|(e, x)| e.2 == *x)
                        .count();

                    m == XMAS.len()
                })
                .count() as i32
        })
        .sum();

    println!("p1: {res}");

    let A = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, letter)| **letter == 'A')
                .map(move |(x, letter)| (x as i32, y as i32, letter))
        })
        .collect::<Vec<_>>();

    let res: i32 = A
        .iter()
        .filter(|(xa, ya, _)| {
            [(-1, -1), (1, -1)]
                .iter()
                .filter(|p| (*ya + p.1) >= 0 && (*ya + p.1) < size_y)
                .filter(|p| (*xa + p.0) >= 0 && (*xa + p.0) < size_x)
                .filter(|p| (*ya - p.1) >= 0 && (*ya - p.1) < size_y)
                .filter(|p| (*xa - p.0) >= 0 && (*xa - p.0) < size_x)
                .filter(|p| {
                    let pos = ((*xa + p.0) as usize, (*ya + p.1) as usize) ;
                    let letter = grid[pos.1][pos.0];

                    let sym = (*xa - p.0, *ya - p.1);
                    let letter_sym = grid[sym.1 as usize][sym.0 as usize];

                    (letter == 'S' && letter_sym == 'M') || (letter == 'M' && letter_sym == 'S')
                })
                .count()
                == 2
        })
        .count() as i32;

    println!("p2: {res}");
    Ok(())
}
