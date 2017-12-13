use std::io::*;

use std::collections::HashSet;

fn load_data() -> Vec<HashSet<i32>> {
    let mut input = String::new();
    let mut out = Vec::new();
    let _ = stdin().read_to_string(&mut input).unwrap();
    for l in input.lines() {
        out.push(parse_line(l));
    }
    out
}

fn parse_line(s : &str) -> HashSet<i32> {
    let mut out = HashSet::new();
    for i in s.rsplit(" ") {
        let t = i.trim().trim_matches(',');
        if t == "<->" { break; }
        out.insert(t.parse().unwrap());
    }
    out
}

fn find_connected(n : i32, data : &Vec<HashSet<i32>>) -> HashSet<i32> {
    let mut prospects : HashSet<i32> = HashSet::new();
    let mut explored : HashSet<i32> = HashSet::new();
    prospects.insert(n);
    while !prospects.is_empty() {
        let mut next : HashSet<i32> = HashSet::new();
        for x in prospects {
            explored.insert(x);
            for &i in data[x as usize].iter() {
                if !explored.contains(&i) {
                    next.insert(i);
                }
            }
        }
        prospects = next;
    }
    explored
}

fn count_connected(n : i32, data : &Vec<HashSet<i32>>) -> usize {
    find_connected(n, data).len()
}

fn find_groups(data : &Vec<HashSet<i32>>) -> i32 {
    let range = 0..2000;
    let mut unexplored : HashSet<i32> = range.collect();
    let mut count = 0;
    while !unexplored.is_empty() {
        let old_unexplored = unexplored.clone();
        let n = old_unexplored.iter().next().unwrap();
        let group = find_connected(*n, data);
        unexplored = old_unexplored.difference(&group).map(|&x| x).collect();
        count += 1;
    }
    count
}

fn main() {
    let v = load_data();
    println!("{}", count_connected(0, &v));
    println!("{}", find_groups(&v));
}