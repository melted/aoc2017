use std::io::*;
use std::collections::HashSet;

fn load_data() -> String {
    let mut input = String::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let phrases = load_data();
    let mut invalid = 0;
    let mut invalid2 = 0;
    let mut count = 0;
    for p in phrases.lines() {
        count += 1;
        let mut words = HashSet::new();
        let mut awords = HashSet::new();
        for w in p.split_whitespace() {
            if words.contains(w) {
                invalid += 1;
                break;
            }
            words.insert(w);
        }
        for aw in p.split_whitespace() {
            let mut ch : Vec<char> = aw.chars().collect();
            ch.sort();
            if awords.contains(&ch) {
                invalid2 += 1;
                break;
            }
            awords.insert(ch);
        }
    }
    println!("{}", count-invalid);
    println!("{}", count-invalid2);
}