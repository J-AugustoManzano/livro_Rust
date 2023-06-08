use std::io;
use std::io::prelude::*;

fn main() {    

    let mut r: u16;

    for i in 15 .. 201 {
        r = (i as u16).pow(2);
        println!("{:5}", r);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
