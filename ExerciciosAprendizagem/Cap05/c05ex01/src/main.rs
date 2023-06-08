use std::io;
use std::io::prelude::*;

fn main() {    

    let x: i8 = 1;
    {
        let y: i8 = 2;
        println!("O valor de x é {} e o valor de y é {}.", x, y);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
