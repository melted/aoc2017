use std::io::*;
use std::i32;
use std::cmp::{max,min};

fn load_data() -> Vec<Vec<i32>> {
    let mut input = String::new();
    let mut out : Vec<Vec<i32>> = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        let mut row : Vec<i32> = Vec::new();
        for w in l.split_whitespace() {
            row.push(w.parse().unwrap())
        }
        out.push(row)
    }
    out
}

fn maxmin(v : &Vec<i32>) -> i32 {
    let mut mx : i32 = i32::MIN;
    let mut mn : i32 = i32::MAX;
    for &i in v.into_iter() {
        mx = max(mx, i);
        mn = min(mn, i);
    }
    mx - mn
}

fn evendivide(v : &Vec<i32>) -> i32 {
    for i in 0..v.len()-1 {
        let n = &v[i];
        let (_, u) = v.split_at(i+1);
        for m in u {
            if max(m,n) % min(m,n) == 0 {
                return max(m,n)/min(m,n);
            }
        }
    }
    0
}

fn main() {
    let v = load_data();
    let answer1 : i32 = v.iter().map(maxmin).sum();
    let answer2 : i32 = v.iter().map(evendivide).sum();
    println!("{}", answer1);
    println!("{}", answer2);
}