use std::io;

fn main() {
    let mut name = String::new();
    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Couldn't read from stdin");
    let name = name.trim();
    println!("Hello, {}, nice to meet you!", name);
}
