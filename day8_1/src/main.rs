use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let f1 = BufReader::new(File::open("input.dat").unwrap());
    let grid: Vec<Vec<char>> = f1.lines()
        .map(|l| l.unwrap().chars()
            .collect())
        .collect();
    
    let mut antinode_grid: Vec<Vec<bool>> = grid.iter()
        .map(|row| vec![false; row.len()])
        .collect();
    
    let sizex: i32 = grid.len().try_into().unwrap();
    let sizey: i32 = grid[0].len().try_into().unwrap();

    let antenna_types = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    for antenna_type in antenna_types.chars() {
        let mut antennas: Vec<(i32,i32)> = Vec::new();
        for (posx, row) in grid.iter().enumerate() {
            for (posy, &c) in row.iter().enumerate() {
                if c == antenna_type {
                    antennas.push((posx.try_into().unwrap(), posy.try_into().unwrap()));
                }
            }
        }
        for comb in antennas.into_iter().combinations(2) {
            let distx = comb[1].0 - comb[0].0;
            let disty = comb[1].1 - comb[0].1;
            if comb[1].0+distx >= 0 && comb[1].0+distx < sizex {
                if comb[1].1+disty >= 0 && comb[1].1+disty < sizey {
                    let posx: usize = (comb[1].0+distx).try_into().unwrap();
                    let posy: usize = (comb[1].1+disty).try_into().unwrap();
                    antinode_grid[posx][posy] = true;
                }
            }
            if comb[0].0-distx >= 0 && comb[0].0-distx < sizex {
                if comb[0].1-disty >= 0 && comb[0].1-disty < sizey {
                    let posx: usize = (comb[0].0-distx).try_into().unwrap();
                    let posy: usize = (comb[0].1-disty).try_into().unwrap();
                    antinode_grid[posx][posy] = true;
                }
            }
        }
    }

    let mut antinodes = 0;
    for row in antinode_grid.iter() {
        for &n in row.iter() {
            if n {
                antinodes+=1;
            }
        }
    }
    println!("{}", antinodes);
}