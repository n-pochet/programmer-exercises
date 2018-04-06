use std::io;

fn main() {
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adjective = String::new();
    let mut adverb = String::new();
    let msg = "Couldn't read from stdin";
    println!("Enter a noun: ");
    io::stdin().read_line(&mut noun).expect(msg);
    println!("Enter a verb: ");
    io::stdin().read_line(&mut verb).expect(msg);
    println!("Enter an adjective: ");
    io::stdin().read_line(&mut adjective).expect(msg);
    println!("Enter an adverb : ");
    io::stdin().read_line(&mut adverb).expect(msg);
    let noun = noun.trim();
    let verb = verb.trim();
    let adjective = adjective.trim();
    let adverb = adverb.trim();
    println!(
        "Do you {} your {} {} {}? That's hilarious",
        verb, adjective, noun, adverb
    );
}
