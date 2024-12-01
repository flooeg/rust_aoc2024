use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("input.dat") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let sv1:Vec<&str> = line.split_whitespace().collect();
            left.push(sv1[0].parse().unwrap());
            right.push(sv1[1].parse().unwrap());
        }
    }
    left.sort();
    right.sort();

    let distance: Vec<i32> = left
        .iter()
        .zip(right.clone())
        .map(|(l, r)|(l-r).abs())
        .collect();

    let sum :i32 = distance.iter().sum();
    println!("Part 1: {}", sum);

    let mut sim_score: i32 = 0;
    for l in left {
        let count:i32 = right.iter().filter(|&x| *x==l).count().try_into().unwrap();
        sim_score += l*count;
    }
    println!("Part 2: {}", sim_score);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}