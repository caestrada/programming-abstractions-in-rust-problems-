/// File: Problem 1
/// ---------------
/// If you did not do so the first time around, rewrite the Celsius-to-Fahrenheit 
/// program from exercise 1 in Chapter 1 so that it uses a function to perform 
/// the conversion.
/// 
/// Write a program that reads in a temperature in degrees Celsius and displays 
/// the corresponding temperature in degrees Fahrenheit. The conversion formula 
/// is:
/// F = (9/5)C + 32
fn main() {
    let celsius = 32.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("Convert {celsius} to Fahrenheit {fahrenheit}");
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    let f = (c * 9.0/5.0) + 32.0;
    
    f
}
