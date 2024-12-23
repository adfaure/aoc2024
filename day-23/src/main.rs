#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let links = BufReader::new(File::open("input")?)
        .lines()
        .take_while(|l| l.is_ok())
        .map(|l| {
            let line = l.unwrap();
            let mut splits = line.split("-");
            (
                splits.next().unwrap().to_string(),
                splits.next().unwrap().to_string(),
            )
        })
        .collect::<HashSet<(_, _)>>();

    // let mut connections : HashMap<String, String> = HashMap::new();

    // for (a, b) in links {
    //     connections.insert(a.clone(), b.clone());
    //     connections.insert(b.clone(), a.clone());
    // }

    println!("links ({}): {:?}", links.len(), links);
    let mut clusters: Vec<HashSet<String>> = vec![];

    for (a, b) in &links {
        println!("looking for {} {}", a, b);
        println!("\tclusters: {:?}", clusters);

        let mut candidates = clusters
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, cluster)| cluster.contains(a) || cluster.contains(b))
            .collect_vec();

        assert!(candidates.len() < 3);

        if candidates.len() == 2 {
            println!("Should remove c1: {:?}, c2: {:?}: {:?}", candidates[0], candidates[1], clusters);
            clusters = clusters
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| *idx != candidates[0].0 && *idx != candidates[1].0)
                .map(|(_, cluster)| cluster)
                .collect_vec();
            println!("Should remove: {:?}", clusters);

            let merged: HashSet<_> = candidates[0]
                .1
                .iter()
                .chain(candidates[1].1.iter())
                .cloned()
                .collect();

            clusters.push(merged);
        } else if candidates.len() == 1 {
            candidates[0].1.insert(a.clone());
            candidates[0].1.insert(b.clone());
            clusters[candidates[0].0] = candidates[0].1.clone();
        } else {
            clusters.push(HashSet::from([a.clone(), b.clone()]))
        }
        println!("\tclusters: {:?}", clusters);
    }

    println!("clusters: {:?}", clusters);

    let p1 = clusters
        .iter()
        .flat_map(|cluster| {
            cluster
                .clone()
                .into_iter()
                .permutations(3)
                .filter(|group| group.iter().any(|computer| computer.starts_with("t")))
                .collect_vec()
        })
        // .inspect(|e| println!("{:?}", e.into_iter().join(",")))
        .count();

    println!("p1: {}", p1);

    Ok(())
}
