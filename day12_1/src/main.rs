use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main(){
    let file = BufReader::new(File::open("input.dat").unwrap());
    let map: Vec<Vec<char>> = file.lines()
        .map(|l| l.unwrap().chars()
            .collect())
        .collect();
    let mut closed: Vec<(usize, usize)> = Vec::new();
    let mut cost = 0;
    for (posx, row) in map.iter().enumerate() {
        for (posy, _) in row.iter().enumerate() {
            if !closed.contains(&(posx, posy)) {
                let reg = region((posx, posy), &map);
                let mut a = 0;
                let mut c = 0;
                for (node, nbs) in reg {
                    c += 4-nbs as u64;
                    a += 1;
                    closed.push(node);
                }
                cost += a*c;
            }
        }
    }
    println!("Part 1: {}", cost);
}

fn region(pos:(usize, usize), map: &Vec<Vec<char>>) -> HashMap<(usize, usize), u8>{
    let mut openlist: Vec<(usize, usize)> = vec![pos];
    let mut closed: HashMap<(usize, usize), bool> = HashMap::new();
    let mut reg: HashMap<(usize, usize), u8> = HashMap::new();
    while !openlist.is_empty() {
        let current_node = openlist.remove(0);
        if *closed.entry(current_node).or_insert(false) {
            continue;
        }
        closed.insert(current_node, true);
        let nbs = expand_node(current_node, &map, &mut closed, &mut openlist);
        reg.insert(current_node, nbs.len() as u8);
    }
    return reg;
}

fn expand_node(current_node: (usize, usize), map: &Vec<Vec<char>>, closed: &mut HashMap<(usize, usize), bool>, openlist: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)>{
    let sizex = map.len();
    let sizey = map[0].len();
    let mut successors: Vec<(usize, usize)> = Vec::new();
    let veg = map[current_node.0][current_node.1];
    if current_node.0 > 0 && map [current_node.0-1][current_node.1] == veg {
        if ! *closed.entry((current_node.0-1, current_node.1)).or_insert(false) {
            openlist.push((current_node.0-1, current_node.1));
        }
        successors.push((current_node.0-1, current_node.1));
    }
    if current_node.0 < sizex-1 && map [current_node.0+1][current_node.1] == veg {
        if ! *closed.entry((current_node.0+1, current_node.1)).or_insert(false) {
            openlist.push((current_node.0+1, current_node.1));
        }
        successors.push((current_node.0+1, current_node.1));
    }
    if current_node.1 > 0 && map [current_node.0][current_node.1-1] == veg {
        if ! *closed.entry((current_node.0, current_node.1-1)).or_insert(false) {
            openlist.push((current_node.0, current_node.1-1));
        }
        successors.push((current_node.0, current_node.1-1));
    }
    if current_node.1 < sizey-1 && map [current_node.0][current_node.1+1] == veg {
        if ! *closed.entry((current_node.0, current_node.1+1)).or_insert(false) {
            openlist.push((current_node.0, current_node.1+1));
        }
        successors.push((current_node.0, current_node.1+1));
    }
    return successors;
}