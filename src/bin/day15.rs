use std::iter::Iterator;

#[derive(Clone, Copy)]
struct Gen {
    val : u64,
    factor : u64
}

impl Gen {
    fn new(v : u64, f : u64) -> Self {
        Gen { val: v, factor: f }
    }
}

impl Iterator for Gen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let old = self.val;
        self.val = (old * self.factor) % 2147483647;
        Some(self.val)
    }
}

fn main() {
    let gen_a = Gen::new(516, 16807);
    let gen_b = Gen::new(190, 48271);
    let test_gen_a = Gen::new(65, 16807);
    let test_gen_b = Gen::new(8921, 48271);
    let count = gen_a.zip(gen_b).take(40000000)
                     .filter(|&(i,j)| i & 0xffff == j & 0xffff).count();
    println!("{}", count);
    let gen_a = Gen::new(516, 16807);
    let gen_b = Gen::new(190, 48271);
    let count2 = gen_a.filter(|&x| x % 4 == 0)
                      .zip(gen_b.filter(|&x| x % 8 == 0)).take(5000000)
                      .filter(|&(i,j)| i & 0xffff == j & 0xffff).count();
    println!("{}", count2);
}