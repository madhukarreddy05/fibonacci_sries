use num_bigint::BigUint;
use num_traits::One;
use std::io;
fn main() {
    println!("Enter the Nth fibonacci number to be printed.");
    let mut n_th_fibo = String::new();
    io::stdin()
        .read_line(&mut n_th_fibo)
        .expect("failed to read line");
    let n_th_fibo: u128 = match n_th_fibo.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid number");
            return;
        }
    };
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::one();
    println!("fibonacci series");
    for _ in 0..n_th_fibo {
        println!("{}", a);
        let next = &a + &b;
        a = b;
        b = next;
    }
}
