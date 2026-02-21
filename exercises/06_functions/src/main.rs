// Exercise 06: Functions and Closures
// Based on: https://doc.rust-lang.org/rust-by-example/fn.html
//
// Topics covered:
//   - Function syntax and return values
//   - Recursion
//   - Higher-order functions (passing functions as arguments)
//   - Closures (anonymous functions that capture their environment)
//   - Iterators with closures: map, filter, fold

// --- Basic functions ---

// Functions that return a value: the last expression (without `;`) is returned.
fn add(a: i32, b: i32) -> i32 {
    a + b // implicit return
}

fn square(n: i32) -> i32 {
    n * n
}

// Functions can also use `return` explicitly (often used for early returns).
fn absolute_value(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}

// --- Recursion ---
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// --- Higher-order functions ---
// Functions can accept other functions as parameters using `fn` types or generics.
fn apply<F: Fn(i32) -> i32>(f: F, value: i32) -> i32 {
    f(value)
}

fn apply_twice<F: Fn(i32) -> i32>(f: F, value: i32) -> i32 {
    f(f(value))
}

fn main() {
    // --- Basic function calls ---
    println!("add(3, 4)          = {}", add(3, 4));
    println!("square(5)          = {}", square(5));
    println!("absolute_value(-7) = {}", absolute_value(-7));

    // --- Recursion ---
    println!("\nFactorials:");
    for n in 0..=10 {
        println!("  {}! = {}", n, factorial(n));
    }

    println!("\nFibonacci sequence (first 10 terms):");
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!();

    // --- Closures ---
    // Closures are anonymous functions with a concise syntax.
    // They can capture variables from the surrounding scope.
    let double = |x: i32| x * 2;
    let add_five = |x| x + 5; // type can be inferred from usage

    println!("\ndouble(6)     = {}", double(6));
    println!("add_five(10)  = {}", add_five(10));

    // Closure capturing environment
    let offset = 10;
    let add_offset = |x| x + offset; // captures `offset` by reference
    println!("add_offset(5) = {}", add_offset(5));

    // Multi-line closure
    let process = |x: i32| {
        let doubled = x * 2;
        let incremented = doubled + 1;
        incremented
    };
    println!("process(4)    = {}", process(4));

    // --- Higher-order functions ---
    println!("\napply(square, 4)          = {}", apply(square, 4));
    println!("apply(double, 4)          = {}", apply(double, 4));
    println!("apply_twice(double, 3)    = {}", apply_twice(double, 3));
    println!("apply_twice(|x| x+1, 0)  = {}", apply_twice(|x| x + 1, 0));

    // --- Iterator adaptors with closures ---
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("\nSquares:    {:?}", squares);

    // filter: keep elements matching a predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens:      {:?}", evens);

    // fold (reduce): accumulate a result
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum 1..=10: {}", sum);

    // Chain multiple adaptors
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum of even squares: {}", sum_of_even_squares);

    // --- move closures ---
    // `move` transfers ownership of captured variables into the closure.
    let message = String::from("hello");
    let greeting = move || println!("Message: {}", message);
    greeting();
    // println!("{}", message); // ERROR: `message` was moved into the closure

    // --- Exercise: Try it yourself! ---
    // 1. Write a function `is_prime(n: u64) -> bool`.
    // 2. Use `filter` to collect all primes up to 50 from a range.
    // 3. Write a closure that captures a mutable counter and increments it each call.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // fn is_prime(n: u64) -> bool { ... }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_closure_capture() {
        let offset = 10;
        let add_offset = |x: i32| x + offset;
        assert_eq!(add_offset(5), 15);
    }

    #[test]
    fn test_iterator_sum() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_filter_map() {
        let result: Vec<i32> = (1..=5).filter(|x| x % 2 == 0).map(|x| x * x).collect();
        assert_eq!(result, vec![4, 16]);
    }
}
