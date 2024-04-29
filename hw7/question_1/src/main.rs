enum Shape {
    Triangle { a: f64, b: f64, c: f64 },
    Rectangle { length: f64, width: f64 },
    Circle { radius: f64 },
}

impl Shape {
    fn new_triangle(a: f64, b: f64, c: f64) -> Shape {
        Shape::Triangle { a, b, c }
    }

    fn new_rectangle(length: f64, width: f64) -> Shape {
        Shape::Rectangle { length, width }
    }

    fn new_circle(radius: f64) -> Shape {
        Shape::Circle { radius }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            },
            Shape::Rectangle { length, width } => length * width,
            Shape::Circle { radius } => std::f64::consts::PI * radius.powi(2),
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Triangle { a, b, c } => a + b + c,
            Shape::Rectangle { length, width } => 2.0 * (length + width),
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
        }
    }

    fn double_perimeter(&mut self) {
        match *self {
            Shape::Triangle { ref mut a, ref mut b, ref mut c } => {
                *a *= 2.0;
                *b *= 2.0;
                *c *= 2.0;
            },
            Shape::Rectangle { ref mut length, ref mut width } => {
                *length *= 2.0;
                *width *= 2.0;
            },
            Shape::Circle { ref mut radius } => {
                *radius *= 2.0;
            },
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Shape::Triangle { a, b, c } => *a > 0.0 && *b > 0.0 && *c > 0.0 && *a + *b > *c && *a + *c > *b && *b + *c > *a,
            Shape::Rectangle { length, width } => *length > 0.0 && *width > 0.0,
            Shape::Circle { radius } => *radius > 0.0,
        }
    }
    
}


fn main() {
    let mut triangle = Shape::new_triangle(3.0, 4.0, 5.0);
    let mut rectangle = Shape::new_rectangle(4.0, 5.0);
    let mut circle = Shape::new_circle(5.0);

    println!("Triangle Area: {}", triangle.area());
    println!("Rectangle Area: {}", rectangle.area());
    println!("Circle Area: {}", circle.area());

    println!("Triangle Perimeter: {}", triangle.perimeter());
    println!("Rectangle Perimeter: {}", rectangle.perimeter());
    println!("Circle Perimeter: {}", circle.perimeter());

    println!("Triangle is valid: {}", triangle.is_valid());
    println!("Rectangle is valid: {}", rectangle.is_valid());
    println!("Circle is valid: {}", circle.is_valid());
    
    triangle.double_perimeter();
    rectangle.double_perimeter();
    circle.double_perimeter();

    println!("Triangle New Perimeter: {}", triangle.perimeter());
    println!("Rectangle New Perimeter: {}", rectangle.perimeter());
    println!("Circle New Perimeter: {}", circle.perimeter());


}
