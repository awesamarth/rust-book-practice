fn main() {
    another_function(2);
    let x = plus_one(3);
    println!("the value of x is: {x}");
}

fn another_function(x:i32){
    println!("The value of x is: {x}");
}

fn plus_one(x:i32)->i32{
    x + 1
}