use std::io;

fn main() {
    let mut quote = String::new();
    let mut author = String::new();
    let msg = "Couldn't read from stdin";
    println!("What is the quote?");
    io::stdin().read_line(&mut quote).expect(msg);
    let trimmed_quote = quote.trim();
    println!("Who said it?");
    io::stdin().read_line(&mut author).expect(msg);
    let trimmed_author = author.trim();
    println!("{} says, \"{}\"", trimmed_author, trimmed_quote);
}
