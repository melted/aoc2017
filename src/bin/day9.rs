use std::io::*;
use std::str::Chars;

fn load_data() -> String {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    input
}

fn parse_garbage(t : &mut Chars) -> i32 {
    let mut count = 0;
    loop {
        match t.next().unwrap() {
            '>' => break,
            '!' => { let _ = t.next(); },
            _ => count += 1
        }
    }
    count
}

fn parse_group(t : &mut Chars, level : i32) -> (i32, i32) {
    let mut score = 0;
    let mut garbage = 0;
    loop {
        match t.next().unwrap() {
            '{' => { let (s, g) = parse_group(t, level + 1);
                     score += s;
                     garbage += g },
            '<' => garbage += parse_garbage(t),
            '}' => return (score + level, garbage),
            ',' => {},
            c => println!("Bad char {}", c)
        }
    }
}

fn parse(s : &str) -> (i32, i32) {
    let mut tokens = s.chars();
    let ch = tokens.next().unwrap();
    if ch == '{' {
        return parse_group(&mut tokens, 1);
    }
    panic!("Not a group");
}

fn main() {
    let src = load_data();
    let (score, garbage) = parse(&src);
    println!("{} {}", score, garbage);
}