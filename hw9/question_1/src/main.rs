use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn generate_data(filename: &str, num_points: usize) {
    let mut file = File::create(filename).expect("Unable to create file");
    let mut rng = rand::thread_rng();
    for _ in 0..num_points {
        let x: i32 = rng.gen_range(-100_000_000..=100_000_000);
        let z: u8 = rng.gen_range(0..=1);
        writeln!(file, "{} {}", x, z).expect("Unable to write to file");
    }
}

fn read_data(filename: &str) -> Vec<(i32, u8)> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut points: Vec<(i32, u8)> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[0].parse::<i32>().unwrap();
        let z = parts[1].parse::<u8>().unwrap();
        points.push((x, z));
    }
    points
}

fn find_best_threshold(points: &[(i32, u8)]) -> (i32, f64) {
    let mut best_accuracy = 0.0;
    let mut best_threshold = 0;
    for &(x, _) in points {
        let mut correctly_classified = 0;
        for &(point_x, point_z) in points {
            let prediction = if point_x >= x { 1 } else { 0 };
            if prediction == point_z {
                correctly_classified += 1;
            }
        }
        let accuracy = correctly_classified as f64 / points.len() as f64;
        if accuracy > best_accuracy {
            best_accuracy = accuracy;
            best_threshold = x;
        }
    }
    (best_threshold, best_accuracy)
}


fn main() {
    let filename = "data.txt";
    let num_points = 500;

    generate_data(filename, num_points);
    let points = read_data(filename);
    
    let (best_threshold, accuracy) = find_best_threshold(&points);

    println!("if x >= {}\nPredicted label is 1\nelse\nPredicted label is 0", best_threshold);
    println!("accuracy: {:.2}", accuracy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_threshold() {
        let points = vec![
            (-30, 1), 
            (-20, 1), 
            (10, 0),  
            (20, 0), 
            (0, 1),   
        ];

        let (threshold, accuracy) = find_best_threshold(&points);

        
        let expected_threshold = -30; 
        let expected_accuracy = 0.6; 

        assert_eq!(threshold, expected_threshold, "Threshold did not match the expected value.");
        assert!((accuracy - expected_accuracy).abs() < f64::EPSILON, "Accuracy did not match the expected value. Expected: {}, Got: {}", expected_accuracy, accuracy);
    }
}

