use std::io;

fn main() {
    let mut first_number = String::new();
    let mut snd_number = String::new();
    let msg = "Couldn't read from stdin";
    println!("What is the first number? ");
    io::stdin().read_line(&mut first_number).expect(msg);
    println!("What is the second number?");
    io::stdin().read_line(&mut snd_number).expect(msg);
    let first_number: i32 = first_number.trim().parse().expect("Couldn't parse number");
    let snd_number: i32 = snd_number.trim().parse().expect("Couldn't parse number");
    let sum = first_number + snd_number;
    let difference = first_number - snd_number;
    let mul = first_number * snd_number;
    let div = first_number as f32 / snd_number as f32;
    println!(
        "{} + {} = {}\n{} - {} = {}\n{} * {} = {}\n{} / {} = {}",
        first_number,
        snd_number,
        sum,
        first_number,
        snd_number,
        difference,
        first_number,
        snd_number,
        mul,
        first_number,
        snd_number,
        div
    );
}
