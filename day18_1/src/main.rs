use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

static SIZE: usize = 71;
static BYTES: u16 = 1024;

fn main() {
    let file = read_to_string("input.dat").unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..SIZE {
        map.push(vec!{'.'; SIZE});
    }
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut it = 0;
    for c in re.captures_iter(&file) {
        let posx: usize = c[1].parse().unwrap();
        let posy: usize = c[2].parse().unwrap();
        map[posx][posy] = '#';
        it += 1;
        if it==BYTES {
            break;
        }
    }
    let start: (usize, usize) = (0,0);
    let end: (usize, usize) = (SIZE-1, SIZE-1);
    let mut openlist: Vec<(usize, usize)> = vec![start];
    let mut closedlist: Vec<(usize, usize)> = vec![];
    let mut f: HashMap<(usize, usize), u64> = HashMap::new();
    let mut g: HashMap<(usize, usize), u64> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    f.insert(start, 0);
    g.insert(start, 0);
    
    while !openlist.is_empty() {
        let min_index = openlist.iter().enumerate().min_by(|(_, a), (_, b)| f[a].cmp(&f[b]))
        .map(|(index, _)| index).unwrap(); // Get the index
        let current_node = openlist.remove(min_index);
        if current_node == end {
            // path found
            break;
        }
        closedlist.push(current_node);
        expand_node(current_node, &mut openlist, closedlist.clone(), &mut f, &mut g, &map, &mut prev);
    }

    println!("Part 1: {}", f[&end]-1);
}

fn expand_node(current_node: (usize, usize), openlist: &mut Vec<(usize, usize)>, closedlist: Vec<(usize, usize)>, f: &mut HashMap<(usize, usize), u64>, g: &mut HashMap<(usize, usize), u64>, map: &Vec<Vec<char>>, prev: &mut HashMap<(usize, usize), (usize, usize)>) {
    let mut successors: Vec<(usize, usize)> = Vec::new();
    if current_node.0 > 0 && map [current_node.0-1][current_node.1] != '#' {
        successors.push((current_node.0-1, current_node.1));
    }
    if current_node.0 < SIZE-1 && map [current_node.0+1][current_node.1] != '#' {
        successors.push((current_node.0+1, current_node.1));
    }
    if current_node.1 > 0 && map [current_node.0][current_node.1-1] != '#'{
        successors.push((current_node.0, current_node.1-1));
    }
    if current_node.1 < SIZE-1 && map [current_node.0][current_node.1+1] != '#'{
        successors.push((current_node.0, current_node.1+1));
    }

    for s in successors {
        if closedlist.contains(&s) {
            continue;
        }
        let tent_g = g[&current_node]+1;
        if openlist.contains(&s) && g[&s] <= tent_g {
            continue;
        }
        prev.insert(s, current_node);
        g.insert(s, tent_g);
        f.insert(s, tent_g+h(s));
        if !openlist.contains(&s) {
            openlist.push(s);
        }
    }
}

fn h(pos: (usize, usize)) -> u64 {
    let b1 = SIZE-pos.0-1;
    let b2 = SIZE-pos.1-1;
    return f64::sqrt(b1.pow(2) as f64+b2.pow(2) as f64) as u64 +1;
}