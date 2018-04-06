use std::io;

fn main() {
    let mut people = String::new();
    let mut pizzas = String::new();
    let mut slices = String::new();
    println!("How many people?");
    io::stdin().read_line(&mut people).unwrap();
    println!("How many pizzas do you have?");
    io::stdin().read_line(&mut pizzas).unwrap();
    println!("How many slices per pizza?");
    io::stdin().read_line(&mut slices).unwrap();
    let people: u32 = people.trim().parse().unwrap();
    let pizzas: u32 = pizzas.trim().parse().unwrap();
    let slices: u32 = slices.trim().parse().unwrap();
    let tot_slices = slices * pizzas;
    let sl_per_person = tot_slices / people;
    let remainder = tot_slices % people;
    println!("{} people with {} pizzas", people, pizzas);
    println!("Each people gets {} pieces of pizza.", sl_per_person);
    println!("There are {} leftover pieces.", remainder);
}
