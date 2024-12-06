use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let f = BufReader::new(File::open("input.dat").unwrap());
    let lab: Vec<Vec<char>> = f.lines()
        .map(|l| l.unwrap().chars()
            .collect())
        .collect();
    
    let x_size = lab.len();
    let y_size = lab[0].len();

    //cycling directions:
    let dirs = [[0,1], [1, 0], [0, -1], [-1, 0]];

    let mut obstacles: Vec<Vec<usize>> = Vec::new();
    let mut g_pos: Vec<usize> = vec![0, 0];
    let mut g_dir: usize = 0;
    for (posx, line) in lab.iter().enumerate() {
        for (posy, c) in line.iter().enumerate() {
            match *c {
                '#' => obstacles.push(vec![posx, posy]),
                '>' => {g_pos = vec![posx, posy]; g_dir = 0;},
                'v' => {g_pos = vec![posx, posy]; g_dir = 1;},
                '<' => {g_pos = vec![posx, posy]; g_dir = 2;},
                '^' => {g_pos = vec![posx, posy]; g_dir = 3;},
                _ => {}
            }
        }
    }

    let mut visited: Vec<Vec<usize>> = vec![g_pos.clone()];
    let mut out: bool = false;
    while !out {
        let mut posx: i32 = g_pos[0].try_into().unwrap();
        let mut posy: i32 = g_pos[1].try_into().unwrap();
        posx += dirs[g_dir][0];
        posy += dirs[g_dir][1];

        if posx < 0 || posx >= x_size.try_into().unwrap() || posy <0 || posy >= y_size.try_into().unwrap() {
            out = true;
        }
        else {
            let tmpx: usize = posx.try_into().unwrap();
            let tmpy: usize = posy.try_into().unwrap();
            if obstacles.iter().any(|i| *i== vec![tmpx, tmpy]) {
                g_dir = (g_dir+1)%4;
            }
            else{
                g_pos = vec![tmpx, tmpy];
                visited.push(vec![tmpx, tmpy]);
            }
        }
    }
    println!("Part 1: {}", visited.into_iter().unique().collect::<Vec<Vec<usize>>>().len());
}