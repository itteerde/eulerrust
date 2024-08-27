fn main() {
    println!("Problem 2");

    let mut result : i64 = 0;
    let mut mem: [i64;3] = [0,0,1];

    while mem[2] < 4000000 {

        if mem[2] % 2 == 0 {
            result += mem[2];
        }

        mem[0] = mem[1];
        mem[1] = mem[2];
        mem[2] = mem[0] + mem[1];
    }

    println!("result: {}", result);
}
