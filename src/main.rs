// use std::env;
mod readfile;
mod clapcli;
mod clapcli2;
mod todocli;
fn main(){
    // let args: Vec<String> = env::args().collect();
    // if args.len() <2 {
    //     println!("No argument passed");
    //     return;
    // }
    //   // dbg!(&args);
    // let result = args[1].trim().chars().rev().collect::<String>();
    // println!("the reversed string is : {result}");
    //readfile::readfile().unwrap();
    //clapcli::cliofcpi();
    clapcli2::clapcli2();
}