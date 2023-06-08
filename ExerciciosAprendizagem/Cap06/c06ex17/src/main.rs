use std::io;
use std::io::prelude::*;

fn main() {

    println!("{:?}", b"Linguagem Rust");

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
