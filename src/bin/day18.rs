use std::io::*;

#[derive(Debug, Clone, Copy)]
enum Arg {
    Reg(usize),
    Imm(i64)
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Snd(Arg),
    Set(Arg, Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Mod(Arg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg)
}

use Arg::*;
use Instr::*;

#[derive(Debug)]
struct Machine {
    instrs : Vec<Instr>,
    regs : Vec<i64>,
    last_sound : i64,
    pc : usize
}

impl Machine {
    fn new(instrs : &Vec<Instr>) -> Self {
        let regs = vec![0; 26];
        Machine { instrs, regs, last_sound: 0, pc: 0 }
    }

    fn execute_instr(&mut self) -> bool {
        match (instrs[pc]) {
            
        }
    }
}

fn get_arg(s : &str) -> Arg {
    if let Ok(n) = s.parse() {
        Imm(n)
    } else {
        let c = s.bytes().next().unwrap() as usize;
        Reg(c - 97)
    }
}

fn parse_instruction(s : &str) -> Instr {
    let mut words = s.split_whitespace();
    let cmd = words.next().unwrap();
    let args : Vec<Arg> = words.map(get_arg).collect();
    match cmd {
        "snd" => Snd(args[0]),
        "set" => Set(args[0], args[1]),
        "add" => Add(args[0], args[1]),
        "mul" => Mul(args[0], args[1]),
        "mod" => Mod(args[0], args[1]),
        "rcv" => Rcv(args[0]),
        "jgz" => Jgz(args[0], args[1]),
        _ => panic!("Bad command")
    }
}

fn load_data() -> Vec<Instr> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        out.push(parse_instruction(l));
    }
    out
}

fn main() {
    let v = load_data();
    println!("{:?}", v)
}