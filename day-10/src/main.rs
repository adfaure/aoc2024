use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|line| {
            line.chars()
                .filter_map(|e| e.to_string().parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dim = (grid[0].len() as i32, grid.len() as i32);

    let p1 = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, h)| {
                if *h == 0 {
                    return Some((x as i32, y as i32, h));
                }
                None
            })
        })
        .map(|(start_x, start_y, _)| {
            let mut bfs = VecDeque::new();
            let mut tagged = HashSet::new();

            bfs.push_back((start_x, start_y));

            while let Some(cell) = bfs.pop_front() {
                let (cur_x, cur_y): (i32, i32) = cell;
                let h = grid[cur_y as usize][cur_x as usize];
                if tagged.insert((cur_x, cur_y, h)) {
                    vec![
                        (cur_x, cur_y + 1),
                        (cur_x, cur_y - 1),
                        (cur_x - 1, cur_y),
                        (cur_x + 1, cur_y),
                    ]
                    .into_iter()
                    .filter(|(neigh_x, neigh_y)| {
                        *neigh_x >= 0 && *neigh_x < dim.0 && *neigh_y >= 0 && *neigh_y < dim.1
                    })
                    .filter(|(neigh_x, neigh_y)| {
                        let neigh_h = grid[*neigh_y as usize][*neigh_x as usize];
                        neigh_h - h == 1
                    })
                    .for_each(|pos| {
                        bfs.push_back(pos);
                    })
                }
            }

            let tagged = tagged.iter().filter(|e| e.2 == 9).collect::<Vec<_>>();

            // println!("sum: {summits:?}");
            tagged.into_iter().filter(|(_, _, h)| *h == 9).count()
        })
        .sum::<usize>();

    println!("p1: {}", p1);

    let p2 = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, h)| {
                if *h == 0 {
                    return Some((x as i32, y as i32, h));
                }
                None
            })
        })
        .map(|(start_x, start_y, _)| try_intersection(&grid, (start_x, start_y)))
        .sum::<i32>();

    println!("p2: {}", p2);

    Ok(())
}

fn try_intersection(grid: &[Vec<i32>], start_pos: (i32, i32)) -> i32 {
    let mut bfs = VecDeque::new();
    let mut tagged = HashSet::new();

    let (start_x, start_y) = start_pos;
    let dim = (grid[0].len() as i32, grid.len() as i32);

    if grid[start_y as usize][start_x as usize] == 9 {
        return 1;
    }

    bfs.push_back((start_x, start_y));

    while let Some(cell) = bfs.pop_front() {
        let (cur_x, cur_y): (i32, i32) = cell;
        let h = grid[cur_y as usize][cur_x as usize];

        if h == 9 {
            return 1;
        }

        if tagged.insert((cur_x, cur_y, h)) {
            let neighbors = vec![
                (cur_x, cur_y + 1),
                (cur_x, cur_y - 1),
                (cur_x - 1, cur_y),
                (cur_x + 1, cur_y),
            ]
            .into_iter()
            .filter(|(neigh_x, neigh_y)| {
                *neigh_x >= 0 && *neigh_x < dim.0 && *neigh_y >= 0 && *neigh_y < dim.1
            })
            .filter(|(neigh_x, neigh_y)| {
                let neigh_h = grid[*neigh_y as usize][*neigh_x as usize];
                neigh_h - h == 1
            })
            .collect::<Vec<_>>();

            if neighbors.len() == 1 {
                bfs.push_back(neighbors[0]);
            } else if neighbors.len() > 1 {
                return neighbors
                    .iter()
                    .map(|pos| try_intersection(grid, *pos))
                    .sum::<i32>() as i32;
            }
        }
    }

    0
}
