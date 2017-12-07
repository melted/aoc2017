#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::*;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

fn load_data() -> HashMap<String, Prog> {
    let mut input = String::new();
    let mut out = HashMap::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        if let Some(prog) = parse_prog(l) {
            out.insert(prog.name.clone(), prog);
        }
    }
    out
}

fn find_base(progs : &HashMap<String, Prog>) -> String {
    let mut supporters = HashSet::new();
    let mut supported = HashSet::new();
    for p in progs.values() {
        supporters.insert(p.name.clone());
        for s in p.supports.iter() {
            supported.insert(s.clone());
        }
    }
    supporters.difference(&supported).nth(0).unwrap().to_string()
}

fn calc_weight(s : &str, progs : &HashMap<String, Prog>, indent : usize) -> i32 {
    let mut out = progs[s].weight;
    let ist = "    ".repeat(indent);
    for n in progs[s].supports.iter() {
        let w = calc_weight(n, progs, indent+1);
        out += w;
        println!("{} {} {}",ist, n, w);
    }
    out
}

fn find_weight(base : &str, progs : &HashMap<String, Prog>, offset : i32) -> i32 {
    calc_weight(base, progs, 0);
    0
}

fn main() {
    let mut v = load_data();
    let base = find_base(&v);
    let weight = find_weight(&base, &v, 0);
    println!("{}", base);
}