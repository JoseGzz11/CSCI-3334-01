// Declare a constant for the freezing point of water in Fahrenheit (32°F).
const FREEZING_POINT_F: f64 = 32.0;

// Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

// Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    //Declares a mutable variable with a temperature in Fahrenheit
    let _temperature_f: f64 = 32.0;
    //Declare a mutable variable with a temperature in Celcius
    let temperature_c: f64 = 0.0;

    // Converts it to Fahrenheit using your function and prints the result
    let mut temperature_f: f64 = celsius_to_fahrenheit(temperature_c);
    println!("{}°C to {:.2}°F", temperature_c, temperature_f);

    // Converts it to Celsius using your function and prints the result
    let temperature_c: f64 = fahrenheit_to_celsius(temperature_f);
    println!("{}°F to {:.2}°C", temperature_f, temperature_c);

    // Uses a loop to convert and print the next 5 integer temperatures 
    // (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
    for _i in 1..=5 {
        temperature_f += 1.0;
        let temperature_c = fahrenheit_to_celsius(temperature_f);
        println!("{}°F to {:.2}°C", temperature_f, temperature_c);
    }
}