
# Map

![Screenshot_2023-11-24_at_10 38 45_AM](https://github.com/fadhlirahim/rust-programming/assets/4056/781bd4f3-bda3-45b7-89da-9935351dcbcc)

Source: Rails in Action by Tim McNamara


# Variables

All variables are immutable by default.

```rust
let x = 5;
```

To make a variable mutable;

```rust
let mut x = 5;
x = 7;
```

Variable **shadowing**

```rust
let x = 6;
let x = x + 1;
```

**Shadowing is not mutating**. When you shadow a variable, you're essentially creating a new variable that just happens to have the same name. It's like having a new actor take over a role in a play; the character's name stays the same, but it's a different person (or in this case, a different spot in memory).

Memory-wise, here's what happens:

1. **First Declaration**: When you first declare a variable, Rust allocates memory for it.
2. **Shadowing**: If you shadow this variable, Rust allocates new memory for the new variable. The original variable remains untouched.
3. **Access**: When you access the variable after shadowing, Rust refers to the new variable.

Think of it like a game of musical chairs, but instead of taking someone else's seat, every time the music stops, a new chair is added with the same name tag, and the previous chairs remain but are no longer in the spotlight.

# Data Types

## Scalar Type:

- Integer
- Floating point numbers
- Boolean
- Characters

### Integers

Signed  - can be negative numbers and positive numbers.

Each signed variant can store numbers from 

$$
-(2^{N-1}) \text{ to } (2^{N-1}) - 1

$$

inclusive, where `n` is the number of bits that variant uses. 

So an `i8` can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. 

Unsigned - non-negative numbers 0 and positive numbers.

Unsigned variants can store numbers from 0 to 2^N - 1, so a `u8` can store numbers from 0 to 2^8 - 1, which equals 0 to 255.

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| arch | isize | usize |

### Primer on binary numbers (8-bit)

$$
2^0 = 1 \\ 2^1 = 2 \\ 2^3 = 8 \\ 2^4 = 16 \\ 2^5 = 32 \\ 2^6 = 64 \\ 2^7 = 128
$$

To illustrate a binary number of 1001, it’s easier to start for the most left number that will start with 2^0

1001 = (2^3)***1**+(2^2)***0**-+(2^1)***0**+(2^0)***1**

### Floating Points

By default is `f64`

Else is `f32`

### Boolean

Self explanatory

### Characters

Rust’s `char` type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters, Chinese/Japanese/Korean ideographs, emoji, and zero width spaces are all valid `char` types in Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a `char` is in Rust.

## Compound Types

### tuples:

```rust

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### arrays:

Every element of an array must have the same type

```rust
let a = [1, 2, 3, 4, 5];
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust

let a: [i32; 5] = [1, 2, 3, 4, 5];

```

```rust
let a = [3; 5];

```

The array named `a` will contain `5` elements that will all be set to the value `3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a more concise way.

If you know a fix size of an element you want data allocated in a stack, just use an `array`. Else , use `vector` 

```rust
// intentionally inducing a runtime error
let index = 10;
let element = a[index];
println!("the value of element is: {}", element);
```

```rust
❯ cargo build
   Compiling variables v0.1.0 (/Users/fadhli/space/rust-programming/variables)
error: this operation will panic at runtime
  --> src/main.rs:89:19
   |
89 |     let element = a[index];
   |                   ^^^^^^^^ index out of bounds: the length is 5 but the index is 10
   |
   = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `variables` (bin "variables") due to previous error
```

Code repo for variables:

[](https://github.com/fadhlirahim/rust-programming/blob/main/variables/src/main.rs)

# Rust Ownership

Unique feature in Rust, ensuring memory safety without a garbage collector.

Ownership is a set of rules that govern how a Rust program manages memory.

- Memory is managed through a system of ownership with a set of rules that the compiler checks.
- If the rules are violated, the program won’t compile.

## The Stack and Heap

- both the stack and heap are parts of memory available to your code at runtime.
- **Stack**:
    - Last In First Out (like a stack of plates).
    - Adding data - pushing onto the stack.
        - **Faster than allocating because it always pushes to the top of the stack.**
    - Removing data - popping off the stack.
    - Data must be fixed size.
    - Data with an unknown size at compile time or size that might change must be stored on the heap instead.
    - When your code calls a function:
        - the values passed in the function (including pointers to data on the heap)
        - functions local variable gets pushed to the stack
        - when the function is over, those values get popped of the stack.
- **Heap**:
    - Less organized.
    - When you put data on the heap, you request a certain amount of space.
    - Memory allocator (**allocating** on the heap) will:
        - finds an empty spot in the heap that’s big enough.
        - marks it being used.
        - returns a pointer, (address of the location).
    - We store the pointer on the stack - because the pointer of the heap is known, fixed size.
    - When you want the actual data, you follow the pointer.
        - Accessing data on the heap is slower because of this.
    - Analogy is like being seated at a restaurant, when you enter, you state number of people, host finds a table that fits everyone and leads you there. When somebody late comes, they can ask the host where you’re party has been seated to find your party.

**What does rust ownership do?**

- Keeping track of what parts of code that use data on the heap
- reducing dupes on the heap
- cleaning up unused data on the heap so we don’t run out of space are problems

## Ownership Rules

1. Each value in Rust has a variable that’s called its *owner*.
2. Only one owner at a time
3. When the owner goes out of scope, the value is dropped. 

Example

```rust
#![allow(unused_variables)]
fn main() {
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid

```

```rust
#![allow(unused_variables)]
fn main() {
let s1 = String::from("hello");
let s2 = s1;
}
```

![trpl04-01](https://github.com/fadhlirahim/rust-programming/assets/4056/d2b8585d-5043-40ed-8399-d38a2b394b4e)


When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. In other words, the data representation in memory looks like Figure 4-2.

![trpl04-02](https://github.com/fadhlirahim/rust-programming/assets/4056/3bccd66a-25a6-43f5-88bf-3bdddb4b0022)


If we clone the string variables, it will look like this:

![trpl04-03](https://github.com/fadhlirahim/rust-programming/assets/4056/bfdb7e4e-d466-4fb0-ba22-c7043481f715)

but in cases where the value is big, this will be a hit in run-time performance because cloning copies use space in memory stack

### What types implement `Copy` trait?

As a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`. Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## Reference & Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

 Notice:

`&s1`

`&String`

ampersands are ***references -***  they allow you to refer to some value without taking ownership of it.

![trpl04-05](https://github.com/fadhlirahim/rust-programming/assets/4056/4c5c88c7-944d-424f-85f3-993b38855fd6)

### Mutable Reference Restriction

Mutable references have one big restriction: 

- You can only have one mutable reference to a particular piece of data in a particular scope.
- This code will fail:

```rust

let mut s = String::from("hello");
let r1 = &mut s;
// this code will fail
let r2 = &mut s;
```

![Screenshot_2023-12-08_at_10 26 27_AM](https://github.com/fadhlirahim/rust-programming/assets/4056/b9caf84e-27f6-4748-a7ee-6f333ae6f6f1)

### Why?

To prevent data races.

### Important Concepts

1. cannot have a mutable reference while we have an immutable one

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

### Why?

Users of an immutable reference don’t expect the values to suddenly change out from under them

1. Dangling reference

In Rust, the compiler guarantees that **references will never be dangling references**: if we have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

### **Slices are References**

Remember, slices don't own the data. They're just a reference to a portion of an array or vector. If **`numbers`** changes or goes out of scope, **`slice`** will no longer be valid.

```rust
#![allow(unused_variables)]
fn main() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
}

```

![trpl04-06](https://github.com/fadhlirahim/rust-programming/assets/4056/7ac513f0-7dcf-41e6-8f65-f86539ff50cd)

## Struct

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl is implemenation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle { width: 30, height: 50 };
        
    println!("rect1 is {:?}", rec);
    println!("rec.width = {}", rec.width);
    println!("rec.height = {}", rec.height);
    println!("area = {}", Rectangle::area(&rec));
}

```

## Enums

```rust
#![allow(unused_variables)]
fn main() {
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
}
```

We can also declare the type of enums we want in our enum block

```rust
#![allow(unused_variables)]
fn main() {
  enum IpAddr {
    V4(String),
    V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));

  let loopback = IpAddr::V6(String::from("::1"));
}
```

This struct is also acceptable as enum values:

```rust
#![allow(unused_variables)]
fn main() {
  struct Ipv4Addr {
    // details elided
  }

  struct Ipv6Addr {
    // details elided
  }

  enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
  }
}
```

