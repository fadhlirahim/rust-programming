fn main() {

    // Using mutable, growable piece of text
    let s1 = String::from("hello"); 
    let s2 = s1; 

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
