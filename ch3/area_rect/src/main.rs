use std::io;

fn main() {
    let mut length = String::new();
    let mut width = String::new();
    println!("What is the length of the room in feet?");
    io::stdin().read_line(&mut length).unwrap();
    println!("What is the width of the room in feet?");
    io::stdin().read_line(&mut width).unwrap();
    let length: u32 = length.trim().parse().unwrap();
    let width: u32 = width.trim().parse().unwrap();
    let area_feet = length * width;
    let area_meters = area_feet as f32 * 0.09290304;
    println!("You entered dimension of {} by {} feet.", length, width);
    println!(
        "The area is\n{} square feet\n{} square meters",
        area_feet, area_meters
    );
}
