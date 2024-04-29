fn main() {
    let mut f: [u128; 101] = [0; 101];
    f[0] = 0;
    f[1] = 1;
    
    for i in 2..101 {
        f[i] = f[i - 1] + f[i - 2];
    }
    
    for i in 0..101 {
        println!("F[{}] = {}", i, f[i]);
    }
}
