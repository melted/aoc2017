use std::io::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Pattern = Vec<Vec<u8>>;

fn parse_pattern(s : &str) -> Pattern {
    let mut out = Vec::new();
    for i in s.split('/') {
        out.push(i.chars().map(|x| if x == '#' { 1 } else { 0 }).collect());
    }
    out
}

fn load_data() -> HashMap<Pattern, Pattern> {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input);
    let mut out = HashMap::new();
    for l in input.lines() {
        let mut parts = l.split_whitespace();
        let seed = parse_pattern(parts.next().unwrap());
        let _ = parts.next(); // =>
        let result = parse_pattern(parts.next().unwrap());
        for p in transformations(&seed) {
            out.insert(p, result.clone());
        }
    }
    out
}

fn flip_ud(p : &Pattern) -> Pattern {
    let mut out = p.clone();
    out.reverse();
    out
}

fn flip_lr(p : &Pattern) -> Pattern {
    let mut out = p.clone();
    for i in 0..out.len() {
        out[i].reverse();
    }
    out
}

fn rotate(p : &Pattern) -> Pattern {
    let mut out = p.clone();
    for i in 0..p.len() {
        for j in 0..p.len() {
            out[j][i] = p[i][j];
        }
    }
    out
}

fn transformations(p : &Pattern) -> HashSet<Pattern> {
    let mut out = HashSet::new();
    let mut s = p.clone();
    for _ in 0..4 {
        out.insert(s.clone());
        out.insert(flip_ud(&s));
        out.insert(flip_lr(&s));
        out.insert(flip_ud(&flip_lr(&s)));
        s = rotate(&s);
    }
    out
}

fn break_down(p : &Pattern, n : usize) -> Vec<Vec<Pattern>> {
    let size = p.len()/n;
    let mut out = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            let mut chunk = Vec::new();
            for y in 0..n {
                chunk.push(vec![0; n]);
                for x in 0..n {
                    chunk[y][x] = p[i*n+y][j*n+x];
                }
            }
            row.push(chunk);
        }
        out.push(row);
    }
    out
}

fn paste_together(v : &Vec<Vec<Pattern>>) -> Pattern {
    let size = v.len()*v[0][0].len();
    let l = v[0][0].len();
    let mut out = Vec::new();
    for i in 0..size {
        out.push(vec![0; size]);
    }
    for y in 0..v.len() {
        for x in 0..v.len() {
            let chunk = &v[y][x];
            for i in 0..chunk.len() {
                for j in 0..chunk.len() {
                    out[y*l+i][x*l+j] = chunk[i][j];
                }
            }
        }
    }
    out
}

fn step(p : &Pattern, rules : &HashMap<Pattern, Pattern>) -> Pattern {
    let size = p.len();
    let chunk_size = if size % 2 == 0 { 2 } else { 3 };
    let mut parts = break_down(p, chunk_size);
    for i in parts.iter_mut() {
        for j in i.iter_mut() {
            let nj = rules[j].clone();
            *j = nj;
        }
    }
    paste_together(&parts)
}

fn main() {
    let start = parse_pattern(".#./..#/###");
    let rules = load_data();
    let mut output = start.clone();
    let mut count = 0;
    for i in 0..18 {
        output = step(&output, &rules);
        count = output.iter().map(|ref x| x.iter().filter(|&&x| x == 1).count()).sum();
        if i == 4 {
            println!("{:?}", count);
        }
    }
    println!("{:?}", count);
}