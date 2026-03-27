fn main() {
    println!("Hello, world!");

    let sx = String::from("hello"); // s comes into scope

    takes_ownership(sx); // s's value moves into the function...

    let x = 5; // x comes into scope

    makes_copy(x);

    let mut s = String::from("hello");

    let s1 = &mut s;
    // let s2 = &mut s;
    //won't work
    // println!("{s1} {s2}");
    println!("{s1}");

    // slices
    let sxa = String::from("hello world");

    let hello = &sxa[0..5];
    let world = &sxa[6..11];

    println!("{hello} {world}");

    let sm = String::from("hello");

    let len = sm.len();

    let slice = &sm[0..len];
    println!("{slice}");

    let slice = &sm[..];
    println!("{slice}");
    
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
    // this will be equal
    
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
