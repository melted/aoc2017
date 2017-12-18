use std::io::*;
use std::rc::Rc;
use std::collections::VecDeque;
use std::cell::RefCell;

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

type QueueRef = Rc<RefCell<VecDeque<i64>>>;

#[derive(Debug)]
struct Machine {
    instrs : Vec<Instr>,
    regs : Vec<i64>,
    pc : usize,
    in_queue : QueueRef,
    out_queue : QueueRef,
    send_counter : i64
}

impl Machine {
    fn new(instrs : &Vec<Instr>, in_queue : QueueRef, out_queue : QueueRef) -> Self {
        let regs = vec![0; 26];
        Machine { instrs: instrs.clone(), regs, pc: 0, in_queue, out_queue, send_counter: 0 }
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
            Snd(a) => {
                self.send_counter += 1; 
                self.out_queue.borrow_mut().push_front(self.eval_arg(a))
            },
            Set(Reg(r), b) => self.regs[r] = self.eval_arg(b),
            Add(Reg(r), b) => self.regs[r] += self.eval_arg(b),
            Mul(Reg(r), b) => self.regs[r] *= self.eval_arg(b),
            Mod(Reg(r), b) => self.regs[r] %= self.eval_arg(b),
            Jgz(a, b) => 
                if self.eval_arg(a) > 0 {
                    let npc = self.pc as i64 + self.eval_arg(b);
                    self.pc = npc as usize;
                    return false;
                },
            Rcv(Reg(r)) =>
                if let Some(v) = self.in_queue.borrow_mut().pop_back() {
                    self.regs[r] = v;
                } else {
                    return true;
                },
            _ => panic!("Bad instruction")
        }
        self.pc += 1;
        false
    }

    fn execute_until_rcv(&mut self) -> i64 {
        while !self.execute_instr() { }
        self.out_queue.borrow_mut().pop_front().unwrap()
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

fn execute_tandem(m1 : &mut Machine, m2 : &mut Machine) -> i64 {
    loop {
        if m1.execute_instr() {
            if m2.execute_instr() {
                break;
            }
        }
        if m2.execute_instr() {
            if m1.execute_instr() {
                break;
            }
        }
    }
    m2.send_counter
}

fn main() {
    let v = load_data();
    let mut m = Machine::new(&v, Rc::new(RefCell::new(VecDeque::new())),
                             Rc::new(RefCell::new(VecDeque::new())));
    let res1 = m.execute_until_rcv();
    println!("{:?}", res1);
    let q1 = Rc::new(RefCell::new(VecDeque::new()));
    let q2 = Rc::new(RefCell::new(VecDeque::new()));
    let mut m1 = Machine::new(&v, Rc::clone(&q1), Rc::clone(&q2));
    let mut m2 = Machine::new(&v, Rc::clone(&q2), Rc::clone(&q1));
    m2.regs[15] = 1;
    let res2 = execute_tandem(&mut m1, &mut m2);
    println!("{}", res2);
}