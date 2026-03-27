struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}


//impl method for adding methods on structs
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
    
    //associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("hellocro"),
        email: String::from("hello@hello.com"),
        sign_in_count: 0,
    };
    
    // could not have been able to mutate had
    // user1 not been mut
    user1.email = String::from("dlkjf@dlk.com");
    
    
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    let user2name = user2.username;
    println!("{user2name}");
    
    // program for area calculation
    let scale = 2;
    let rect1 = Rectangle{
        width:dbg!(30*scale),
        height:50
    };
    
    dbg!(&rect1);
    let area_of_rect= rect1.area();
    println!("{area_of_rect}");

    
    
    

}
