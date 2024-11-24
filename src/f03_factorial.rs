pub fn f03_factorial(n: u32) -> () {
    println!("===f03_factorial===");
    let result = (1..=n).fold(1, |acc, i| acc * i);
    /* actually acc & i are no longer in scope beyond the fold function */
    println!("accumulator final value: {}", result);
}
