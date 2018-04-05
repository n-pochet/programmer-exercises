use std::io;

fn main() {
    let mut buffer = String::new();
    println!("What is the input string?");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");
    let trimmed_input = buffer.trim();
    let count = trimmed_input.len();
    println!("{} has {} characters.", trimmed_input, count);
}
