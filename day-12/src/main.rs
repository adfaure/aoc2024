use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dim = (grid[0].len() as i32, grid.len() as i32);

    let mut tagged = HashSet::new();

    let areas = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| (x as i32, y as i32, *c))
                .collect::<Vec<_>>()
        })
        .filter_map(|(cell_x, cell_y, cell_c)| {
            let mut area = HashSet::new();
            let mut fifo = VecDeque::new();
            fifo.push_back((cell_x, cell_y));

            if tagged.contains(&(cell_x, cell_y)) {
                return None;
            }

            area.insert((cell_x, cell_y, cell_c));

            while let Some((x, y)) = fifo.pop_front() {
                if !tagged.insert((x, y)) {
                    continue;
                }

                fifo.extend(
                    [(0_i32, 1_i32), (1, 0), (-1, 0), (0, -1)]
                        .iter()
                        .map(|(vx, vy)| (vx + x, vy + y))
                        .filter(|(nx, ny)| *nx >= 0 && *nx < dim.0 && *ny >= 0 && *ny < dim.1)
                        .map(|(nx, ny)| (nx, ny, grid[ny as usize][nx as usize]))
                        .filter(|(_, _, nc)| cell_c == *nc)
                        .filter(|(nx, ny, c)| area.insert((*nx, *ny, *c)))
                        .map(|(nx, ny, _)| (nx, ny)),
                );
            }

            Some((area, cell_c))
        })
        .map(|(area, cell)| {
            let fences = area
                .iter()
                .flat_map(|(cell_x, cell_y, cell_c)| {
                    [(0_i32, 1_i32), (1, 0), (-1, 0), (0, -1)]
                        .into_iter()
                        .map(|(vx, vy)| ((vx + *cell_x, vy + *cell_y), (vx, vy)))
                        .filter(|((nx, ny), _)| {
                            if !(*nx >= 0 && *nx < dim.0 && *ny >= 0 && *ny < dim.1) {
                                true
                            } else {
                                let c = grid[*ny as usize][*nx as usize];
                                c != *cell_c
                            }
                        })
                        .map(move |(_, (vx, vy))| ((cell_x, cell_y), (vx, vy)))
                })
                .collect::<Vec<_>>();

            // println!("borders: {:?}", fences);

            let mut total = 0;
            for ((x, y), (vx, vy)) in fences.clone().into_iter() {
                let nb_neigh = fences
                    .iter()
                    .filter(|((fx, fy), (fvx, fvy))| {
                        if [(0_i32, 1_i32), (1, 0), (-1, 0), (0, -1)]
                            .iter()
                            .map(|(vx, vy)| (vx + *fx, vy + *fy))
                            .any(|(nx, ny)| nx == *x && ny == *y)
                        {
                            if vx == *fvx && vy == *fvy {
                                return true;
                            }
                        }

                        false
                    })
                    .count();

                if nb_neigh == 0 {
                    total += 2;
                } else if nb_neigh == 1 {
                    total += 1;
                } else {
                    total += 0;
                }

            }

            (area, cell, total/2)
        })
        .map(|(area, cell_c, sides)| {
            (
                area.len(),
                area.iter()
                    .map(|(cell_x, cell_y, cell_c)| {
                        4 - [(0_i32, 1_i32), (1, 0), (-1, 0), (0, -1)]
                            .iter()
                            .map(|(vx, vy)| (vx + cell_x, vy + cell_y))
                            .filter(|(nx, ny)| *nx >= 0 && *nx < dim.0 && *ny >= 0 && *ny < dim.1)
                            .map(|(nx, ny)| (nx, ny, grid[ny as usize][nx as usize]))
                            .filter(|(_, _, nc)| *cell_c == *nc)
                            .count()
                    })
                    .sum::<usize>(),
                    sides
            )
        })
        .map(|(area, perimeter, total)| (area * perimeter, area * total))
        .reduce(|acc, (p1, p2)| (acc.0 + p1, acc.1 + p2)).unwrap();

    println!("p1: {}\np2: {}", areas.0, areas.1);
    Ok(())
}
