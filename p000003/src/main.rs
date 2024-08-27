
fn main() {
    println!("Problem 3");

    let sieve = primal::Sieve::new(10_000_000);

    let n : i64 = 600851475143;

    let mut i: i64 = (n as f64).sqrt() as i64;
    while i > 1 {

        if sieve.is_prime(i as usize) && n % i == 0 {
            println!("result: {}", i);
            return;
        }
        
        i-=1;
    }

}
