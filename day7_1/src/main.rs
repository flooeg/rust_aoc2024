use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = BufReader::new(File::open("input.dat").unwrap());
    let re = Regex::new(r"(\d+)").unwrap();
    let mut calib = 0;
    for line in file.lines() {
        let l = line.unwrap();
        let mut nums: Vec<i64> = Vec::new();
        for cap in re.captures_iter(l.as_str()){
            let n: i64 = cap[0].parse().unwrap();
            nums.push(n);
        }
        let res = nums.remove(0);
        if evaluate(res, nums) {
            calib += res;
        }
    }
    println!("Part 1: {}", calib);
}

fn evaluate(res: i64, mut n_vec:Vec<i64>) -> bool{
    let n = n_vec.remove(n_vec.len()-1);
    if n_vec.len() == 0 {
        if res==n {
            return true;
        }
        else {
            return false;
        }
    }

    if res%n == 0 && evaluate(res/n, n_vec.clone()){
        return true;
    }
    if res-n > 0 && evaluate(res-n, n_vec.clone()) {
        return true;
    }
    return false;
}
