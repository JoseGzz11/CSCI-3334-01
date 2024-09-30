use std::fs::File;
use std::io::prelude::*;

struct Car {
    make: String,
    model: String,
    year: u16,
}

impl Car {
    fn from_file(path: &str) -> Car {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let make = lines.next().unwrap().to_string();
        let model = lines.next().unwrap().to_string();
        let year = lines.next().unwrap().parse().unwrap();


        Car { make, model, year }
    }
}

fn reading_from_file() {
    let car = Car::from_file("user_info.txt");
    println!("Make: {}", car.make);
    println!("Model: {}", car.model);
    println!("Year: {}", car.year);
}

fn main() {
    reading_from_file();
}