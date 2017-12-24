use std::io::*;
use std::collections::HashSet;
use std::cmp::max;

fn load_data() -> HashSet<(i32,i32)> {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input);
    let mut out = HashSet::new();
    for l in input.lines() {
        let v : Vec<i32> = l.split('/').map(|x| x.parse().unwrap()).collect();
        out.insert((v[0], v[1]));
    }
    out
}

fn find_strongest(pieces : &HashSet<(i32, i32)>, n : i32) -> i32 {
    let eligible = pieces.iter().filter(|&&(a, b)| a == n || b == n);
    let mut out = 0;
    for p in eligible {
        let mut remaining = pieces.clone();
        remaining.remove(p);
        let m = if p.0 == n {
            find_strongest(&remaining, p.1)
        } else {
            find_strongest(&remaining, p.0)
        };
        out = max(out, m+p.0+p.1);
    }
    out
}

fn find_longest(pieces : &HashSet<(i32, i32)>, n : i32) -> (i32, i32) {
    let eligible = pieces.iter().filter(|&&(a, b)| a == n || b == n);
    let mut length = 0;
    let mut strength = 0;
    for p in eligible {
        let mut remaining = pieces.clone();
        remaining.remove(p);
        let (m, s) = if p.0 == n {
            find_longest(&remaining, p.1)
        } else {
            find_longest(&remaining, p.0)
        };
        if m+1 > length {
            length = m + 1;
            strength = s+p.0+p.1;
        } else if m+1 == length {
            strength = max(strength, s+p.0+p.1);
        }
    }
    (length, strength)
}

fn main() {
    let data = load_data();
    let m = find_strongest(&data, 0);
    let (l, s) = find_longest(&data, 0);
    println!("{:?}", m);
    println!("{:?}", s);
}