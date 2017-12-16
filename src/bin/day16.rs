#![feature(slice_rotate)]
use std::io::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize)
}

use Move::*;

fn get_move(s : &str) -> Move {
    let (first, rest) = s.split_at(1);
    match first {
        "s" => Spin(rest.parse().unwrap()),
        "p" => {
            let mut iter = rest.bytes();
            let a = (iter.next().unwrap() - 97) as usize;
            let _ = iter.next();
            let b = (iter.next().unwrap() - 97) as usize;
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

fn spin(s : &mut Vec<usize>, n : usize) {
    let l = s.len();
    s.rotate(l - n);
}

fn exchange(s : &mut Vec<usize>, a : usize, b : usize) {
    s.swap(a,b);
}

fn partner(s : &mut Vec<usize>, a : usize, b : usize) {
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

fn dance(start : &Vec<usize>, moves : &Vec<Move>) -> Vec<usize> {
    let mut out = start.clone();
    for &m in moves.iter() {
        match m {
            Spin(n) => spin(&mut out, n),
            Exchange(a, b) => exchange(&mut out, a, b),
            Partner(a, b) => partner(&mut out, a, b)
        }
    }
    out
}

fn dance_rounds(start : &Vec<usize>, moves : &Vec<Move>, target : usize) -> Vec<usize> {
    let mut out = start.clone();
    let mut memo = HashMap::new();
    memo.insert(start.clone(), 0);
    for i in 1..target+1 {
        let next = dance(&out, moves);
        if memo.contains_key(&next) {
            let s = memo.get(&next).unwrap();
            let period = i - s;
            let new_target = (target - s) % period + s;
            return dance_rounds(start, moves, new_target);
        } else {
            memo.insert(next.clone(), i);
        }
        out = next;
    }
    out
}

fn order_to_string(s : &str, order : &Vec<usize>) -> String {
    let mut out = String::new();
    let chars : Vec<char> = s.chars().collect();
    for &i in order.iter() {
        out.push(chars[i]);
    }
    out
}

fn main() {
    let start = String::from("abcdefghijklmnop");
    let init = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let v = load_data();
    let a = dance_rounds(&init,&v,1);
    println!("{}", order_to_string(&start, &a));
    let b = dance_rounds(&init, &v, 1000000000);
    println!("{}", order_to_string(&start, &b));
}