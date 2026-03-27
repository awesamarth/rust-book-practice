fn main() {
    let mut s = String::from("hello");
    
    let s1 = &mut s;
    let s2 = &mut s;
    //won't work
    println!("{s1} {s2}")
    
    
}
