fn fib(k: u32) -> u128 {
    match k {
        0 => 0,
        1 => 1,
        _ => fib(k - 1) + fib(k - 2),
    }
}

use std::time::SystemTime;

fn main() {
    println!("k, Fk, Time it took");

    for k in 0..=50 {
        let before = SystemTime::now();
        let result = fib(k);
        let after = SystemTime::now();
        
        let difference = after.duration_since(before)
            .expect("System time moved backwards");

        println!("{}, {}, {:?}", k, result, difference);
    }
}
