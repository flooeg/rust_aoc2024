use std::fs;
use regex::Regex;


fn main() {
    let file = fs::read_to_string("input.dat").unwrap();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let sizex = 101;
    let sizey = 103;

    for i in 0..10000 {
        let mut floor: Vec<Vec<char>> = Vec::new();
        for _ in 0..sizey {
            floor.push(vec!('.'; sizex));
        }
        for cap in re.captures_iter(&file) {
            let mut px: i32 = cap[1].parse().unwrap();
            let mut py: i32 = cap[2].parse().unwrap();
            let vx: i32 = cap[3].parse().unwrap();
            let vy: i32 = cap[4].parse().unwrap();

            px = (px+vx*i).rem_euclid(sizex.try_into().unwrap());
            py = (py+vy*i).rem_euclid(sizey.try_into().unwrap());
            let idx: usize = px.try_into().unwrap();
            let idy: usize = py.try_into().unwrap();
            floor[idy][idx] = '#';
        }
        for row in floor.iter() {
            let s: String = row.iter().collect();
            if s.contains("################") {
                println!("{}", i);
                draw(floor.clone());
                break;
            }
        }
    }
}

fn draw(floor: Vec<Vec<char>>) {
    for row in floor.iter() {
        let s: String = row.iter().collect();
        println!("{s}");
    }
}