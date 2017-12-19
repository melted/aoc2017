use std::io::*;

fn load_data() -> Vec<Vec<char>> {
    let mut out = Vec::new();
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input);
    for l in input.lines() {
        out.push(l.chars().collect());
    }
    out
}

fn trace(m : &Vec<Vec<char>>) -> (String, i32) {
    let mut out = String::new();
    let mut dir : (i32, i32) = (1, 0);
    let mut p = (0, 0);
    let mut steps = 0;
    for i in 0..m[0].len() {
        if m[0][i] != ' ' {
            p = (0,i);
            break;
        }
    }

    loop {
        let c = m[p.0][p.1];
        if c.is_alphabetic() {
            out.push(c);
        }
        if c == ' ' {
            break;
        }
        if c == '+' {
            match dir {
                (0, _) => 
                    if m[p.0 - 1][p.1] == '|' {
                        dir = (-1, 0);
                    } else if m[p.0 + 1][p.1] == '|' {
                        dir = (1, 0);
                    } else {
                        break;
                    },
                (_, 0) => 
                    if m[p.0][p.1 - 1] == '-' {
                        dir = (0, -1);
                    } else if m[p.0][p.1 + 1] == '-' {
                        dir = (0, 1);
                    } else {
                        break;
                    },
                (_, _) => break
            }
        }
        p.0 = (p.0 as i32 + dir.0) as usize;
        p.1 = (p.1 as i32 + dir.1) as usize;
        steps += 1;
    }
    (out, steps)
}

fn main() {
    let v = load_data();
    let (s, n) = trace(&v);
    println!("{} {}", s, n);
}