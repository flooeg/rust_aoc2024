use std::fs;

fn main() {
    let mut stones: Vec<u64> = fs::read_to_string("input.dat").unwrap().split_whitespace().map(|c| c.parse().unwrap()).collect();
    let mut ipos = 0;
    for _ in 0..25 {
        ipos = 0;
        while ipos<stones.len() {
            ipos = transform(&mut stones, ipos);
        }
    }
    println!("Part 1: {}", stones.len());
}

fn transform(stones: &mut Vec<u64>, pos: usize) -> usize{
    let s = stones.remove(pos);

    if s == 0 {
        stones.insert(pos, 1);
        return pos+1;
    }
    let p = 1+s.ilog10();
    if p%2==0 {
        let tmp: &str = &s.to_string();
        let tmp1 = &tmp[0..tmp.len()/2];
        let tmp2 = &tmp[tmp.len()/2..tmp.len()];
        stones.insert(pos, tmp2.parse().unwrap());
        stones.insert(pos, tmp1.parse().unwrap());
        return pos+2;
    }
    else {
        stones.insert(pos, s*2024);
        return pos+1;
    }
}