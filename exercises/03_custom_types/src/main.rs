// Exercise 03: Custom Types
// Based on: https://doc.rust-lang.org/rust-by-example/custom_types.html
//
// Topics covered:
//   - Structs (unit, tuple, named-field)
//   - Enums (C-style, data-carrying)
//   - The `impl` block for methods
//   - Pattern matching on enums

// --- Structs ---

// Unit struct (no fields)
struct Unit;

// Tuple struct
struct Color(u8, u8, u8);

// Named-field struct
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function (like a constructor)
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Method – takes &self
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// --- Enums ---

// Simple C-style enum
#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data (algebraic data type)
#[derive(Debug)]
enum Shape {
    Circle(f64),           // radius
    Rectangle(f64, f64),   // width, height
    Triangle(f64, f64, f64), // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle with radius {:.2}", r),
            Shape::Rectangle(w, h) => format!("Rectangle {}x{}", w, h),
            Shape::Triangle(a, b, c) => format!("Triangle ({}, {}, {})", a, b, c),
        }
    }
}

fn print_direction(dir: &Direction) {
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
    }
}

fn main() {
    // --- Unit struct ---
    let _unit = Unit; // used by name, no fields

    // --- Tuple struct ---
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    println!("Red:   RGB({}, {}, {})", red.0, red.1, red.2);
    println!("Green: RGB({}, {}, {})", green.0, green.1, green.2);

    // --- Named-field struct ---
    let origin = Point::new(0.0, 0.0);
    let p = Point::new(3.0, 4.0);
    println!("Origin: {:?}", origin);
    println!("Point:  {:?}", p);
    println!("Distance from origin: {}", p.distance_from_origin());

    // Struct update syntax: create a new struct based on an existing one
    let p2 = Point { x: 1.0, ..p };
    println!("p2: {:?}", p2);

    // --- Enums ---
    let dir = Direction::North;
    println!("\nDirection: {:?}", dir);
    print_direction(&dir);

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    println!();
    for shape in &shapes {
        println!("{} => area = {:.2}", shape.describe(), shape.area());
    }

    // --- Exercise: Try it yourself! ---
    // 1. Add a `perimeter` method to Shape.
    // 2. Create a struct `Rectangle` with methods `area` and `is_square`.
    // 3. Add a `Diagonal` variant to Direction and update `print_direction`.
    println!("\n--- Your turn! Add your own structs and enums above ---");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_circle_area() {
        let c = Shape::Circle(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Shape::Rectangle(4.0, 6.0);
        assert_eq!(r.area(), 24.0);
    }

    #[test]
    fn test_color_fields() {
        let c = Color(10, 20, 30);
        assert_eq!(c.0, 10);
        assert_eq!(c.1, 20);
        assert_eq!(c.2, 30);
    }
}
