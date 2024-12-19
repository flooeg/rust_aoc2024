use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let file = read_to_string("input.dat").unwrap();
    let f = file.split("\n\n").collect::<Vec<&str>>();
    let towels = f[0].split(", ").collect::<Vec<&str>>();
    let designs = f[1].split("\n").collect::<Vec<&str>>();
    
    let mut valid = 0;
    for design in designs {
        valid += can_split(design, &towels);
    }
    println!("Part 2: {}", valid);
}

fn can_split(design: &str, towels: &Vec<&str>) -> u64 {
    let n = design.len();
    let towels_set: HashSet<&str> = towels.iter().cloned().collect();
    let mut splittable = vec![0; n + 1];
    splittable[0] = 1;

    for i in 1..=n {
        for j in 0..i {
            if splittable[j]>0 && towels_set.contains(&design[j..i]) {
                splittable[i] += splittable[j];
            }
        }
    }
    return splittable[n];
}