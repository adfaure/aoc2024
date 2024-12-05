use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let rules = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|line| line != &"")
        .map(|l| l.to_string())
        .map(|l| {
            let mut ord = l.split("|").filter_map(|e| e.parse::<i32>().ok());

            (ord.next().unwrap(), ord.next().unwrap())
        })
        .map(|pair| (pair.1, pair.0));

    let mut map = HashMap::new();
    let mut map_r = HashMap::new();

    for rule in rules {
        let dissallow = map.entry(rule.0).or_insert(HashSet::new());
        dissallow.insert(rule.1);

        let dissallow = map_r.entry(rule.1).or_insert(HashSet::new());
        dissallow.insert(rule.0);
    }

    let updates: Vec<_> = BufReader::new(File::open("input")?)
        .lines()
        .map(|l| l.ok().unwrap())
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|l| l.to_string())
        .map(|l| {
            l.split(",")
                .filter_map(|e| e.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe: i32 = updates
        .iter()
        .filter(|update| {
            let mut set: HashSet<i32> = HashSet::new();
            update.iter().all(|n| {
                if set.contains(n) {
                    return false;
                }

                if map.contains_key(n) {
                    set.extend(map.get(n).unwrap());
                };

                true
            })
        })
        .map(|update| {
            let midle = update.len() / 2;
            update[midle]
        })
        .sum();

    println!("p1: {safe}");

    // let mut after = HashMap::new();
    // let mut before = HashMap::new();

    // for rule in rules {
    //     let dissallow = after.entry(rule.0).or_insert(HashSet::new());
    //     dissallow.insert(rule.1);

    //     let dissallow = before.entry(rule.1).or_insert(HashSet::new());
    //     dissallow.insert(rule.1);
    // }

    let fixed: i32 = updates
        .iter()
        .filter(|update| {
            let mut set: HashSet<i32> = HashSet::new();
            !update.iter().all(|n| {
                if set.contains(n) {
                    return false;
                }

                if map.contains_key(n) {
                    set.extend(map.get(n).unwrap());
                };

                true
            })
        })
        .map(|update| {
            let mut result = update.clone();

            for (p, _) in update.iter().enumerate() {
                let mut safe = false;
                while !safe {
                    let cur = &result[p];
                    let should_be_after = map.get(cur);

                    if should_be_after.is_some() {
                        let to_check = should_be_after.unwrap();
                        let conflict = result
                            .clone()
                            .into_iter()
                            .enumerate()
                            .skip(p)
                            .filter(|e| to_check.contains(&e.1))
                            .last();

                        if conflict.is_some() {
                            let (pos, val) = conflict.unwrap();
                            result[pos] = *cur;
                            result[p] = val;
                        } else {
                            safe = true;
                        }
                    } else {
                        safe = true;
                    }

                }
            }
            result
        })
        .map(|update| {
            let midle = update.len() / 2;
            update[midle]
        })
        .sum();

    println!("p2: {fixed}");

    Ok(())
}
