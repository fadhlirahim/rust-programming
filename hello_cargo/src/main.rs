fn main() {
    println!("Hello, world!");

    another_function(42);

    another_function2(32, 64);
    
    let x = 5;

    println!("the value initial x: {}", x);

    let y = {
        let x = 3; // shodowing x
        println!("the value of x in the block: {}", x);
        x + 1 // expression, no semicolon in the end.
    };

    println!("the value of y is: {}", y);

    let x = five();
    println!("the value of x is: {}", x);

    let y = plus_one(x);
    println!("the value of y is: {}", y);

    // Conditionals
    if y < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number == 3 {
        println!("number match three");
    }

    let num = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number condition value is 5?: {}", number);

//     loop {
//         println!("again!");
//     }

    // while loop 
    let mut num = 3;

    while num != 0 {
        println!("the value of num is: {}", num);
        num -= 1;
    }

    println!("LIFTOFF!!!!");

    // Looping through a collection with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // this approach is error prone
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // much more elegant way to loop through a collection
    // especially if we don't know the size of the index.
    for el in a.iter() {
        println!("the value is: {}", el);
    }

    // using range
    for number in 1..a.len(){
        println!("the value is: {}", number);
    }
    // using range and reverse it
    for number in (1..4).rev() {
        println!("the value is: {}", number);
    }

    println!("LIFTOFF!!!!");
}

fn another_function(x: i32) {
    println!("the value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

