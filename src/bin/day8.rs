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
    Some(Instr { reg, op, val, creg, cond, cval })
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

fn execute(instr : &Instr, state : &mut HashMap<String, i32>, maxes : &mut Vec<i32>) {
   let cond_result = {
        let cr_val = state.get(&instr.creg).unwrap_or(&0);
        match instr.cond {
            Cond::Equ => *cr_val == instr.cval,
            Cond::NEq => *cr_val != instr.cval,
            Cond::Gt => *cr_val > instr.cval,
            Cond::GtEq => *cr_val >= instr.cval,
            Cond::Lt => *cr_val < instr.cval,
            Cond::LtEq => *cr_val <= instr.cval
        }
   };
   if cond_result {
       let entry = state.entry(instr.reg.clone()).or_insert(0);
       match instr.op {
           Op::Inc => *entry += instr.val,
           Op::Dec => *entry -= instr.val
       }
   }
   maxes.push(*state.values().max().unwrap_or(&0));
}

fn main() {
    let instrs = load_data();
    let mut state = HashMap::new();
    let mut maxes = Vec::new();
    for i in instrs.iter() {
        execute(i, &mut state, &mut maxes);
    }
    let m = state.values().max().unwrap();
    let m2 = maxes.iter().max().unwrap();
    println!("{} {}", m, m2);
}