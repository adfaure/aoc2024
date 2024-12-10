use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let files = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .next()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_string().parse::<u32>().ok())
                .enumerate()
                .collect_vec()
        })
        .unwrap();

    let p1 = files
        .iter()
        .flat_map(|(id, size)| (0..*size).map(|_| if *id % 2 == 0 { Some(*id / 2) } else { None }))
        .batching(|it| match it.next() {
            Some(Some(e)) => Some(e),
            Some(None) => {
                while let Some(r) = it.next_back() {
                    if let Some(v) = r {
                        return Some(v);
                    }
                }
                // No more file to loop
                None
            }
            None => None,
        })
        .enumerate()
        .map(|(pos, id)| id * pos)
        .sum::<usize>();

    println!("p1: {p1:?}");

    let defrag = files
        .iter()
        .rev()
        .fold(files.clone(), |mut acc, (last_id, size)| {
            if *last_id == 0 {
                return acc;
            };

            if last_id % 2 != 0 {
                return acc;
            }

            match acc.iter().position(|e| *last_id == e.0) {
                Some(fs_pos) => {
                    let (next_file, size_to_fill) = acc[fs_pos];

                    match acc.iter().enumerate().position(|(c, (fid, fsize))| {
                        fid % 2 == 1 && *fsize >= size_to_fill && c < fs_pos
                    }) {
                        Some(insert_pos) => {
                            assert!(insert_pos < fs_pos);
                            assert!(acc[insert_pos].0 % 2 == 1);

                            let tmp = acc[insert_pos];
                            let remaining = tmp.1 - size_to_fill;

                            acc[insert_pos] = (next_file, size_to_fill);
                            acc[fs_pos] = (tmp.0, size_to_fill);

                            if remaining > 0 {
                                acc.insert(insert_pos + 1, (tmp.0, remaining));
                            }

                            acc
                        }
                        None => acc,
                    }
                }
                None => acc,
            }
        });

    let p2 = defrag
        .iter()
        .flat_map(|(id, size)| (0..*size).map(move |_| id))
        .enumerate()
        .map(|(pos, id)| if id % 2 == 0 { pos * id / 2 } else { 0 })
        .sum::<usize>();

    println!("p2: {p2:?}");
    Ok(())
}

fn print_fs(fs: &Vec<(usize, u32)>) {
    fs.iter()
        .flat_map(|(id, size)| (0..*size).map(move |_| id))
        .inspect(|e| {
            if *e % 2 == 0 {
                print!("{}", *e / 2)
            } else {
                print!(".")
            }
        })
        .enumerate()
        .map(|(pos, id)| pos * id)
        .for_each(|_| {});

    println!();
}
