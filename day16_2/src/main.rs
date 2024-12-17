use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use itertools::Itertools;

use indicatif::ProgressBar;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Node {
    pos: (usize, usize),
    dir: usize,
}

fn main(){
    let dirs: Vec<(i64, i64)> = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut cost: HashMap<Node, u64> = HashMap::new();
    let mut prev: HashMap<Node, Vec<Node>> = HashMap::new();

    let file = BufReader::new(File::open("input.dat").unwrap());
    let map: Vec<Vec<char>> = file.lines()
        .map(|l| l.unwrap().chars()
            .collect())
        .collect();
    let mut q: Vec<Node> = Vec::new();
    let mut end: Vec<Node> = Vec::new();

    for (posx, row) in map.iter().enumerate() {
        for (posy, &c) in row.iter().enumerate() {
            if c == 'E' {
                for d in 0..4 {
                    end.push(Node {pos: (posx, posy), dir:d});
                }
            }
            if c != '#' {
                for d in 0..4 {
                    let tmp = Node {pos: (posx, posy), dir:d};
                    q.push(tmp.clone());
                    cost.insert(tmp.clone(), u64::MAX);
                }
            }
            if c == 'S' {
                cost.entry(Node {pos: (posx, posy), dir: 0}).and_modify(|t| *t=0);
                prev.insert(Node {pos: (posx, posy), dir: 0}, vec![]);
            }
        }
    }
    let bar= ProgressBar::new(q.len() as u64);
    while !q.is_empty() {
        bar.inc(1);
        let min_index = q.iter().enumerate().min_by(|(_, a), (_, b)| cost[a].cmp(&cost[b]))
        .map(|(index, _)| index).unwrap(); // Get the index
        let fnode = &q.remove(min_index);
        let dir = dirs[fnode.dir];
        let straight_pos: (i64, i64)= ((fnode.pos.0 as i64)+(dir.0), (fnode.pos.1 as i64)+(dir.1));
        if straight_pos.0 >= 0 && straight_pos.0 < map.len() as i64 && straight_pos.1 >= 0 && straight_pos.1 < map[0].len() as i64 {
                let straight_cont = map[straight_pos.0 as usize][straight_pos.1 as usize];
                if straight_cont != '#' {
                    let c = cost[fnode]+1;
                    let tmp = Node {pos: (straight_pos.0 as usize, straight_pos.1 as usize), dir: fnode.dir};
                    if cost[&tmp]>c {
                        cost.entry(tmp.clone()).and_modify(|t| *t=c);
                        let p = prev.entry(tmp.clone()).or_insert(vec![]);
                        *p=vec![fnode.clone()];
                    }
                    else if cost[&tmp]==c {
                        cost.entry(tmp.clone()).and_modify(|t| *t=c);
                        let p = prev.entry(tmp.clone()).or_insert(vec![]);
                        p.push(fnode.clone());
                    }
                }
        }
        let left = Node {pos: fnode.pos, dir:(fnode.dir+1).rem_euclid(4)};
        let right = Node {pos: fnode.pos, dir:(((fnode.dir as i64)-1).rem_euclid(4))as usize};
        let c = cost[&fnode] + 1000;
        if cost[&left] > c {
            cost.entry(left.clone()).and_modify(|t| *t=c);
            let p = prev.entry(left.clone()).or_insert(vec![]);
            *p=vec![fnode.clone()];
        }
        else if cost[&left]==c {
            cost.entry(left.clone()).and_modify(|t| *t=c);
            let p = prev.entry(left.clone()).or_insert(vec![]);
            p.push(fnode.clone());
        }

        if cost[&right] > c {
            cost.entry(right.clone()).and_modify(|t| *t=c);
            let p = prev.entry(right.clone()).or_insert(vec![]);
            *p=vec![fnode.clone()];
        }
        else if cost[&right]== c {
            cost.entry(right.clone()).and_modify(|t| *t=c);
            let p = prev.entry(right.clone()).or_insert(vec![]);
            p.push(fnode.clone());
        }
    }
    let mut cheapest = u64::MAX;
    let mut e = Node {pos: (0,0), dir: 0};
    for n in &end {
        if cost[n] < cheapest {
            cheapest = cost[n];
            e = n.clone();
        }
    }
    bar.finish();

    let path = find_seats(e.clone(), &prev);
    println!("Part 2: {}", path.into_iter().unique().collect::<Vec<_>>().len());
}

fn find_seats(n: Node, prev: & HashMap<Node, Vec<Node>>) -> Vec<(usize, usize)> {
    let mut good_seats = vec![];
    for p in &prev[&n] {
        good_seats.append(&mut find_seats(p.clone(), &prev));
    }
    good_seats.push(n.pos);
    return good_seats;
}