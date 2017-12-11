use std::io::*;
use std::cmp::max;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N, NW, NE, S, SW, SE
}

use Dir::*;

fn load_data() -> Vec<Dir> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.split(',') {
        out.push(get_direction(l));
    }
    out
}

fn get_direction(s : &str) -> Dir {
    match s.trim() {
        "n" => N,
        "nw" => NW,
        "ne" => NE,
        "s" => S,
        "sw" => SW,
        "se" => SE,
        c => { println!("{}", c); N }
    }
}

fn get_move(d : &Dir) -> (i32, i32) {
    match *d {
        N => (0,2),
        NW => (-2,1),
        NE => (2,1),
        S => (0,-2),
        SW => (-2,-1),
        SE => (2,-1)
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
    let end = v.iter().map(get_move).fold((0,0,0), |a, b| {
        let x = a.0 + b.0;
        let y = a.1 + b.1;
        let d = distance(x,y);
        (x, y, max(a.2, d))
    });
    println!("{} {:?}", distance(end.0, end.1), end.2);
}