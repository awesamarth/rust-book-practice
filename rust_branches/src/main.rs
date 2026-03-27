fn main() {
    let number = 3;
    if number < 5 {
        println!("the number is less than 5");
    } else {
        println!("the number is more than or equal to 5");
    }
    let condition = number < 3;
    let mut new_number = if condition { 5 } else { 6 };

    println!("new number is: {new_number}");

    let result = loop {
        new_number += 1;

        if new_number == 10 {
            break new_number * 2;
        }
    };

    println!("the result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining count is: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
                
            }
            remaining -=1;
            
        }
        count+=1;
    }
    println!("end count is: {count}");
    
    while count <10{
        println!("{count}!");
        count+=1;
    }
    println!("LIFTOFF!!");
    
    let an_array = [1, 2, 5, 2, 11, 23];
    for mut element  in an_array {
        println!("the value is: {element}");
        element+=1;
        println!("value+1 is {element}");
        
    }
    
    
    // countdown via .rev
    for number in (1..4).rev(){
        println!("{number}!");
        
    }
    println!("LIFTOFF!!");
}
