use std::io::*;

fn load_data() -> Vec<u8> {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    let mut l = input.lines();
    let mut data = l.next().unwrap().as_bytes().to_vec();
    let mut extra =vec![17, 31, 73, 47, 23];
    data.append(&mut extra);
    data
}

fn one_round<T : Copy>(v : &Vec<T>, pos : i32, size : i32) -> Vec<T> {
    let mut out = v.clone();
    let end = pos + size;
    let len = v.len() as i32;
    for j in 0..size {
        out[((end-j-1)%len) as usize] = v[((pos+j)%len) as usize];
    }
    out
}

fn scramble(input : &Vec<i32>) -> Vec<i32> {
    let mut skip_size = 0;
    let mut pos = 0;
    let range = 0..256;
    let mut data : Vec<i32> = range.collect();
    let len = data.len() as i32;
    for i in input {
        let next = one_round(&data, pos, *i);
        data = next;
        pos += i + skip_size;
        skip_size += 1;
        pos %= len;
    }
    data
}

fn scramble2(input : &Vec<u8>) -> String {
    let mut skip_size = 0;
    let mut pos = 0;
    let mut data : Vec<u8> = vec![0; 256];
    for n in 0usize..256usize {
        data[n] = n as u8;
    }
    let len = data.len() as i32;
    for _ in 0..64 { 
        for &i in input {
            let next = one_round(&data, pos, i as i32);
            data = next;
            pos += i as i32 + skip_size;
            skip_size += 1;
            pos %= len;
        }
    }
    let mut s = String::new();
    for sub in data.chunks(16) {
        let res = sub.iter().fold(0, |a, &b| a^b);
        let hex = format!("{:x}", res);
        s.push_str(&hex);
    }
    s
} 

fn main() {
    let input1 : Vec<i32> = vec![187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216];
    let v = scramble(&input1);
    println!("{}", v[0] * v[1]);
    let input2 = load_data();
    let s = scramble2(&input2);
    println!("{}", s);
}