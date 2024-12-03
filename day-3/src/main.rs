use std::io::{BufReader, Read};
use std::fs::File;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut instructions = String::new();
    BufReader::new(File::open("input")?).read_to_string(&mut instructions);
    let re = Regex::new(r"mul\((\d+),(\d+)\)");

    let mut sum = 0;
    for (_, [a, b]) in re.clone().ok().unwrap().captures_iter(&instructions).map(|c| c.extract()) {
        let left = a.parse::<i32>().ok().unwrap();
        let right = b.parse::<i32>().ok().unwrap();

        sum += left * right;
    }
    println!("p1: {sum}");

    let do_dont = Regex::new(r"(do\(\))|(don't\(\))");
    instructions = instructions.replace("do()", "do()0");
    instructions = instructions.replace("don't()", "don't()1");
    instructions = format!("0{instructions}");

    let res: i32 = do_dont.clone().ok().unwrap().split(&instructions)
        .filter(|split| split.starts_with("0"))
        .map(|to_compute| {
            let mut sum = 0;
            for (_, [a, b]) in re.clone().ok().unwrap().captures_iter(to_compute).map(|c| c.extract()) {
                let left = a.parse::<i32>().ok().unwrap();
                let right = b.parse::<i32>().ok().unwrap();

                sum += left * right;
            }
            sum
        }).sum();


    println!("p2: {res}");
    Ok(())
}
