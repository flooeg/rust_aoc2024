use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

fn main() {
    let f1 = BufReader::new(File::open("instr.dat").unwrap());
    let instr: Vec<Vec<u32>> = f1.lines()
        .map(|l| l.unwrap().split("|")
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    let f2 = BufReader::new(File::open("input.dat").unwrap());
    let updates: Vec<Vec<u32>> = f2.lines()
        .map(|l| l.unwrap().split(",")
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    for update in updates.iter() {
        let mut u = update.clone();
        u.sort_by(|a, b| custom_sort(*a, *b, instr.clone()));
        if u==*update {
            sum1 += u[u.len()/2];
        }
        else {
            sum2 += u[u.len()/2];
        }
    }
    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}

fn custom_sort(a: u32, b: u32, instr: Vec<Vec<u32>>) -> Ordering {
    if instr.contains(&vec![a,b]) {
        return Ordering::Less;
    }
    else if instr.contains(&vec![b,a]) {
        return Ordering::Greater;
    }
    else {
        return Ordering::Equal;
    }
}