// It's not leet-code problem

fn main() {
    let n: u32 = 5;
    let res: u32 = finbonacci(n);

    println!("Final finbonacci is: {}", res);
}

// A simple implementation of finbonacci recursion algorithm.
fn finbonacci(n: u32) -> u32 {
    println!("N is {}", n);

    match n {
        0 => 1,
        1 => 1,
        _ => finbonacci(n - 1) + finbonacci(n - 2),
    }
}
