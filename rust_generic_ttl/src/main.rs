use std::fmt::Display;

pub struct NewsArticle{
    pub headline :String,
    pub location: String,
    pub author: String, 
    pub content: String
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    
    fn summarize_author(&self) -> String;

}

impl Summary for NewsArticle {

    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + PartialOrd,
{23}


fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
    
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//lifetimes


fn main() {
    println!("Hello, world!");
    let r;

    {
        let x= String::from("hello");
        r = x;
    }

    println!("r: {r}");
    
}
