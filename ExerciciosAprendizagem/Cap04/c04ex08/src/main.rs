use std::io;
use std::io::prelude::*;

fn main() {

    for i in (1 .. 6).rev() {
        println!("{}", i);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
