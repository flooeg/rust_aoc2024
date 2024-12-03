use regex::Regex;
use std::fs;

fn main() {
    let memory = fs::read_to_string("input.dat").unwrap();
    let chunks: Vec<&str> = memory.split("do()").collect();
    let mut sum: u32 = 0;
    for chunk in chunks.iter() {
        let c: Vec<&str> = chunk.split("don't()").collect();
        sum += multiply(c[0])
    }
    println!("Part 2: {}", sum);
}

fn multiply(chunk: &str) -> u32{
    let re = Regex::new(r"mul\((?<mul1>\d{1,3}),(?<mul2>\d{1,3})\)").unwrap();
    let mut sum: u32 = 0;
    for cap in re.captures_iter(chunk) {
        let mul1: u32 = cap["mul1"].parse().unwrap();
        let mul2: u32 = cap["mul2"].parse().unwrap();
        sum += mul1*mul2;
    }
    return sum;
}
