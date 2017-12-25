use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum State {
    A, B, C, D, E, F
}

use State::*;

#[derive(Debug)]
struct Machine {
    tape : HashSet<i32>,
    pos : i32,
    state : State
}

impl Machine {
    fn new() -> Machine {
        Machine { tape: HashSet::new(), pos: 0, state: State::A }
    }

    fn step(&mut self) {
        let current = self.tape.contains(&self.pos);
        let (val, dpos, next) = match (self.state, current) {
            (A, false) => (true, 1, B),
            (A, true) => (false, -1, C),
            (B, false) => (true, -1, A),
            (B, true) => (true, 1, C),
            (C, false) => (true, 1, A),
            (C, true) => (false, -1, D),
            (D, false) => (true, -1, E),
            (D, true) => (true, -1, C),
            (E, false) => (true, 1, F),
            (E, true) => (true, 1, A),
            (F, false) => (true, 1, A),
            (F, true) => (true, 1, E),
        };
        match (current, val) {
            (false, true) => { self.tape.insert(self.pos); }, 
            (true, false) => { self.tape.remove(&self.pos); },
            _ => {}
        }
        self.pos += dpos;
        self.state = next;
    }
}

fn main() {
    let mut machine = Machine::new();
    for _ in 0..12261543 {
        machine.step();
    }
    println!("{:?}", machine.tape.len());
}