fn spinlock1(n : usize) -> usize {
    let mut spinlock = Vec::new();
    let step : usize = 359;
    let mut pos : usize = 0;
    spinlock.push(0);
    for i in 1..n+1 {
        pos = (pos + step) % i;
        if pos == i-1 {
            spinlock.push(i)
        } else {
            spinlock.insert(pos + 1, i);
        }
        pos += 1;
    }
    spinlock[(pos+1)%n]
}

fn spinlock2(n : usize) -> usize {
    let mut last = 0;
    let step : usize = 359;
    let mut pos : usize = 0;
    for i in 1..n+1 {
        pos = (pos + step) % i;
        if pos == 0 {
            last = i;
        }
        pos += 1;
    }
    last
}

fn main() {
    println!("{}", spinlock1(2017));
    println!("{}", spinlock2(50000000));
}