use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        
        Ok(file)=>file,
        Err(error)=>match error.kind(){
        ErrorKind::NotFound => match File::create("hello.txt"){
            Ok(fc)=> fc,
            Err(e)=>panic!("Problem creating the file")
        },
         _ =>{
             panic!("Problem in displaying file");
         }
        }
        
    };
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
    
        File::open("hello.txt")?.read_to_string(&mut username)?;
    
        Ok(username)
    }
    
    panic!("crash and burn!");
    
}
d 