#![allow(unused_variables)]
use std::io;
use std::io::prelude::*;

fn main() {    

    let a: u8 = 1;
    let b: u8 = 2;

    println!("variável [A] = {}", a);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
