use std::collections::HashSet;

fn one_round<T : Copy>(v : &Vec<T>, pos : i32, size : i32) -> Vec<T> {
    let mut out = v.clone();
    let end = pos + size;
    let len = v.len() as i32;
    for j in 0..size {
        out[((end-j-1)%len) as usize] = v[((pos+j)%len) as usize];
    }
    out
}

fn knot_hash(input : &str) ->Vec<u8> {
    let mut skip_size = 0;
    let mut pos = 0;
    let mut data : Vec<u8> = vec![0; 256];
    for n in 0usize..256usize {
        data[n] = n as u8;
    }
    let mut ivec : Vec<u8>= input.bytes().collect();
    let mut extra =vec![17, 31, 73, 47, 23];
    ivec.append(&mut extra);
    let len = data.len() as i32;
    for _ in 0..64 { 
        for &i in ivec.iter() {
            let next = one_round(&data, pos, i as i32);
            data = next;
            pos += i as i32 + skip_size;
            skip_size += 1;
            pos %= len;
        }
    }
    let mut out = Vec::new();
    for sub in data.chunks(16) {
        let res = sub.iter().fold(0, |a, &b| a^b);
        out.push(res);
    }
    out
}

fn get_map(key : &str) -> HashSet<(i32, i32)> {
    let mut out = HashSet::new();
    for y in 0..128 {
        let rowkey = format!("{}-{}", key, y);
        let v = knot_hash(&rowkey);
        for i in 0..16 {
            let b = v[i as usize];
            for j in 0..8 {
                if b & (1 << j) != 0 {
                    let x = (7 - j) + 8*i;
                    out.insert((x, y));
                } 
            }
        }
    }
    out
}

fn neighbours(p : &(i32, i32)) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    out.push((p.0 - 1, p.1));
    out.push((p.0, p.1 - 1));
    out.push((p.0 + 1, p.1));
    out.push((p.0, p.1 + 1));
    out
}


fn count_regions(data : &HashSet<(i32, i32)>) -> i32 {
    let mut unexplored = data.clone();
    let mut count = 0;
    while !unexplored.is_empty() {
        let n = *unexplored.iter().next().unwrap();
        let mut frontier = HashSet::new();
        frontier.insert(n);
        while !frontier.is_empty() {
            let mut next = HashSet::new();
            for q in frontier {
                for p in neighbours(&q) {
                    if unexplored.contains(&p) {
                        next.insert(p);
                    }
                } 
                unexplored.remove(&q);
            }
            frontier = next;
        }
        count += 1;
    }
    count
}

fn main() {
    let testkey = "flqrgnkx";
    let key = "ugkiagan";
    let h = get_map(key);
    println!("{}", h.len());
    let c = count_regions(&h);
    println!("{}", c)
}