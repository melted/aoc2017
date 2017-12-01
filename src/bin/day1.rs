use std::convert::From;
use std::io::*;

fn load_data() -> Vec<i32> {
    let mut v : Vec<u8> = Vec::new();
    stdin().read_to_end(&mut v);
    let mut out : Vec<i32> = v.into_iter().map(|x| i32::from(x) - 48).filter(|&x| x >= 0 && x <= 9).collect();
    let first = out[0];
    out.push(first);
    out
}

fn main() {
    let v = load_data();
    let answer1 : i32 = v.windows(2).filter(|&w| w[0] == w[1]).map(|w| w[0]).sum();
    let (x,z) = v.as_slice().split_at(v.len()/2);
    let answer2 : i32 = x.iter().zip(z.iter()).filter( |&(a,b)| a == b).map(|(a, b)| a+b).sum();
    println!("{}", answer1);
    println!("{}", answer2);
}