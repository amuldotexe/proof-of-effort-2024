/*
This main rs file is basically a record of all the hands-on code I will write without copy pasting a single character

Basically this will be a list of 100 functions which we will invoke to cover all the aspects that we learn everyday

All of these will be modules invoked by this main function

*/

// module declarations
mod f01_shadowing;
mod f02_panic;
// use modules
use f01_shadowing::f01_shadowing;
use f02_panic::f02_panic;
fn main() {
    println!("Hello, world!");
    f01_shadowing();
    f02_panic();
}
