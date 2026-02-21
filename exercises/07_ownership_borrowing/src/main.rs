// Exercise 07: Ownership and Borrowing
// Based on: https://doc.rust-lang.org/rust-by-example/scope.html
//
// Topics covered:
//   - Ownership rules
//   - Move semantics vs. Copy types
//   - Borrowing: shared (&T) and mutable (&mut T) references
//   - Slices
//   - Lifetimes (basic introduction)

// --- Demonstrating ownership with a helper ---
fn takes_ownership(s: String) {
    println!("  Inside takes_ownership: '{}'", s);
} // s is dropped here

fn makes_copy(n: i32) {
    println!("  Inside makes_copy: {}", n);
} // n is copied in, nothing special happens

fn gives_ownership() -> String {
    String::from("hello from gives_ownership")
}

// --- Borrowing ---
// Shared (immutable) reference: &T
fn calculate_length(s: &String) -> usize {
    s.len()
} // s is a reference; the original is not dropped

// Mutable reference: &mut T
fn change(s: &mut String) {
    s.push_str(", world");
}

// --- Slices ---
// A slice is a reference to a contiguous part of a collection.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s // the whole string is one word
}

// Lifetime annotation: tells the compiler the returned reference lives at
// least as long as the shorter-lived of `x` and `y`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("=== Ownership ===\n");

    // --- Move semantics ---
    // When a heap-allocated value is assigned to another variable, ownership moves.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2; s1 is no longer valid
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("s2 (moved from s1): {}", s2);

    // To keep both, use clone() (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4 (cloned): {}", s3, s4);

    // --- Copy types ---
    // Scalar types (integers, floats, bool, char) implement Copy.
    // They are copied, not moved.
    let x = 5;
    let y = x; // x is copied
    println!("x: {}, y (copied): {}", x, y);

    // --- Ownership and functions ---
    println!("\n--- Ownership and functions ---");
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s was moved into takes_ownership

    let n = 42;
    makes_copy(n);
    println!("n is still valid: {}", n); // Copy types are unaffected

    // Return values can transfer ownership
    let returned = gives_ownership();
    println!("Returned string: {}", returned);

    // --- Borrowing ---
    println!("\n=== Borrowing ===\n");

    let s5 = String::from("hello, rust");
    let len = calculate_length(&s5); // pass a reference; s5 is not moved
    println!("'{}' has {} characters", s5, len);

    // Multiple immutable references are allowed simultaneously
    let r1 = &s5;
    let r2 = &s5;
    println!("r1: {}, r2: {}", r1, r2);

    // Mutable reference: only one at a time, and no immutable refs must exist
    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("After change: {}", s6);

    // --- Slices ---
    println!("\n=== Slices ===\n");

    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word of '{}': '{}'", sentence, word);

    // Integer slice
    let numbers = [10, 20, 30, 40, 50];
    let slice = &numbers[1..4]; // [20, 30, 40]
    println!("Array slice [1..4]: {:?}", slice);

    // String slices
    let s = "hello, world";
    println!("Full string: {}", s);
    println!("First 5 chars: {}", &s[..5]);
    println!("Last 5 chars:  {}", &s[7..]);

    // --- Lifetimes ---
    println!("\n=== Lifetimes ===\n");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Longest: '{}'", result);
    }

    // --- Exercise: Try it yourself! ---
    // 1. Write a function that takes a &Vec<i32> and returns the largest element.
    // 2. Demonstrate that you cannot have a mutable and immutable reference at the same time.
    //    (Uncomment and observe the error.)
    // 3. Write a function returning the last word in a sentence using slices.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s; // ERROR: cannot borrow as mutable because it is also borrowed as immutable
    // println!("{}, {}", r1, r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("oneword"), "oneword");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_longest() {
        let s1 = String::from("long string");
        let s2 = String::from("short");
        assert_eq!(longest(&s1, &s2), "long string");
        assert_eq!(longest("abc", "abcd"), "abcd");
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        // s is still usable after the call
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_change() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }
}
