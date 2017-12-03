use std::collections::HashMap;

fn generate_neighbours((x,y) : (i32, i32)) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for i in x-1..x+2 {
        for j in y-1..y+2 {
            out.push((i,j));
        }
    }
    out
}

fn change_dir((x,y) : (i32, i32)) -> bool {
    if x > 0 && y < 0 {
        return x == i32::abs(y) + 1;
    }
    i32::abs(x) == i32::abs(y) 
}

fn main() {
    let mut vals : HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (1, 0);
    let dirs = [(1, 0), (0,1), (-1,0), (0, -1)];
    let mut dir = 1;
    let target = 347991;
    vals.insert((0,0), 1);
    loop {
        let v = generate_neighbours(pos);
        let mut value = 0;
        for p in v.iter() {
            value += vals.get(p).unwrap_or(&0);
        }
        vals.insert(pos, value);
        if value >= target {
            println!("{}", value);
            break;
        }
        pos.0 += dirs[dir].0;
        pos.1 += dirs[dir].1;
        if change_dir(pos) {
            dir = (dir + 1) % 4
        }
    }
}