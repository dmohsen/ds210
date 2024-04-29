use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a non-negative integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k: u32 = input.parse().expect("Not a good number!");

    let mut sum_of_squares: u64 = 0;
    for i in 1..=k {
        sum_of_squares += (i as u64).pow(2);
    }

    println!("The sum of squares up to {} is: {}", k, sum_of_squares);
}
