use std::io;
use std::io::prelude::*;

fn main() {

    let palavra = "Mundo!";
    let mensagem = format!("Alô, {}!", palavra);

    println!("{}", mensagem);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
