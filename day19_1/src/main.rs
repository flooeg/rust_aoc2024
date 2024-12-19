use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let file = read_to_string("input.dat").unwrap();
    let f = file.split("\n\n").collect::<Vec<&str>>();
    let towels = f[0].split(", ").collect::<Vec<&str>>();
    let designs = f[1].split("\n").collect::<Vec<&str>>();
    
    let mut valid = 0;
    for design in designs {
        if can_split(design, &towels) {
            valid += 1;
        }
    }
    println!("Part 1: {}", valid);
}

fn can_split(design: &str, towels: &Vec<&str>) -> bool {
    let n = design.len();
    let towels_set: HashSet<&str> = towels.iter().cloned().collect();
    let mut splittable = vec![false; n + 1];
    splittable[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if splittable[j] && towels_set.contains(&design[j..i]) {
                splittable[i] = true;
                break;
            }
        }
    }
    return splittable[n];
}