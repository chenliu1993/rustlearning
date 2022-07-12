use std::fs::File;
use std::io::{self, Read};

fn read_name_from_file() -> Result<String, io::Error> {
    // ? can only be used on Result<> return type
    let mut f = File::open("HelloWorld.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// use std::io::ErrorKind;
fn main() {
    // let f = File::open("Helloworld.txt")
    //     .expect("This is expected");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("Helloworld.txt") {
    //           Ok(fc) => fc,
    //           Err(e) => panic!("yes"),
    //         },
    //         other_error => panic!("no"),
    //     },
    // };
    
}

