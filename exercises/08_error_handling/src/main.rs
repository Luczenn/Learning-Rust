// Exercise 08: Error Handling
// Based on: https://doc.rust-lang.org/rust-by-example/error.html
//
// Topics covered:
//   - panic! for unrecoverable errors
//   - Option<T> for nullable values
//   - Result<T, E> for recoverable errors
//   - The `?` operator for error propagation
//   - Combining Option and Result

use std::num::ParseIntError;

// --- Option<T> ---
// Option represents either Some(value) or None (the absence of a value).

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

// --- Result<T, E> ---
// Result represents either Ok(value) or Err(error).

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?; // ? propagates the error if parsing fails
    Ok(n * 2)
}

// Custom error type
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i32),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::NegativeNumber(n) => write!(f, "Negative number: {}", n),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.trim().parse()?; // ParseIntError → AppError via From impl
    if n < 0 {
        Err(AppError::NegativeNumber(n))
    } else {
        Ok(n as u32)
    }
}

fn main() {
    // --- panic! ---
    // Use only for truly unrecoverable errors (bugs, not expected failure paths).
    println!("=== panic! ===");
    // Uncomment to see a panic:
    // panic!("something went very wrong!");
    println!("(panic! is commented out – uncomment it to see a crash)\n");

    // --- Option ---
    println!("=== Option<T> ===\n");

    let result = divide(10.0, 2.0);
    println!("10.0 / 2.0 = {:?}", result);

    let bad = divide(5.0, 0.0);
    println!("5.0 / 0.0  = {:?}", bad);

    // Unwrap methods
    println!("unwrap()         = {}", divide(9.0, 3.0).unwrap());
    println!("unwrap_or(0)     = {}", divide(1.0, 0.0).unwrap_or(0.0));
    println!("unwrap_or_else   = {}", divide(1.0, 0.0).unwrap_or_else(|| f64::INFINITY));

    // if let (clean single-variant matching)
    if let Some(val) = divide(20.0, 4.0) {
        println!("20.0 / 4.0 = {}", val);
    }

    // Option chaining with map and and_then
    let doubled = divide(10.0, 2.0).map(|v| v * 2.0);
    println!("(10/2) * 2 via map: {:?}", doubled);

    let numbers = vec![1, 3, 7, 4, 9];
    match find_first_even(&numbers) {
        Some(n) => println!("First even in {:?}: {}", numbers, n),
        None    => println!("No even numbers in {:?}", numbers),
    }

    // --- Result ---
    println!("\n=== Result<T, E> ===\n");

    println!("parse_and_double(\"21\")  = {:?}", parse_and_double("21"));
    println!("parse_and_double(\"abc\") = {:?}", parse_and_double("abc"));
    println!("parse_and_double(\" 5\") = {:?}", parse_and_double(" 5"));

    // match on Result
    match parse_and_double("7") {
        Ok(n)  => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // unwrap variants
    println!("unwrap_or(-1) on err: {}", parse_and_double("bad").unwrap_or(-1));

    // map and and_then on Result
    let result: Result<String, _> = parse_and_double("5").map(|n| format!("value is {}", n));
    println!("map to string: {:?}", result);

    // --- Custom error type ---
    println!("\n=== Custom Error Type ===\n");

    println!("parse_positive(\"42\")  = {:?}", parse_positive("42"));
    println!("parse_positive(\"-3\")  = {:?}", parse_positive("-3"));
    println!("parse_positive(\"abc\") = {:?}", parse_positive("abc"));

    // --- The ? operator in a closure / function ---
    // Collect multiple results
    let inputs = vec!["1", "2", "three", "4"];
    let parsed: Vec<Result<i32, _>> = inputs.iter().map(|s| s.parse::<i32>()).collect();
    for (s, r) in inputs.iter().zip(parsed.iter()) {
        println!("  parse('{}') = {:?}", s, r);
    }

    // --- Exercise: Try it yourself! ---
    // 1. Write `safe_sqrt(x: f64) -> Option<f64>` returning None for negative x.
    // 2. Write a function that reads a comma-separated list of integers from a string,
    //    returning Result<Vec<i32>, ParseIntError>.
    // 3. Add an `EmptyInput` variant to AppError and handle an empty string in parse_positive.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // fn safe_sqrt(x: f64) -> Option<f64> { ... }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_some() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
    }

    #[test]
    fn test_divide_none() {
        assert_eq!(divide(5.0, 0.0), None);
    }

    #[test]
    fn test_parse_and_double_ok() {
        assert_eq!(parse_and_double("21"), Ok(42));
    }

    #[test]
    fn test_parse_and_double_err() {
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_parse_positive_ok() {
        assert_eq!(parse_positive("10").unwrap(), 10u32);
    }

    #[test]
    fn test_parse_positive_negative() {
        matches!(parse_positive("-1").unwrap_err(), AppError::NegativeNumber(-1));
    }

    #[test]
    fn test_find_first_even() {
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
    }
}
