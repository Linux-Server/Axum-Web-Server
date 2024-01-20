use std::fs::{File};
use std::io::Error;
use anyhow::Result;
fn main() {
    println!("Hello, world!");
    let x = check(true);
    println!("The out is {:?}", x);
}


fn check(val:bool)-> Result<()>{
   let file_open = File::open("hello.txt")?;
    println!("the stat is {:?}", file_open);
    Ok(())
}

//The out is Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })