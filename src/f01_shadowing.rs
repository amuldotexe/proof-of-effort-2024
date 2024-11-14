pub fn f01_shadowing() {
    let x = 10;
    {
        println!("===f01_shadowing begins here===");
        println!("Is outer x carried inside here? x: {}", x);

        let x = x + 5; // Shadowing the outer `x`
                       // Inside this inner scope, `x` is shadowed by a new variable.
                       // Memory-wise, the inner `x` occupies a new space, while the outer `x` remains unchanged.
        println!("Inner x: {}", x); // Outputs: Inner x: 15
    }
    println!("Outer x: {}", x); // Outputs: Outer x: 10

    let x = 15;
    // Here, `x` is shadowed again in the outermost scope.
    // A fresh memory allocation is made for this `x`, keeping previous `x` values intact up to this point.
    println!("Outer x with shadowing: {}", x);
}
