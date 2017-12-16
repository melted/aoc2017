#![feature(slice_rotate)]
use std::io::*;

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

fn dance(start : &Vec<usize>, moves : &Vec<Move>) -> Vec<usize> {
    let mut out = start.clone();
    let l = start.len();
    for &m in moves.iter() {
        match m {
            Spin(n) => out.rotate(l - n),
            Exchange(a, b) => out.swap(a,b),
            Partner(a, b) => {
                let mut pos_a = 0;
                let mut pos_b = 0;
                for i in 0..l {
                    if out[i] == a {
                        pos_a = i;
                    }
                    if out[i] == b {
                        pos_b = i;
                    }
                }
                out.swap(pos_a, pos_b);
            }
        }
    }
    out
}

fn dance_rounds(start : &Vec<usize>, moves : &Vec<Move>, target : usize) -> Vec<usize> {
    let mut out = start.clone();
    for i in 1..target+1 {
        let next = dance(&out, moves);
        if next == *start {
            return dance_rounds(start, moves, target % i);
        }
        out = next;
    }
    out
}

fn order_to_string(order : &Vec<usize>) -> String {
    let mut out = String::new();
    let chars : Vec<char> = "abcdefghijklmnop".chars().collect();
    for &i in order.iter() {
        out.push(chars[i]);
    }
    out
}

fn main() {
    let init = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let v = load_data();
    let a = dance_rounds(&init, &v, 1);
    println!("{}", order_to_string(&a));
    let b = dance_rounds(&init, &v, 1000000000);
    println!("{}", order_to_string(&b));
}