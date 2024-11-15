use std::panic;

pub fn f02_panic() {
    println!("===f02 panic===");
    let result = panic::catch_unwind(|| {
        println!("About to panic!");
        panic!("This is a panic."); // Deliberately trigger panic
    });
    match result {
        Ok(_value) => {
            // value has type T - the successful return value from the closure
            println!("No panic occurred.");
        }
        Err(_panic_payload) => {
            println!("Caught a panic!");
        }
    }
}
