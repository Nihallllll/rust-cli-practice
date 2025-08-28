use std::fs::File;
use std::io::{self, Read};

pub fn readfile()-> io::Result<()> {
    let mut file = File::open("foo.txt")?;
     println!("{:?}", file);
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(())  
}