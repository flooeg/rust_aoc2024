use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("input.dat") {
        // Consumes the iterator, returns an (Optional) String
        let mut safety = 0;
        for line in lines.flatten() {
            let sv1:Vec<&str> = line.split_whitespace().collect();
            let mut report:Vec<i32> = Vec::new();
            for s in sv1.iter() {
                let i:i32=s.parse().unwrap();
                report.push(i);
            }
            if is_safe(report) {
                safety += 1;
            }
        }
        println!("Part 1: {}", safety);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_safe(report: Vec<i32>) -> bool{
    // Check gradually increasing
    let mut sign:i32 = -1;
    if report[0]>report[1] {
        sign = 1;
    }
    
    for (pos,n) in report.iter().enumerate(){
        if pos > 0 {
            if !(sign*(report[pos-1]-n)>=1 && sign*(report[pos-1]-n)<=3) {
                return false;
            }
        }
    }
    //println!("{:?}", report);
    return true;
}