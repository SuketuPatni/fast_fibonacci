// // Inspired by https://www.nayuki.io/res/fast-fibonacci-algorithms/fastfibonacci.py

// use std::io;
use rug::Integer;
// use std::time::Instant;

fn fibonacci(n: u64) -> (Integer, Integer) {
    if n == 0 {
        (Integer::new(), Integer::from(1))
    } else {
        let temp = fibonacci(n / 2);
        let a = temp.0;
        let b = temp.1;
        let c = &a * Integer::from(&b * 2 - &a);
        let d = Integer::from(&a * &a) + Integer::from(&b * &b);
        let e = Integer::from(&c + &d);
        if n % 2 == 0 {
            (c, d)
        } else {
            (d, e)
        }
    }
}

fn main() {
    println!("{:?}", fibonacci(10000).0);
}
