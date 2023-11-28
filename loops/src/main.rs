fn main() {
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
