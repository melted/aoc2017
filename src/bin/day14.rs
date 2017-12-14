fn one_round<T : Copy>(v : &Vec<T>, pos : i32, size : i32) -> Vec<T> {
    let mut out = v.clone();
    let end = pos + size;
    let len = v.len() as i32;
    for j in 0..size {
        out[((end-j-1)%len) as usize] = v[((pos+j)%len) as usize];
    }
    out
}

fn knot_hash(input : String) -> u32 {
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
    let mut out = 0;
    for sub in data.chunks(16) {
        let res = sub.iter().fold(0, |a, &b| a^b);
        out += res.count_ones();
    }
    out
}

fn main() {
    let key = "ugkiagan";
    let mut count = 0;
    for i in 0..128 {
        let rowkey = format!("{}-{}", key, i);
        count += knot_hash(rowkey);
    }
    println!("{}", count);
}