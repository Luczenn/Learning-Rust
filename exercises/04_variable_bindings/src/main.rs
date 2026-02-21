// Exercise 04: Variable Bindings
// Based on: https://doc.rust-lang.org/rust-by-example/variable_bindings.html
//
// Topics covered:
//   - `let` bindings and type inference
//   - Mutability with `mut`
//   - Shadowing (re-binding with let)
//   - Scope and block expressions
//   - Freezing (immutable rebinding of a mutable variable)
//   - Constants and statics

// Constants are always immutable and require a type annotation.
// They can be declared in any scope, including global scope.
const MAX_POINTS: u32 = 100_000;

// Statics have a fixed memory location and live for the entire program.
static GREETING: &str = "Hello from a static!";

fn main() {
    // --- Basic binding ---
    let x = 5; // type inferred as i32
    println!("x = {}", x);

    // --- Explicit type annotation ---
    let y: f64 = 3.14;
    println!("y = {}", y);

    // --- Mutability ---
    let mut count = 0;
    println!("count before: {}", count);
    count += 1;
    count += 1;
    println!("count after:  {}", count);

    // --- Shadowing ---
    // You can re-declare a variable with `let`, even changing its type.
    let spaces = "   ";          // &str
    println!("spaces (str): '{}'", spaces);
    let spaces = spaces.len();   // usize – different type, same name
    println!("spaces (len): {}", spaces);

    let number = 5;
    let number = number * 2;     // shadows the previous binding
    let number = number + 1;
    println!("shadowed number: {}", number);

    // --- Block expressions ---
    // Blocks are expressions: the last expression without a semicolon is returned.
    let result = {
        let a = 10;
        let b = 20;
        a + b // no semicolon → this is the value of the block
    };
    println!("Block result: {}", result);

    // --- Scope ---
    let outer = "I'm in the outer scope";
    {
        let inner = "I'm in the inner scope";
        println!("{}", outer); // outer is visible here
        println!("{}", inner);
    }
    // println!("{}", inner); // ERROR: inner is out of scope here
    println!("{}", outer);    // outer is still valid

    // --- Freezing ---
    // When a mutable variable is shadowed by an immutable one, it is "frozen"
    // within the inner scope.
    let mut mutable = 42;
    {
        let mutable = mutable; // immutable shadow
        // mutable = 100;       // would be a compile error here
        println!("frozen mutable: {}", mutable);
    }
    mutable = 100; // still mutable in the outer scope
    println!("mutable after: {}", mutable);

    // --- Constants and statics ---
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("{}", GREETING);

    // --- Declare first, assign later ---
    let delayed;
    // println!("{}", delayed); // ERROR: use of possibly-uninitialized variable
    delayed = "assigned now";
    println!("delayed: {}", delayed);

    // --- Exercise: Try it yourself! ---
    // 1. Declare `temperature` as mutable f64, then shadow it with an integer.
    // 2. Use a block expression to compute the factorial of 5.
    // 3. Declare a constant for the speed of light (299_792_458 m/s).
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // const SPEED_OF_LIGHT: u64 = 299_792_458;
    // println!("Speed of light: {} m/s", SPEED_OF_LIGHT);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shadowing_changes_type() {
        let val = "hello";
        let val = val.len();
        assert_eq!(val, 5usize);
    }

    #[test]
    fn test_block_expression() {
        let result = {
            let a = 10;
            let b = 20;
            a + b
        };
        assert_eq!(result, 30);
    }

    #[test]
    fn test_mutation() {
        let mut x = 0;
        x += 5;
        assert_eq!(x, 5);
    }

    #[test]
    fn test_constant() {
        assert_eq!(super::MAX_POINTS, 100_000);
    }
}
