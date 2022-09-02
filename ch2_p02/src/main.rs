/*
 * Exercise: 2
 * -----------
 * Reimplement the distance-conversion program from exercise 2 in Chapter 1 so 
 * that it uses a function. In this case, the function must produce both the 
 * number of feet and the number of inches, which means that you need to use 
 * call by reference to return these values.
 * 
 * Write a program that converts a distance in meters to the corresponding 
 * English distance in feet and inches. The conversion factors you need are
 * 1 inch = 0.0254 meters 
 * 1 foot = 12 inches
 */

fn main() {
    let meters = 10.0;
    let mut feet = 0;
    let mut inches = 0.0;

    let (ft, inch) = convert_distance(meters, feet.clone(), inches.clone());

    println!("{meters} meters is equivalent to:");
    println!("{ft} feet and {inch} inches");
}

// TODO: update input parameters to be passed by reference.
fn convert_distance(meters: f32, mut feet: u32, mut inches: f32) -> (u32, f32) {
    let total_in = meters / 0.0254;
    feet = (total_in / 12.0) as u32;
    inches = total_in - (feet * 12) as f32;

    (feet, inches)
 }