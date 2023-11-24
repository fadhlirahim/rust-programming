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
    
    // floating point
    let x = 2.0;
    let y: f32 = 3.1;

    println!("x + y = {}", x + y);


    // Numeric Operations
    
    // additions
    let sum = 5 + 10;
    println!("sum is: {}", sum);

    // subtractions
    let difference = 95.5 - 4.3;
    println!("difference is: {}", difference);

    // multiplications
    let product = 4 * 30;
    println!("product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is: {}", remainder);


    // boolean
    let t = true;
    println!("{}", t);

    let f: bool = false;
    println!("f: {}", f);

    // Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
}


