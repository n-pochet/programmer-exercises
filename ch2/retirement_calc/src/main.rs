extern crate chrono;

use std::io;
use chrono::prelude::*;

fn main() {
    let mut age = String::new();
    let mut retirement_age = String::new();
    let msg = "Couldn't read from stdin";
    println!("What is your current age?");
    io::stdin().read_line(&mut age).expect(msg);
    println!("At what age would you like to retire?");
    io::stdin().read_line(&mut retirement_age).expect(msg);
    let msg = "Couldn't parse number";
    let age: u32 = age.trim().parse().expect(msg);
    let retirement_age: u32 = retirement_age.trim().parse().expect(msg);
    let years_left: i32 = (retirement_age - age) as i32;
    if years_left < 0 {
        println!("You can already retire");
    } else {
        let now = Local::now();
        let year_now = now.year();
        let year_retirement = year_now + years_left;
        println!(
            "It's {}, so you can retire in {}",
            year_now, year_retirement
        );
    }
}
