use std::io::*;
use std::collections::HashMap;

type Pos = (i32, i32);
static DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1,0)];

#[derive(Debug, Clone, Copy)]
enum Virus { Infected, Weakened, Flagged }

use Virus::*;

#[derive(Debug)]
struct State {
    infected : HashMap<Pos, Virus>,
    pos : Pos,
    dir : usize,
    count : i32
}

impl State {
    fn new(infected : HashMap<Pos, Virus>) -> Self {
        State { infected, pos: (0,0), dir: 0, count: 0 }
    }

    fn step(&mut self) {
        if self.infected.contains_key(&self.pos) {
            self.dir = (self.dir + 1) % 4;
            self.infected.remove(&self.pos);
        } else {
            self.dir = (self.dir + 3) % 4;
            self.infected.insert(self.pos.clone(), Infected);
            self.count += 1;
        }
        self.pos.0 += DIRS[self.dir].0;
        self.pos.1 += DIRS[self.dir].1;
    }

    fn step2(&mut self) {
        if self.infected.contains_key(&self.pos) {
            match self.infected[&self.pos] {
                Weakened => {
                    *self.infected.get_mut(&self.pos).unwrap() = Infected;
                    self.count += 1;
                },
                Infected => {
                    self.dir = (self.dir + 1) % 4;
                    *self.infected.get_mut(&self.pos).unwrap() = Flagged;
                },
                Flagged => {
                    self.dir = (self.dir + 2) % 4;
                    self.infected.remove(&self.pos);
                }
            }
        } else {
            self.dir = (self.dir + 3) % 4;
            self.infected.insert(self.pos.clone(), Weakened);
        }
        self.pos.0 += DIRS[self.dir].0;
        self.pos.1 += DIRS[self.dir].1;
    }
}

fn load_data() -> HashMap<Pos, Virus> {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input);
    let offset = (input.lines().count()/2) as i32;
    let mut out = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                let pos = (j as i32 - offset, i as i32 - offset);
                out.insert(pos, Infected);
            }
        }
    }
    out
}

fn main() {
    let data = load_data();
    let mut state = State::new(data.clone());
    for _ in 0..10000 {
        state.step();
    }
    println!("{:?}", state.count);
    let mut state2 = State::new(data);
    for _ in 0..10000000 {
        state2.step2();
    }
    println!("{:?}", state2.count);
}