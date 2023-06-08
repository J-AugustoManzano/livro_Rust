use std::io;
use std::io::prelude::*;

fn main() {

    let mensagem = "Linguagem Rust".to_string();

    println!("{}", mensagem);
    println!();

    for i in mensagem.as_bytes() {
        print!("{} ", i);
    }    
    println!("\n");

    for i in mensagem.chars() {
        print!("{} ", i);
    }    
    println!("\n");

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
