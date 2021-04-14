fn main() {
    // should be 95°F
    let celsius = 35.0;
    // should be 48.8889°C
    let farenheit = 120.0;

    println!("Here is {}F to C = {}", farenheit, convert_degrees(farenheit, "to_celsius"));
    println!("Here is {}C to F = {}", celsius, convert_degrees(celsius, "to_farenheit"));
}

fn convert_degrees(from: f64, direction: &str) -> f64 {
    let mut result = 0.0;

    if direction == "to_celsius" {
       result = (from - 32.0) * 5.0/9.0; 
    } else if direction == "to_farenheit" {
        result = (from / (5.0/9.0)) + 32.0;
    }

    result
}
