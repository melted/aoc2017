#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type V3 = (i64, i64, i64);

#[derive(Debug, Clone, Copy)]
struct Particle {
    pos : V3,
    vel : V3,
    acc : V3
}

fn parse_v3(s : &str) -> Option<V3> {
    let v : Vec<i64> = s.split(',').map(|x| x.parse().unwrap()).collect();
    Some((v[0], v[1], v[2]))
} 

fn parse_particle(s : &str) -> Particle {
    lazy_static!(
        static ref RE : Regex = Regex::new(r"p=<(.*)>, v=<(.*)>, a=<(.*)>").unwrap();
    );
    RE.captures(s).and_then(
        |c| {
            Some(Particle {
                pos: parse_v3(&c[1]).unwrap(),
                vel: parse_v3(&c[2]).unwrap(),
                acc: parse_v3(&c[3]).unwrap()
            })
        }
    ).unwrap()
}

fn load_data() -> Vec<Particle> {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input);
    input.lines().map(parse_particle).collect()
}

fn tick(p : &mut Particle) {
    p.vel.0 += p.acc.0;
    p.vel.1 += p.acc.1;
    p.vel.2 += p.acc.2;
    p.pos.0 += p.vel.0;
    p.pos.1 += p.vel.1;
    p.pos.2 += p.vel.2;
}

fn tick_world(v : &mut Vec<Particle>) {
    let mut seen = HashMap::new();
    let mut destroyed = HashSet::new();
    for (i, p) in v.iter_mut().enumerate() {
        tick(p);
        if seen.contains_key(&p.pos) {
            let other = seen[&p.pos];
            destroyed.insert(other);
            destroyed.insert(i);
        } else {
            seen.insert(p.pos, i);
        }
    }
    let mut d : Vec<usize> = destroyed.into_iter().collect();
    d.sort_unstable_by_key(|x| 10000000-x);
    for &i in d.iter() {
        v.remove(i);
    }
}

fn manhattan(v : &V3) -> i64 {
    i64::abs(v.0) + i64::abs(v.1) + i64::abs(v.2)
}

fn slowest(v : &Vec<Particle>) {
    let acc = v.iter().map(|&p| manhattan(&p.acc)).min().unwrap();
    for i in v.iter().filter(|&&p| manhattan(&p.acc) == acc) {
        println!("{:?}", i);
    }
}

fn main() {
    let mut v = load_data();
    slowest(&v);
    for _ in 0..10000 {
        tick_world(&mut v);
    }
    println!("{}", v.len()); 
}