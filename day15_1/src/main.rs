use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};

fn main() {
    let instr: Vec<char> = read_to_string("input.dat").unwrap().split("\n").collect::<String>().chars().collect();
    let file = BufReader::new(File::open("map.dat").unwrap());
    let mut map: Vec<Vec<char>> = file.lines()
        .map(|l| l.unwrap().chars()
            .collect())
        .collect();
    let mut roby: usize = 0;
    let mut robx: usize = 0;

    for (posy, row) in map.iter().enumerate() {
        for (posx, &c) in row.iter().enumerate() {
            if c == '@' {
                roby = posy;
                robx = posx;
            }
        }
    }
    map[roby][robx] = '.';

    for &i in instr.iter() {
        if i=='^' {
            (roby, robx) = rob_move(roby as i32, robx as i32, -1, 0, &mut map);
        }
        else if i=='v' {
            (roby, robx) = rob_move(roby as i32, robx as i32, 1, 0, &mut map);
        }
        else if i=='<' {
            (roby, robx) = rob_move(roby as i32, robx as i32, 0, -1, &mut map);
        }
        else if i=='>' {
            (roby, robx) = rob_move(roby as i32, robx as i32, 0, 1, &mut map);
        }
    }

    let mut gps_sum = 0;
    for (posy, row) in map.iter().enumerate() {
        for (posx, &c) in row.iter().enumerate() {
            if c=='O' {
                gps_sum += 100*posy+posx;
            }
        }
    }
    println!("Part 1: {}", gps_sum);
}

fn rob_move(roby: i32, robx: i32, dy: i32, dx: i32, map: &mut Vec<Vec<char>>) -> (usize, usize){
    let mut cx = robx;
    let mut cy = roby;
    let mut boxes: Vec<(usize, usize)> = Vec::new();
    loop {
        cx += dx;
        cy += dy;
        let curs = map[cy as usize][cx as usize];
        if curs == '#' {
            //failed
            return (roby as usize, robx as usize);
        }
        else if curs == 'O' {
            boxes.push((cy as usize, cx as usize));
        }
        else if curs == '.' {
            // start moving
            break;
        }
    }
    for &(py,px) in boxes.iter() {
        map[py][px]='.';
    }
    for &(py, px) in boxes.iter() {
        let ppy: usize = ((py as i32)+ dy) as usize;
        let ppx: usize = ((px as i32)+ dx) as usize;
        map[ppy][ppx]='O';
    }
    return ((roby+dy).try_into().unwrap(), (robx+dx).try_into().unwrap());
}