use std::io;

fn main() {
    println!("Please input in Celcius.");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse() 
        .expect("Error parsing input");

    let farenheit = input * 9/5 + 32;
    println!("{} Celcius is {} Farenheit", input, farenheit);
}
