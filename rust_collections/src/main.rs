use std::collections::HashMap;

fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");
    //push_str for multiple chars, 
    //push for single char
    
    for c in "Зд".chars() {
        println!("{c}");
    }
    
    let hello = "Здравствуйте";
    
    let s = &hello[0..4];
    // this will print Зд
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get("Blue").copied().unwrap_or(0);
    scores.entry(String::from("Yellow")).or_insert(50);
    

    
}
