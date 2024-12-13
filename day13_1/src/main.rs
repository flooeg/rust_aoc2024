use regex::Regex;
use std::fs;
use nalgebra::{Matrix2, Vector2};

fn main() {
    let file = fs::read_to_string("input.dat").unwrap();

    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let mut tokens: f64 = 0.0;
    for cap in re.captures_iter(&file) {
        let a_r: (f64, f64) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let b_r: (f64, f64) = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
        let p_r: (f64, f64) = (cap[5].parse().unwrap(), cap[6].parse().unwrap());

        let a = Vector2::new(a_r.0, a_r.1);
        let b = Vector2::new(b_r.0, b_r.1);
        let p = Vector2::new(p_r.0, p_r.1);
        let m = Matrix2::from_columns(&[a, b]);

        //my input does not have collinear a and b
        let i = m.try_inverse().unwrap();
        let presses = i*p;
        if is_integer(presses[0]) && is_integer(presses[1]) && presses[0]<=100.0 && presses[1]<=100.0{
            let cost: f64 = presses[0]*3.0+presses[1];
            tokens += cost;
        }
    }
    println!("Part 1: {}", tokens);
}

fn is_integer(value:f64) -> bool{
    (value - value.round()).abs() < (2.0*f32::EPSILON).into()
}