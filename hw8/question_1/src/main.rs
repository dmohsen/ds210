#[derive(Debug, PartialEq)]

struct Point<T> {
    x: T,
    y: T,
}

use std::ops::Neg;

impl<T: Copy + Neg<Output = T>> Point<T> {
    fn clockwise(&self) -> Self {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    fn counterclockwise(&self) -> Self {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
}

fn main() {
    let point_f64 = Point { x: 3.0, y: 4.0 };
    let rotated_clockwise_f64 = point_f64.clockwise();
    println!("Point f64 rotated clockwise: ({}, {})", rotated_clockwise_f64.x, rotated_clockwise_f64.y);
    let rotated_counterclockwise_f64 = rotated_clockwise_f64.counterclockwise();
    println!("Point f64 rotated counterclockwise: ({}, {})", rotated_counterclockwise_f64.x, rotated_counterclockwise_f64.y);

    let point_i32 = Point { x: 5, y: 6 };
    let rotated_counterclockwise_i32 = point_i32.counterclockwise();
    println!("Point i32 rotated counterclockwise: ({}, {})", rotated_counterclockwise_i32.x, rotated_counterclockwise_i32.y);
    let rotated_clockwise_i32 = rotated_counterclockwise_i32.clockwise();
    println!("Point i32 rotated clockwise: ({}, {})", rotated_clockwise_i32.x, rotated_clockwise_i32.y);

    assert_eq!(rotated_clockwise_f64, Point { x: 4.0, y: -3.0 }, "Clockwise rotation for f64 failed");
    assert_eq!(rotated_counterclockwise_f64, point_f64, "Counterclockwise rotation back to original for f64 failed");

    assert_eq!(rotated_counterclockwise_i32, Point { x: -6, y: 5 }, "Counterclockwise rotation for i32 failed");
    assert_eq!(rotated_clockwise_i32, point_i32, "Clockwise rotation back to original for i32 failed");
}
