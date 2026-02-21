// Exercise 02: Primitives
// Based on: https://doc.rust-lang.org/rust-by-example/primitives.html
//
// Topics covered:
//   - Scalar types: integers, floats, booleans, characters
//   - Compound types: tuples, arrays
//   - Type inference and explicit annotations
//   - Type casting with `as`

fn main() {
    // --- Integer types ---
    // Signed:   i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer:   {}", x);
    println!("Unsigned integer: {}", y);

    // Integer literals can use _ as a visual separator
    let one_million = 1_000_000u32;
    println!("One million: {}", one_million);

    // --- Floating-point types ---
    // f32 (single precision) and f64 (double precision, default)
    let pi: f64 = 3.141_592_653_589_793;
    let e: f32 = 2.718_28;
    println!("Pi:         {:.6}", pi);
    println!("Euler's e:  {:.5}", e);

    // --- Boolean ---
    let is_rust_fun: bool = true;
    let is_hard: bool = false;
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is Rust hard? {}", is_hard);
    println!("Fun AND hard: {}", is_rust_fun && is_hard);

    // --- Characters (Unicode scalar values, 4 bytes each) ---
    let letter: char = 'R';
    let emoji: char = '🦀'; // Rust's mascot, Ferris the crab!
    println!("Letter: {}, Emoji: {}", letter, emoji);

    // --- Tuples ---
    // Fixed-size, can contain different types.
    let tup: (i32, f64, bool, char) = (500, 6.4, true, 'z');
    // Destructure with pattern matching
    let (a, b, c, d) = tup;
    println!("Tuple values: {}, {}, {}, {}", a, b, c, d);
    // Access by index
    println!("First element: {}", tup.0);

    // Unit tuple () – represents an empty value (similar to void)
    let unit: () = ();
    println!("Unit value: {:?}", unit);

    // --- Arrays ---
    // Fixed-size, all elements must be the same type.
    let primes: [i32; 5] = [2, 3, 5, 7, 11];
    println!("Primes:         {:?}", primes);
    println!("Third prime:    {}", primes[2]);
    println!("Array length:   {}", primes.len());

    // Initialise an array with the same value: [value; count]
    let zeros = [0u8; 8];
    println!("Zeros:          {:?}", zeros);

    // --- Type casting with `as` ---
    let big: i32 = 1000;
    let small = big as u8; // truncates: 1000 % 256 = 232
    println!("1000 as u8:  {}", small);
    let float_val: f64 = 9.9;
    let int_val = float_val as i32; // truncates towards zero
    println!("9.9 as i32:  {}", int_val);

    // --- Exercise: Try it yourself! ---
    // 1. Declare a tuple (name_length, first_char) for the string "Rust".
    // 2. Create an array of your 5 favourite numbers and print their sum.
    // 3. Cast -1i8 to u8 and explain the result.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // let my_tuple = (4usize, 'R');
    // println!("{:?}", my_tuple);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_cast() {
        let big: i32 = 1000;
        let small = big as u8;
        assert_eq!(small, 232); // 1000 mod 256
    }

    #[test]
    fn test_float_to_int_truncation() {
        let f: f64 = 9.9;
        assert_eq!(f as i32, 9);
    }

    #[test]
    fn test_tuple_destructure() {
        let tup = (1, 2.0, true);
        let (a, b, c) = tup;
        assert_eq!(a, 1);
        assert_eq!(b, 2.0);
        assert!(c);
    }

    #[test]
    fn test_array_length() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
    }
}
