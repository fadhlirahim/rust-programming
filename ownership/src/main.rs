fn main() {

    // Using mutable, growable piece of text
    let s1 = String::from("hello"); 
    let s2 = &s1; // no problem with immutable reference
    // let s3 = &mut s1; // big  problem

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    let hello = String::from("bye, hello");
    let first_word = first_word(&hello);
    println!("first_word = {}", first_word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


