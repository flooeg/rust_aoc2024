use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let f = BufReader::new(File::open("input.dat").unwrap());

    let arr: Vec<Vec<char>> = f.lines()
        .map(|l| l.unwrap().chars()
            .map(|c| c)
            .collect())
        .collect();
    let mut words :u32 = 0;
    for (posx, line) in arr.iter().enumerate() {
        for (posy, c) in line.iter().enumerate() {
            if *c == 'X' {
                words += count_xmas(posx, posy, arr.clone());
            }
        }
    }
    println!("Part 1: {}", words);
}

fn count_xmas(posx:usize, posy:usize, arr: Vec<Vec<char>>) -> u32{
    let mut count: u32 = 0;
    let mut allowed_x: Vec<i32> = Vec::new();
    let mut allowed_y: Vec<i32> = Vec::new();
    allowed_x.push(0);
    allowed_y.push(0);
    if posx >= 3 {allowed_x.push(-1)};
    if posx <= arr.len()-4 {allowed_x.push(1)};
    if posy >= 3 {allowed_y.push(-1)};
    if posy <= arr.len()-4 {allowed_y.push(1)};
    let it = allowed_x.into_iter().cartesian_product(allowed_y);
    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];

    for dir in it {
        if dir!=(0,0) {
            let mut runx: i32 = posx.try_into().unwrap();
            let mut runy: i32 = posy.try_into().unwrap();
            let mut hit: bool = true;
            for i in 1..4 {
                runx += dir.0;
                runy += dir.1;
                let tmpx: usize = runx.try_into().unwrap();
                let tmpy: usize = runy.try_into().unwrap();
                hit &= arr[tmpx][tmpy]==xmas[i];
            }
            if hit {
                count += 1;
            }
        }
    }
    return count;
}
