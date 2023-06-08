#[macro_use]
extern crate cfor;

use std::io;
use std::io::prelude::*;

fn main() {

    cfor!(let mut i = 1; i <= 10; i += 2; {
        println!("{}", i);
    });

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
