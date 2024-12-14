use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("input.dat").unwrap();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let sizex = 101;
    let sizey = 103;

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut quad3 = 0;
    let mut quad4 = 0;

    for cap in re.captures_iter(&file) {
        let mut px: i16 = cap[1].parse().unwrap();
        let mut py: i16 = cap[2].parse().unwrap();
        let vx: i16 = cap[3].parse().unwrap();
        let vy: i16 = cap[4].parse().unwrap();

        px = (px+vx*100).rem_euclid(sizex);
        py = (py+vy*100).rem_euclid(sizey);

        if px < sizex/2 && py < sizey/2{
            quad1 += 1;
        }
        else if px < sizex/2 && py > sizey/2{
            quad2 += 1;
        }
        else if px > sizex/2 && py < sizey/2{
            quad3 += 1;
        }
        else if px > sizex/2 && py > sizey/2{
            quad4 += 1;
        }
    }
    println!("Part 1: {}", quad1*quad2*quad3*quad4);
}
