
fn scramble(input : &Vec<i32>) -> Vec<i32> {
    let mut skip_size = 0;
    let mut pos = 0;
    let range = 0..256;
    let mut data : Vec<i32> = range.collect();
    let len = data.len();
    for i in input {
        let mut next = data.clone();
        let mut v = Vec::new();
        let end = pos + i;
        if (end < len) {
            v = data[pos..end].iter().rev().collect();
            let (_, start) = data.split_at_mut(pos);
            let (target, _) = data.split_at_mut(end);
            target.copy_from_slice(v);
        } else {

        }
    }
}



fn main() {
    let input = vec![187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216];
    let v = scramble(&input);
    println!("{}", v[0] * v[1]);
}