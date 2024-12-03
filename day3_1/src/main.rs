use regex::Regex;
use std::fs;

fn main() {
    let memory = fs::read_to_string("input.dat").unwrap();
    let re = Regex::new(r"mul\((?<mul1>\d{1,3}),(?<mul2>\d{1,3})\)").unwrap();
    let mut sum: u32 = 0;
    for cap in re.captures_iter(memory.as_str()) {
        let mul1: u32 = cap["mul1"].parse().unwrap();
        let mul2: u32 = cap["mul2"].parse().unwrap();
        sum += mul1*mul2;
    }
    println!("Part 1: {}", sum);
}
