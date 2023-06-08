use std::io;
use std::io::prelude::*;

fn main() {    

    let x = true;
    let y: bool = false;

    println!("Valor logico verdadeiro de X ...: {}", x);
    println!("Valor logico falso de Y ........: {}", y);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
