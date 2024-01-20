use std::fs::{File};
use std::io::Error;
use anyhow::{anyhow, Context, ensure, Result};
use anyhow::__private::kind::{AdhocKind, TraitKind};

fn main() {
    println!("Hello, world!");
    let x = check(true);
    println!("{:#?}", x);

    match x{
        Err(val) => {
             println!("The vals : {:?}", val);
        },
        _ => ()
    }

    let z = check_two(false);
    println!("The second is {:?}", z.with_context(||"AGAINA"));
}


fn check(val:bool)-> Result<()>{
    // ensure!(false, "Not true");
    let file_open = File::open("hello.txt").with_context(|| format!("NEVER MISS THE GAME"))?;
    println!("the stat is {:?}", file_open);
    Ok(())
}


fn check_two(val:bool)-> Result<()>{
    match val {
        true => Ok(()),
        false => Err(anyhow!("Not applicable"))
    }
}

//The out is Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })