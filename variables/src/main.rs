fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("the length of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("the value of guess is: {}", guess);

    let x = 2.0;
    let y: f32 = 3.1;

    println!("x + y = {}", x + y);
}


