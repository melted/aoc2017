use std::io::*;

fn load_data() -> Vec<i32> {
    let mut input = String::new();
    let mut out : Vec<i32> = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        out.push(l.parse().unwrap());
    }
    out
}

fn run(v : &Vec<i32>, b : bool) -> i32 {
    let mut instr = v.to_vec();
    let mut counter = 0;
    let mut pos : i32 = 0;
    while pos >= 0 && pos < instr.len() as i32 {
        let p = pos as usize;
        pos += instr[p];
        if b {
            instr[p] += 1;
        } else {
            instr[p] += if instr[p] > 2 { -1 } else { 1 };
        }
        counter += 1;
    }
    counter
}
fn main() {
    let instr = load_data();
    println!("{}", run(&instr, true));
    println!("{}", run(&instr, false));
}