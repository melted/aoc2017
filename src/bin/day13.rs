use std::io::*;
use std::collections::HashMap;

fn load_data() -> HashMap<i32, i32> {
    let mut input = String::new();
    let mut out = HashMap::new();
    let _ = stdin().read_to_string(&mut input);
    for l in input.lines() {
        let mut parts = l.split(':');
        let x = parts.next().unwrap().trim().parse().unwrap();
        let p = parts.next().unwrap().trim().parse().unwrap();
        out.insert(x, p);
    }
    out
}

fn traverse(data :&HashMap<i32, i32>, delay : i32) -> (i32, bool) {
    let mut score = 0;
    let mut hit = false;
    for i in 0..100 {
        let t = i + delay;
        if data.contains_key(&i) {
            let p = 2*data[&i]-2;
            if t % p == 0 {
                score += data[&i]*i;
                hit = true;
            }
        }
    }
    (score, hit)
}

fn main() {
    let data = load_data();
    println!("{}", traverse(&data, 0).0);
    for d in 0.. {
        let (_, hit) = traverse(&data, d);
        if !hit {
            println!("{}", d);
            break;
        }
    }
}