#![feature(iter_array_chunks)]
use itertools::Itertools;
use rayon::prelude::*;
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

    println!("links ({}): {:?}", links.len(), links);

    let mut direct_links: HashMap<String, HashSet<String>> = HashMap::new();

    for (a, b) in &links {
        direct_links
            .entry(a.clone())
            .and_modify(|vec| {
                vec.insert(b.clone());
            })
            .or_insert_with(|| HashSet::from([b.clone()]));

        direct_links
            .entry(b.clone())
            .and_modify(|vec| {
                vec.insert(a.clone());
            })
            .or_insert_with(|| HashSet::from([a.clone()]));
    }

    for (dl, links) in &direct_links {
        println!("{dl}, {links:?}")
    }

    println!();
    println!();

    let p1 = links
        .iter()
        .flat_map(|(a, b)| {
            let (a_nodes, b_nodes) = (direct_links.get(a).unwrap(), direct_links.get(b).unwrap());
            let commons = a_nodes.intersection(b_nodes);

            let mut res = vec![];
            for common in commons {
                let mut cluster = vec![a.clone(), b.clone(), common.clone()];
                cluster.sort();
                res.push(cluster);
            }

            res
        })
        .unique()
        .filter(|cluster| cluster.iter().any(|computer| computer.starts_with("t")))
        // .inspect(|e| println!("{:?}", e))
        .count();


    let p2 = direct_links
        .par_iter()
        .map(|(k, _)| k)
        .flat_map(|k| get_interco(&direct_links, k))
        // .unique()
        .max_by(|a, b| a.len().cmp(&b.len())).unwrap();

    println!("p2: {:?}", p2.into_iter().join(","));

    Ok(())
}

fn get_interco(direct_links: &HashMap<String, HashSet<String>>, computer: &String) -> Vec<Vec<String>> {
    let mut fifo = VecDeque::from([(computer.to_string(), HashSet::from([computer.clone()]))]);
    let mut seen = HashSet::new();
    let mut results = vec![];

    while let Some((node, reachable)) = fifo.pop_front() {
        let mut cut = reachable.clone().into_iter().collect_vec();
        cut.sort();

        if seen.insert((node.clone(), cut.clone())) {
            // println!("Reachable so far: {reachable:?}");
            let connected_to = direct_links
                .get(&node)
                .unwrap_or_else(|| panic!("cannot find {node}"));

            // println!("{node} => {:?}", connected_to);
            for next_node in connected_to {
                // println!("\tReachable so far: {reachable:?}");
                let can_reach = direct_links.get(next_node).unwrap();
                // println!("\t{next_node:?} => {can_reach:?}");

                if reachable.iter().all(|n| can_reach.contains(n)) {
                    let mut new_recheable = reachable.clone();
                    new_recheable.insert(next_node.to_string());

                    // println!("\t\tChecked {next_node:?} => {can_reach:?}");
                    fifo.push_back((next_node.to_string(), new_recheable));
                } else {
                    results.push(reachable.clone());
                }
            }
        }
    }

    results.into_iter().map(|s| {
        let mut as_vec = s.into_iter().collect_vec();
        as_vec.sort();
        as_vec

    }).unique().collect_vec()

}
