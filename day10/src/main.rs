use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let file = BufReader::new(File::open("input.dat").unwrap());
    let top_map: Vec<Vec<u32>> = file.lines()
        .map(|l| l.unwrap().chars()
            .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            .collect())
        .collect();

    let mut scores_sum = 0;
    let mut ratings_sum = 0;
    for (posx, row) in top_map.iter().enumerate() {
        for (posy, &loc) in row.iter().enumerate() {
            if loc == 0 {
                let mut goals: Vec<(usize, usize)> = Vec::new();
                find_paths(posx, posy, &top_map, &mut goals);
                ratings_sum += goals.len();
                scores_sum += goals.into_iter().unique().collect::<Vec<_>>().len();
            }
        }
    }
    println!("Part 1: {}", scores_sum);
    println!("Part 2: {}", ratings_sum);
}

fn find_paths(posx: usize, posy: usize, map: &Vec<Vec<u32>>, goals: &mut Vec<(usize, usize)>){
    let size_x = map.len();
    let size_y = map[0].len();
    let val = map[posx][posy];
    if val == 9 {
        goals.push((posx, posy));
    }
    if posx>0 && map[posx-1][posy]==val+1 {
        find_paths(posx-1, posy, &map, goals);
    }
    if posx<size_x-1 && map[posx+1][posy]==val+1 {
        find_paths(posx+1, posy, &map, goals);
    }
    if posy>0 && map[posx][posy-1]==val+1 {
        find_paths(posx, posy-1, &map, goals);
    }
    if posy<size_y-1 && map[posx][posy+1]==val+1 {
        find_paths(posx, posy+1, &map, goals);
    }
}