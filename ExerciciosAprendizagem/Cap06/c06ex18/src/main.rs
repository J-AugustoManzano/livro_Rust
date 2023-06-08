use std::io;
use std::io::prelude::*;

fn main() {

    for i in 65 .. 70 {
        println!("{}", i as u8 as char);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
