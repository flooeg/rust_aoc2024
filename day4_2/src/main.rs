use std::fs::File;
use std::io::{BufRead, BufReader};

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
            if *c == 'A' {
                words += count_xmas(posx, posy, arr.clone());
            }
        }
    }
    println!("Part 2: {}", words);
}

fn count_xmas(posx:usize, posy:usize, arr: Vec<Vec<char>>) -> u32{
    if posx == 0 || posx == arr.len()-1 || posy == 0 || posy == arr.len()-1 {
        return 0;
    }

    if (arr[posx-1][posy-1]=='M' && arr[posx+1][posy+1]=='S') || (arr[posx-1][posy-1]=='S' && arr[posx+1][posy+1]=='M') {
        if (arr[posx+1][posy-1]=='M' && arr[posx-1][posy+1]=='S') || (arr[posx+1][posy-1]=='S' && arr[posx-1][posy+1]=='M') {
            return 1;
        }
    }
    return 0;
}