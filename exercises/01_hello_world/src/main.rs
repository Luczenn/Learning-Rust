// Exercise 01: Hello World
// Based on: https://doc.rust-lang.org/rust-by-example/hello.html
//
// Topics covered:
//   - Printing to the console with println!
//   - Formatted output with {} and {:?}
//   - Comments (line and block)
//   - The main() entry point

fn main() {
    // Line comments start with //
    /* Block comments are enclosed in /* ... */ */

    // --- Basic printing ---
    println!("Hello, World!");
    println!("Hello, {}!", "Rust"); // {} is a format placeholder

    // --- Printing numbers ---
    println!("The answer is {}", 42);
    println!("Pi is approximately {:.2}", 3.14159); // 2 decimal places

    // --- Debug formatting with {:?} ---
    // {:?} uses the Debug trait, useful for arrays and tuples
    let numbers = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    let point = (10, 20);
    println!("Point: {:?}", point);

    // --- Multiple placeholders ---
    println!("{} + {} = {}", 2, 3, 2 + 3);

    // --- Named arguments ---
    println!("{name} is {age} years old", name = "Alice", age = 30);

    // --- eprintln! writes to stderr instead of stdout ---
    eprintln!("This is a message on stderr");

    // --- Exercise: Try it yourself! ---
    // 1. Print your name and favourite number.
    // 2. Print a tuple (x, y, z) using {:?}.
    // 3. Use {:#?} (pretty-print debug) to print a larger array.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // println!("My name is {} and my favourite number is {}", "...", 0);
}

#[cfg(test)]
mod tests {
    // Test that the code compiles and the logic is correct.
    #[test]
    fn test_formatted_output() {
        let result = format!("{} + {} = {}", 2, 3, 2 + 3);
        assert_eq!(result, "2 + 3 = 5");
    }

    #[test]
    fn test_named_args() {
        let result = format!("{name} is {age}", name = "Alice", age = 30);
        assert_eq!(result, "Alice is 30");
    }
}
