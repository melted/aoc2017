use std::io::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Op { Inc, Dec }

#[derive(Debug)]
enum Cond {
    Equ,
    Lt,
    LtEq,
    Gt,
    GtEq,
    NEq
}

#[derive(Debug)]
struct Instr {
    reg : String,
    op : Op,
    val : i32,
    creg : String,
    cond : Cond,
    cval : i32
}

fn parse_instruction(s : &str) -> Option<Instr> {
    println!("{}", s);
    let mut iter = s.split_whitespace();
    let reg = iter.next().unwrap().to_string();
    let op = match iter.next().unwrap() {
        "inc" => Op::Inc,
        "dec" => Op::Dec,
        _ => panic!("bang")
    };
    let val = iter.next().unwrap().parse().unwrap();
    iter.next(); // if
    let creg = iter.next().unwrap().to_string();
    let cond = match iter.next().unwrap() {
        "==" => Cond::Equ,
        "!=" => Cond::NEq,
        ">" => Cond::Gt,
        ">=" => Cond::GtEq,
        "<" => Cond::Lt,
        "<=" => Cond::LtEq,
        _ => panic!("boom")
    };
    let cval = iter.next().unwrap().parse().unwrap();
    Some(Instr { reg: reg, op: op, val: val, creg: creg, cond: cond, cval: cval })
}

fn load_data() -> Vec<Instr> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        out.push(parse_instruction(l).unwrap());
    }
    out
}

fn execute(inst : Instr, state : &mut HashMap<String, i32>) {
    
}

fn main() {
    let instrs = load_data();
    println!("{:?}", instrs);
}