/*
This main rs file is basically a record of all the hands-on code I will write without copy pasting a single character

Basically this will be a list of 100 functions which we will invoke to cover all the aspects that we learn everyday

All of these will be modules invoked by this main function

*/

// module declarations
mod f01_shadowing;
mod f02_panic;
mod f03_factorial;
mod f04_tokio_01;
mod f05_some_none;
// use modules
use f01_shadowing::f01_shadowing;
use f02_panic::f02_panic;
use f03_factorial::f03_factorial;
use f05_some_none::f05_some_division;

fn main() {
    println!("Hello, world!");
    f01_shadowing();
    f02_panic();
    f03_factorial(5);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(f04_tokio_01::async_main());
    let f05_result = f05_some_division(1.0, 2.0);
    println!(
        "===f05_some_none===
        extra print {:?}
        ",
        f05_result
    );
    match f05_result {
        Some(f05_result) => println!("Some valid result {:?}", f05_result),
        None => println!("Denominator is 0"),
    }
}
