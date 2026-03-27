use std::thread;

fn main() {
    
    let mut nlist = vec![1, 2, 3];
    println!("Before defining closure: {nlist:?}");

    let mut borrows_mutably = || nlist.push(7);
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    add_one_v3(1);
    add_one_v4(1);

    let list = vec![1,2,3];
    println!("before defining closure: {list:?}");
    
    thread::spawn(move || println!("From thread {list:?}"))
        .join()
        .unwrap();
    
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    
}
