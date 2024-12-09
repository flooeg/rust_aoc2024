use std::fs;

struct Pointer {
    addr: u64,
    id: u64,
    len: u64
}

fn main() {
    let hdd: Vec<u64> = fs::read_to_string("input.dat").unwrap().chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();
    let mut files: Vec<Pointer> = Vec::new();
    let mut empty: Vec<Pointer> = Vec::new();
    let mut addr_pointer = 0;
    let mut iter = 0;
    for &entry in hdd.iter() {
        if iter%2==0 {
            files.push(Pointer {addr: addr_pointer, id: iter/2, len: entry});
            addr_pointer += entry;
            iter += 1;
        }
        else {
            empty.push(Pointer {addr: addr_pointer, id: 0, len: entry});
            addr_pointer += entry;
            iter += 1;
        }
    }
    let mut moved: Vec<Pointer> = Vec::new();
    while files.len() > 0 {
        let mut mv = files.remove(files.len()-1);
        for tar in empty.iter_mut() {
            if tar.addr > mv.addr {
                break;
            }
            else if tar.len >= mv.len {
                mv.addr = tar.addr;
                tar.len -= mv.len;
                tar.addr += mv.len;
                break;
            }
        }
        moved.push(mv);
    }
    let mut checksum = 0;
    for f in moved {
        for i in f.addr..(f.addr+f.len) {
            checksum += i*f.id;
        }
    }
    println!("Part 2: {}", checksum);

}