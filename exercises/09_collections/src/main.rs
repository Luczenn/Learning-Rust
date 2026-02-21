// Exercise 09: Collections
// Based on: https://doc.rust-lang.org/rust-by-example/std.html
//
// Topics covered:
//   - Vec<T>   – growable array
//   - String   – owned UTF-8 text
//   - HashMap<K, V> – key-value store
//   - HashSet<T>    – unique values
//   - Common iterator patterns

use std::collections::{HashMap, HashSet};

fn main() {
    // =========================================================
    // Vec<T> – growable, heap-allocated array
    // =========================================================
    println!("=== Vec<T> ===\n");

    // Create with Vec::new() or the vec! macro
    let mut nums: Vec<i32> = Vec::new();
    nums.push(3);
    nums.push(1);
    nums.push(4);
    nums.push(1);
    nums.push(5);
    println!("nums:         {:?}", nums);
    println!("length:       {}", nums.len());
    println!("first:        {:?}", nums.first());
    println!("last:         {:?}", nums.last());

    // Indexing (panics if out of bounds) vs. get (returns Option)
    println!("nums[2]:      {}", nums[2]);
    println!("nums.get(10): {:?}", nums.get(10));

    // Iteration
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    println!("doubled:      {:?}", doubled);

    let sum: i32 = nums.iter().sum();
    println!("sum:          {}", sum);

    // Sort and dedup
    nums.sort();
    nums.dedup();
    println!("sorted+dedup: {:?}", nums);

    // Remove an element by index
    let removed = nums.remove(0);
    println!("removed [0]:  {} → {:?}", removed, nums);

    // Retain only elements satisfying a predicate
    let mut evens = vec![1, 2, 3, 4, 5, 6];
    evens.retain(|&x| x % 2 == 0);
    println!("retain evens: {:?}", evens);

    // =========================================================
    // String – owned, growable UTF-8 text
    // =========================================================
    println!("\n=== String ===\n");

    let mut greeting = String::from("Hello");
    greeting.push_str(", world"); // append a &str
    greeting.push('!');           // append a single char
    println!("greeting:     {}", greeting);
    println!("length:       {}", greeting.len());
    println!("uppercase:    {}", greeting.to_uppercase());
    println!("contains 'wo': {}", greeting.contains("wo"));
    println!("replace:      {}", greeting.replace("world", "Rust"));

    // String slices
    let s = String::from("Hello, Rust!");
    println!("slice [7..11]: {}", &s[7..11]);

    // Split and collect
    let csv = "one,two,three,four";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split csv:    {:?}", parts);

    // Join
    let joined = parts.join(" | ");
    println!("joined:       {}", joined);

    // String formatting
    let name = "Alice";
    let age: u32 = 30;
    let bio = format!("{} is {} years old", name, age);
    println!("format!:      {}", bio);

    // =========================================================
    // HashMap<K, V>
    // =========================================================
    println!("\n=== HashMap<K, V> ===\n");

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    println!("scores:       {:?}", scores);
    println!("Alice:        {:?}", scores.get("Alice"));
    println!("Dave:         {:?}", scores.get("Dave")); // None

    // entry API: insert only if the key does not exist
    scores.entry(String::from("Dave")).or_insert(75);
    scores.entry(String::from("Alice")).or_insert(0); // Alice already exists
    println!("after entry:  {:?}", scores);

    // Update based on old value
    let alice_score = scores.entry(String::from("Alice")).or_insert(0);
    *alice_score += 5; // dereference to mutate
    println!("Alice +5:     {}", scores["Alice"]);

    // Iterate (order is not guaranteed)
    let mut names: Vec<&String> = scores.keys().collect();
    names.sort(); // sort for deterministic output
    for name in &names {
        println!("  {} → {}", name, scores[*name]);
    }

    // Remove
    scores.remove("Bob");
    println!("after remove: {:?}", scores.contains_key("Bob"));

    // Build from an iterator
    let teams = vec!["Red", "Blue", "Green"];
    let initial_scores = vec![10u32, 20, 30];
    let map: HashMap<&str, u32> = teams.into_iter().zip(initial_scores).collect();
    println!("built map:    {:?}", map);

    // =========================================================
    // HashSet<T>
    // =========================================================
    println!("\n=== HashSet<T> ===\n");

    let mut set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();

    println!("set_a:        {:?}", {
        let mut v: Vec<_> = set_a.iter().collect();
        v.sort();
        v
    });

    // Set operations
    let mut union: Vec<i32> = set_a.union(&set_b).cloned().collect();
    union.sort();
    println!("union:        {:?}", union);

    let mut inter: Vec<i32> = set_a.intersection(&set_b).cloned().collect();
    inter.sort();
    println!("intersection: {:?}", inter);

    let mut diff: Vec<i32> = set_a.difference(&set_b).cloned().collect();
    diff.sort();
    println!("a - b:        {:?}", diff);

    set_a.insert(10);
    set_a.remove(&1);
    println!("contains 3:   {}", set_a.contains(&3));
    println!("contains 1:   {}", set_a.contains(&1));

    // =========================================================
    // Word count example combining all collections
    // =========================================================
    println!("\n=== Word Count Example ===\n");

    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut word_pairs: Vec<(&&str, &u32)> = word_count.iter().collect();
    word_pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0))); // by count desc, then alpha
    for (word, count) in &word_pairs {
        println!("  {:10} : {}", word, count);
    }

    // --- Exercise: Try it yourself! ---
    // 1. Read a Vec of strings and count how many start with each letter (HashMap).
    // 2. Find duplicate numbers in a Vec using a HashSet.
    // 3. Implement a simple stack using Vec with push/pop/peek operations.
    println!("\n--- Your turn! Uncomment the exercises below ---");
    // let stack: Vec<i32> = vec![];
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_vec_operations() {
        let mut v = vec![3, 1, 4, 1, 5];
        v.sort();
        v.dedup();
        assert_eq!(v, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_vec_retain() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_string_operations() {
        let mut s = String::from("Hello");
        s.push_str(", Rust");
        assert_eq!(s, "Hello, Rust");
        assert!(s.contains("Rust"));
        assert_eq!(s.replace("Rust", "World"), "Hello, World");
    }

    #[test]
    fn test_hashmap_entry() {
        let mut map: HashMap<&str, u32> = HashMap::new();
        *map.entry("a").or_insert(0) += 1;
        *map.entry("a").or_insert(0) += 1;
        *map.entry("b").or_insert(0) += 1;
        assert_eq!(map["a"], 2);
        assert_eq!(map["b"], 1);
    }

    #[test]
    fn test_hashset_intersection() {
        let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
        let mut inter: Vec<i32> = a.intersection(&b).cloned().collect();
        inter.sort();
        assert_eq!(inter, vec![2, 3]);
    }

    #[test]
    fn test_word_count() {
        let text = "a b a c a b";
        let mut counts: HashMap<&str, u32> = HashMap::new();
        for word in text.split_whitespace() {
            *counts.entry(word).or_insert(0) += 1;
        }
        assert_eq!(counts["a"], 3);
        assert_eq!(counts["b"], 2);
        assert_eq!(counts["c"], 1);
    }
}
