use std::io;

fn main() {
    println!("Please input in Celcius.");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse() 
        .expect("Error parsing input");

    let farenheit = celsius_to_farenheit(input as f32);
    println!("{} Celcius is {} Farenheit", input, farenheit);
}

fn celsius_to_farenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

