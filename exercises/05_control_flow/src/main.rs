// Exercise 05: Control Flow
// Based on: https://doc.rust-lang.org/rust-by-example/flow_control.html
//
// Topics covered:
//   - if / else if / else (as expressions)
//   - loop with break and continue
//   - while loops
//   - for loops and ranges
//   - match (pattern matching)
//   - if let and while let

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }
}

fn classify_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

fn main() {
    // --- if / else if / else ---
    // In Rust, `if` is an expression, so it can return a value.
    let number = 7;
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, description);
    println!("10 is {}", classify_number(10));
    println!("-3 is {}", classify_number(-3));
    println!("0 is {}", classify_number(0));

    // --- loop ---
    // `loop` runs forever until a `break`.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // loop returns a value via break
        }
    };
    println!("\nLoop result (counter*2 when counter==5): {}", result);

    // `continue` skips the rest of the current iteration
    print!("Even numbers 0-9: ");
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        if i % 2 != 0 {
            i += 1;
            continue;
        }
        print!("{} ", i);
        i += 1;
    }
    println!();

    // --- Nested loops with labels ---
    'outer: for x in 0..4 {
        for y in 0..4 {
            if x == 2 && y == 2 {
                println!("Breaking outer loop at ({}, {})", x, y);
                break 'outer;
            }
        }
    }

    // --- while ---
    let mut n = 1u32;
    print!("\nPowers of 2 up to 1024: ");
    while n <= 1024 {
        print!("{} ", n);
        n *= 2;
    }
    println!();

    // --- for and ranges ---
    // `a..b`  → exclusive end (a to b-1)
    // `a..=b` → inclusive end (a to b)
    print!("1 to 5 (exclusive): ");
    for i in 1..6 {
        print!("{} ", i);
    }
    println!();

    print!("1 to 5 (inclusive): ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Iterating over a collection
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  fruit[{}] = {}", index, fruit);
    }

    // --- FizzBuzz with match ---
    println!("\nFizzBuzz 1-20:");
    for i in 1..=20 {
        print!("{} ", fizzbuzz(i));
    }
    println!();

    // --- match ---
    let coin = 25u32;
    let name = match coin {
        1  => "penny",
        5  => "nickel",
        10 => "dime",
        25 => "quarter",
        _  => "unknown",
    };
    println!("\n{} cents is a {}", coin, name);

    // Match with binding (@)
    let num = 13;
    let category = match num {
        n @ 1..=12  => format!("{} is between 1 and 12", n),
        n @ 13..=19 => format!("{} is a teen", n),
        n           => format!("{} is something else", n),
    };
    println!("{}", category);

    // --- if let ---
    // Concise single-pattern matching; useful with Option
    let favourite: Option<&str> = Some("chocolate");
    if let Some(flavour) = favourite {
        println!("\nFavourite flavour: {}", flavour);
    }

    // --- while let ---
    let mut stack = vec![1, 2, 3];
    print!("Popping stack: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // --- Exercise: Try it yourself! ---
    // 1. Rewrite `classify_number` using match.
    // 2. Print a multiplication table (1-5) using nested for loops.
    // 3. Use `if let` to safely get the square root only when a number is >= 0.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // for row in 1..=5 {
    //     for col in 1..=5 {
    //         print!("{:3} ", row * col);
    //     }
    //     println!();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-5), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(5), "positive");
    }

    #[test]
    fn test_loop_return_value() {
        let mut c = 0;
        let result = loop {
            c += 1;
            if c == 3 {
                break c * 10;
            }
        };
        assert_eq!(result, 30);
    }

    #[test]
    fn test_range_sum() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
    }
}
