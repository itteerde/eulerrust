fn main() {
    println!("Problem 1");

    let mut result : i64 = 0;
    for n in 1..999 {
        if n % 3 == 0 || n % 5 == 0 {
            result += n;
        }
    }
    println!("result: {}", result);
}
