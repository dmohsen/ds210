pub struct Polygon {
    sides: usize,
    side_length: f64,
}

pub trait PolygonProperties {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
}

impl PolygonProperties for Polygon {
    fn perimeter(&self) -> f64 {
        self.sides as f64 * self.side_length
    }

    fn area(&self) -> f64 {
        self.perimeter() * self.apothem() / 2.0
    }

    fn radius(&self) -> f64 {
        self.side_length / (2.0 * (std::f64::consts::PI / self.sides as f64).sin())
    }

    fn apothem(&self) -> f64 {
        self.radius() * (std::f64::consts::PI / self.sides as f64).cos()
    }
}

fn main() {
    let sides_array = [6, 12, 24, 128, 256, 512, 1024, 2048, 65536];
    let radius_lengths = [1.0, 2.0, 3.0]; 

    for &radius in &radius_lengths {
        println!("Radius: {}", radius);
        for &sides in &sides_array {
            let side_length = 2.0 * radius * (std::f64::consts::PI / sides as f64).sin();
            let polygon = Polygon { sides, side_length };
            let polygon_area = polygon.area();
            let circle_area = std::f64::consts::PI * radius.powi(2);

            println!("Sides: {}, Polygon Area: {:.4}, Circle Area: {:.4}", sides, polygon_area, circle_area);
        }
        println!("----------------------------------------");
    }
}
