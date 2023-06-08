use std::io;
use std::io::prelude::*;

fn main() {    

    let x: i8 = 1;
    {
        println!("x = {}.", x);
        let x: i8 = 2;
        println!("x = {}.", x);
    }

    println!("x = {}.", x);

    let x: i8 = 3;

    println!("x = {}.", x);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
