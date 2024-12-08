use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let dimensions = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .enumerate()
        .last()
        .map(|e| (e.1.chars().enumerate().last().unwrap().0 + 1, e.0 + 1))
        .unwrap();

    let mut towers = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (x as i32, y as i32, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    towers.sort_by(|a, b| a.2.cmp(&b.2));


    let mut signals = HashSet::new();
    for (key, chunk) in &towers.iter().chunk_by(|elt| elt.2) {
        let group = chunk.collect_vec();

        group.iter().combinations(2).for_each(|e| {
            let (xa, ya, _) = e[0];
            let (xi, yi, _) = e[1];

            let first_signal = (2 * xi - xa, 2 * yi - ya);
            let second_signal = (2 * xa - xi, 2 * ya - yi);


            if first_signal.0 < dimensions.0 as i32
                && first_signal.0 >= 0
                && first_signal.1 < dimensions.1 as i32
                && first_signal.1 >= 0
            {
                let usized = (first_signal.0 as u32, first_signal.1 as u32);
                signals.insert(usized);
            }

            if second_signal.0 < dimensions.0 as i32
                && second_signal.0 >= 0
                && second_signal.1 < dimensions.1 as i32
                && second_signal.1 >= 0
            {
                let usized = (second_signal.0 as u32, second_signal.1 as u32);
                signals.insert(usized);
            }

        });
    }
    println!("p1: {}", signals.len());

    // for i in 0..dimensions.1 {
    //     for j in 0..dimensions.0 {
    //         let tower = towers.iter().position(|e| e.1 == i as i32 && e.0 == j as i32);

    //         if tower.is_some() {
    //             print!("{}", towers[tower.unwrap()].2);
    //         }
    //         else if signals.contains(&(j as u32, i as u32)) {
    //             print!("#")
    //         } else {
    //             print!(".");
    //         }
    //     }

    //     println!();
    // }

    let mut signals = HashSet::new();
    for (key, chunk) in &towers.iter().chunk_by(|elt| elt.2) {
        let group = chunk.collect_vec();

        group.iter().combinations(2).for_each(|e| {
            let (xa, ya, _) = e[0];
            let (xi, yi, _) = e[1];

            signals.insert((*xa as u32, *ya as u32));
            signals.insert((*xi as u32, *yi as u32));

            let first_signal = (2 * xi - xa, 2 * yi - ya);
            let leap_first = (xi - xa, yi - ya);
            let second_signal = (2 * xa - xi, 2 * ya - yi);
            let leap_second = (xa - xi, ya - yi);

            let mut resonant = first_signal;
            while resonant.0 < dimensions.0 as i32
                && resonant.0 >= 0
                && resonant.1 < dimensions.1 as i32
                && resonant.1 >= 0
            {
                let usized = (resonant.0 as u32, resonant.1 as u32);
                signals.insert(usized);
                resonant = (resonant.0 + leap_first.0, resonant.1 + leap_first.1);
            }

            let mut resonant = second_signal;
            while resonant.0 < dimensions.0 as i32
                && resonant.0 >= 0
                && resonant.1 < dimensions.1 as i32
                && resonant.1 >= 0
            {
                let usized = (resonant.0 as u32, resonant.1 as u32);
                signals.insert(usized);
                resonant = (resonant.0 + leap_second.0, resonant.1 + leap_second.1);
            }

        });
    }

    // for i in 0..dimensions.1 {
    //     for j in 0..dimensions.0 {
    //         let tower = towers.iter().position(|e| e.1 == i as i32 && e.0 == j as i32);

    //         if tower.is_some() {
    //             print!("{}", towers[tower.unwrap()].2);
    //         }
    //         else if signals.contains(&(j as u32, i as u32)) {
    //             print!("#")
    //         } else {
    //             print!(".");
    //         }
    //     }

    //     println!();
    // }

    println!("p2: {}", signals.len());

    Ok(())
}
