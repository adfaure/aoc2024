use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> std::io::Result<()> {
    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|line| {
            line.as_ref()
                .unwrap()
                .split_whitespace()
                .filter_map(|value| value.to_string().parse::<i32>().ok())
                .try_fold((true, None, None), |acc, cur| {
                    match acc {
                        (_, None, None) => return Some((true, Some(cur), None)),
                        (_, Some(prev), None) => {
                            if cur > prev && (cur - prev).abs() <= 3 {
                                return Some((true, Some(cur), Some(true)));
                            }

                            if cur < prev && (cur - prev).abs() <= 3 {
                                return Some((true, Some(cur), Some(false)));
                            }
                        }
                        (_, Some(prev), Some(asc)) => {
                            if cur > prev && (cur - prev).abs() <= 3 && asc {
                                return Some((true, Some(cur), Some(asc)));
                            }

                            if cur < prev && (cur - prev).abs() <= 3 && !asc {
                                return Some((true, Some(cur), Some(asc)));
                            }
                        }
                        _ => unreachable!(),
                    };
                    None
                })
        })
        .count();

    println!("p1: {res}");

    let res = BufReader::new(File::open("input")?)
        .lines()
        .filter(|line| {
            let report_size = line.as_ref().unwrap().split_whitespace().count();
            (0..(report_size + 1)).rev().any(|i| {
                if line
                    .as_ref()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|value| value.to_string().parse::<i32>().ok())
                    .enumerate()
                    .filter_map(|(idx, e)| if idx == i { None } else { Some(e) })
                    .try_fold((true, None, None), |acc, cur| {
                        match acc {
                            (_, None, None) => return Some((true, Some(cur), None)),
                            (_, Some(prev), None) => {
                                if cur > prev && (cur - prev).abs() <= 3 {
                                    return Some((true, Some(cur), Some(true)));
                                }

                                if cur < prev && (cur - prev).abs() <= 3 {
                                    return Some((true, Some(cur), Some(false)));
                                }
                            }
                            (_, Some(prev), Some(asc)) => {
                                if cur > prev && (cur - prev).abs() <= 3 && asc {
                                    return Some((true, Some(cur), Some(asc)));
                                }

                                if cur < prev && (cur - prev).abs() <= 3 && !asc {
                                    return Some((true, Some(cur), Some(asc)));
                                }
                            }
                            _ => unreachable!(),
                        };
                        None
                    })
                    .is_some()
                {
                    return true;
                }
                false
            })
        })
        .count();

    println!("p2: {res}");

    Ok(())
}
