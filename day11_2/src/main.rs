use std::fs;
use std::collections::HashMap;

fn main() {
    let mut stones: Vec<u64> = fs::read_to_string("input.dat").unwrap().split_whitespace().map(|c| c.parse().unwrap()).collect();
    let mut hmp: HashMap<u64, u64> = stones.iter().map(|x| (*x, 1)).collect();
    for _ in 0..75 {
        hmp = transform(hmp.clone());
    }
    let mut ans = 0;
    for &s in hmp.keys() {
        ans += hmp[&s];
    }
    println!("Part 2: {}", ans);
}

fn transform(mut hmp: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut hmp_new: HashMap<u64, u64> = HashMap::new();
    for &s in hmp.keys() {
        let amount = hmp[&s];
        if s == 0 {
            add_to_hmp(&mut hmp_new, 1, amount);
            continue;
        }
        let p = 1+s.ilog10();
        if p%2==0 {
            let tmp: &str = &s.to_string();
            let tmp1 = &tmp[0..tmp.len()/2];
            let tmp2 = &tmp[tmp.len()/2..tmp.len()];
            add_to_hmp(&mut hmp_new, tmp1.parse().unwrap(), amount);
            add_to_hmp(&mut hmp_new, tmp2.parse().unwrap(), amount);
        }
        else {
            add_to_hmp(&mut hmp_new, s*2024, amount);
        }
    }
    return hmp_new;
}

fn add_to_hmp(hmp: &mut HashMap<u64,u64>, key: u64, amount: u64) {
    hmp.entry(key)
        .and_modify(|existing_value| *existing_value += amount)
        .or_insert(amount);
}