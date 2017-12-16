#![feature(slice_rotate)]
use std::io::*;

#[derive(Clone, Copy, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8)
}

use Move::*;

fn get_move(s : &str) -> Move {
    let (first, rest) = s.split_at(1);
    match first {
        "s" => Spin(rest.parse().unwrap()),
        "p" => {
            let mut iter = rest.bytes();
            let a = iter.next().unwrap();
            let _ = iter.next();
            let b = iter.next().unwrap();
            Partner(a, b)
        },
        "x" => {
            let mut iter = rest.split('/');
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            Exchange(a, b)
        },
        _ => panic!("Bad move")
    }
}

fn load_data() -> Vec<Move> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.split(',') {
        out.push(get_move(l));
    }
    out
}

fn spin(s : &mut [u8], n : usize) {
    let l = s.len();
    s.rotate(l - n);
}

fn exchange(s : &mut [u8], a : usize, b : usize) {
    s.swap(a,b);
}

fn partner(s : &mut [u8], a : u8, b : u8) {
    let mut pos_a = 0;
    let mut pos_b = 0;
    for i in 0..s.len() {
        if s[i] == a {
            pos_a = i;
        }
        if s[i] == b {
            pos_b = i;
        }
    }
    s.swap(pos_a, pos_b);
}

fn dance(s : &String, moves : &Vec<Move>) -> String {
    let mut temp = s.clone();
    let mut out = unsafe { temp.as_bytes_mut() };
    for &m in moves.iter() {
        match m {
            Spin(n) => spin(out, n),
            Exchange(a, b) => exchange(out, a, b),
            Partner(a, b) => partner(out, a, b)
        }
    }
    String::from_utf8(out.to_vec()).unwrap()
}

fn main() {
    let mut start = String::from("abcdefghijklmnop");
    let v = load_data();
    let finish = dance(&start, &v);
    println!("{:?}", finish);
}