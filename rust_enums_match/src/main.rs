enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(UsState)
}



impl Message{
    fn call(&self){
        println!("Quit");
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn main() {
    let message = Message::Quit;
    message.call();
    
    value_in_cents(Coin::Quarter(UsState::Alabama));
    
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    }
    
    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn reroll(){}
    
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_alternative(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn value_in_cents(coin : Coin)->u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel=>5,
        Coin::Quarter(state)=>{
            println!("State quarter from {state:?}!");
            25
        }
    }
}
