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
    let start_x:i32 = g_pos[0].try_into().unwrap();
    let start_y:i32 = g_pos[1].try_into().unwrap();
    let start_dir:usize = g_dir.clone();

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
    let mut allowed_obs = visited.into_iter().unique().collect::<Vec<Vec<usize>>>();
    //remove starting position, cannot put obstacle there
    allowed_obs.remove(0);
    
    let mut testlab: Vec<Vec<char>> = lab.clone();
    let mut loops = 0;
    for obs in allowed_obs.iter() {
        testlab[obs[0]][obs[1]] = '#';
        if check_loop(&testlab, start_x, start_y, start_dir){
            loops+=1;
        }
        testlab[obs[0]][obs[1]] = '.';
    }
    println!("Part 2: {}", loops);
}

fn check_loop(testlab: &Vec<Vec<char>>, mut posx: i32, mut posy: i32, mut g_dir: usize) -> bool{
    //cycling directions:
    let dirs = [[0,1], [1, 0], [0, -1], [-1, 0]];
    let x_size: i32 = testlab.len().try_into().unwrap();
    let y_size: i32 = testlab[0].len().try_into().unwrap();

    let mut visited: Vec<Vec<i32>> = Vec::new();
    for i in 0..x_size {
        visited.push(vec![]);
        for _ in 0..y_size{
            let i_help: usize = i.try_into().unwrap();
            visited[i_help].push(0);
        }
    }

    loop {
        posx += dirs[g_dir][0];
        posy += dirs[g_dir][1];
        if posx < 0 || posx >= x_size || posy <0 || posy >= y_size {
            return false;
        }
        else {
            let tmpx: usize = posx.try_into().unwrap();
            let tmpy: usize = posy.try_into().unwrap();
            if testlab[tmpx][tmpy]=='#' {
                posx -= dirs[g_dir][0];
                posy -= dirs[g_dir][1];
                g_dir = (g_dir+1)%4;
            }
            else{
                if visited[tmpx][tmpy]>=4{
                    return true;
                }
                visited[tmpx][tmpy] += 1;
            }
        }
    }
}