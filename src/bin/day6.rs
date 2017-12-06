use std::io::*;
use std::collections::HashMap;

fn load_data() -> Vec<i32> {
    let mut input = String::new();
    let mut out : Vec<i32> = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.split_whitespace() {
        out.push(l.parse().unwrap());
    }
    out
}

fn mutate(v : &Vec<i32>) -> Vec<i32> {
    let mut out = v.to_vec();
    let mut max_pos = 0;
    let mut max = 0;
    let len = v.len();
    for i in 0..len {
        if v[i] > max {
            max_pos = i;
            max = v[i];
        }
    }
    out[max_pos] = 0;
    for p in 1..max+1 {
        let i = p as usize + max_pos;
        out[i % len] += 1;
    }
    out
}

fn run(v : &Vec<i32>) -> (i32, i32) {
    let mut configs = HashMap::new();
    let mut counter = 0;
    configs.insert(v.clone(), 0);
    let mut last = v.clone();
    loop {
        let w = mutate(&last);
        counter += 1;
        if configs.contains_key(&w) {
            return (counter, counter - configs[&w]);
        }
        last = w.clone();
        configs.insert(w, counter);
    }
}

fn main() {
    let v = load_data();
    let (rounds, cycle) = run(&v);
    println!("{} {}", rounds, cycle);
}