use std::io::*;

#[derive(Debug, Clone, Copy)]
enum Arg {
    Reg(usize),
    Imm(i64)
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Set(Arg, Arg),
    Sub(Arg, Arg),
    Mul(Arg, Arg),
    Jnz(Arg, Arg)
}

use Arg::*;
use Instr::*;

#[derive(Debug)]
struct Machine {
    instrs : Vec<Instr>,
    regs : Vec<i64>,
    pc : usize,
    counter : i64
}

impl Machine {
    fn new(instrs : &Vec<Instr>) -> Self {
        let regs = vec![0; 8];
        Machine { instrs: instrs.clone(), regs, pc: 0, counter: 0 }
    }

    fn eval_arg(&self, a : Arg) -> i64 {
        match a {
            Reg(n) => self.regs[n],
            Imm(x) => x
        }
    }

    fn execute_instr(&mut self) -> bool {
        if self.pc >= self.instrs.len() {
            return true;
        }
        match self.instrs[self.pc] {
            Set(Reg(r), b) => self.regs[r] = self.eval_arg(b),
            Sub(Reg(r), b) => self.regs[r] -= self.eval_arg(b),
            Mul(Reg(r), b) => {
                self.counter += 1;
                self.regs[r] *= self.eval_arg(b)
            },
            Jnz(a, b) => 
                if self.eval_arg(a) != 0 {
                    let npc = self.pc as i64 + self.eval_arg(b);
                    if npc < 0 || npc >= self.instrs.len() as i64 {
                        return true;
                    }
                    self.pc = npc as usize;
                    return false;
                },
            _ => panic!("Bad instruction")
        }
        self.pc += 1;
        false
    }

    fn execute(&mut self) -> i64 {
        while !self.execute_instr() { }
        self.counter
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
        "set" => Set(args[0], args[1]),
        "sub" => Sub(args[0], args[1]),
        "mul" => Mul(args[0], args[1]),
        "jnz" => Jnz(args[0], args[1]),
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

fn is_prime(n : i64) -> bool {
    for i in 2..500 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn optimized() -> i64 {
    let mut b = 108100;
    let c = b + 17000;
    let mut h = 0;
    while !(c < b) {
        if !is_prime(b) {
            h += 1;
        }
        b += 17;
    }
    h
}

fn main() {
    let v = load_data();
    let mut m = Machine::new(&v);
    let res1 = m.execute();
    println!("{:?}", res1);
    println!("{}", optimized());
}