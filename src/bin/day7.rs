#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::*;
use regex::Regex;

#[derive(Debug)]
struct Prog {
    name : String,
    weight : i32,
    supports : Vec<String>
}

fn parse_prog(s : &str) -> Option<Prog> {
    lazy_static!(
        static ref RE : Regex = Regex::new(r"([a-z]+) \((\d*)\)(?: -> ((?:[a-z]+, )*[a-z]+))*").unwrap();
    );
    RE.captures(s).and_then(
        |c| {
            let mut children = Vec::new();
            if let Some(t) = c.get(3) {
                children = t.as_str().split(", ").map(|x| x.to_string()).collect();
            }
            Some(Prog { name: c[1].to_string(),
                        weight: c[2].parse().unwrap(), 
                        supports: children })
        }) 
}

fn load_data() -> Vec<Prog> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        if let Some(prog) = parse_prog(l) {
            out.push(prog);
        }
    }
    out
}

fn main() {
    let v = load_data();
    println!("{:?}",v);
}