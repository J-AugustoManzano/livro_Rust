use std::io;
use std::io::prelude::*;

fn main() {    

    let x: u8 = 5;
    let y = x as i64;

    println!("{}", y);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
