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
    while empty.len() > 0 {
        let mut mv = files.remove(files.len()-1);
        let mut tar = empty.remove(0);
        if tar.addr > mv.addr {
            files.push(mv);
            break;
        }

        if mv.len < tar.len {
            mv.addr = tar.addr;
            tar.addr += mv.len;
            tar.len -= mv.len;
            files.insert(0, mv);
            empty.insert(0, tar);
        }
        else if mv.len == tar.len {
            mv.addr = tar.addr;
            files.insert(0, mv);
        }
        else {
            let mv1 = Pointer {addr: tar.addr, len: tar.len, id: mv.id};
            mv.len -= tar.len;
            files.push(mv);
            files.insert(0, mv1);
        }
    }
    let mut checksum = 0;
    for f in files {
        for i in f.addr..(f.addr+f.len) {
            checksum += i*f.id;
        }
    }
    println!("Part 1: {}", checksum);

}