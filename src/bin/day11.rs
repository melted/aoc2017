use std::io::*;
use std::cmp::max;

fn load_data() -> Vec<(i32, i32)> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.split(',') {
        out.push(get_move(l));
    }
    out
}

fn get_move(s : &str) -> (i32, i32) {
    match s.trim() {
        "n" => (0,2),
        "nw" => (-2,1),
        "ne" => (2,1),
        "s" => (0,-2),
        "sw" => (-2,-1),
        "se" => (2,-1),
        _ => { println!("{}", s); (0,0)}
    }
}

fn distance(x : i32, y : i32) -> i32 {
    let (a, b) = (i32::abs(x), i32::abs(y));
    if a < b/2 { 
        (b + (a/2))/2
    } else {
        a/2
    }
}

fn main() {
    let v = load_data();
    let mut x = 0;
    let mut y = 0;
    let mut max_dist = 0;
    for &(dx, dy) in v.iter() {
        x += dx;
        y += dy;
        max_dist = max(max_dist, distance(x, y));
    }
    println!("{} {}", distance(x, y), max_dist);
}