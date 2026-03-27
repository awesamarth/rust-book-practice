fn main() {
    const SOME_CONSTANT: u32 = 60 * 60;
    let mut x = 6;
    {
        let x = 2;
        println!("the value of x is {x}");
    }
    let result = 5.6/ 3.2;

    println!("the value of x is {x}");
    println!("result is : {result}");

    println!("some constant is {SOME_CONSTANT}");
    x = 7;
    println!("the value of x is now {x}");
    
    let tup = (200, true, "hello");
    let (x,y,z) = tup;
    println!("x y and z are {x}, {y} and {z}");
    
    // this won't work
    //println!("tuple index 0 is: {tup.0}");
    
    let x = tup.0;
    println!("tuple index 0 is : {x}");
    
    let array = [1, 1, 2, 2];
    let ii = array[0];
    println!("first element ie 0 index of array is {ii}");
    
    let c: char = 'z';
    println!("the char is {c}");
    
    
}
