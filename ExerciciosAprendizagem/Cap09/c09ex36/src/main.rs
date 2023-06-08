use std::io;
use std::io::prelude::*;

fn main() {

    let cadeia = String::from("Alô, Mundo!");
    
    let cadeia_string = &cadeia; 
    let cadeia_str: &str = &cadeia; 

    println!("Cadeia do tipo String ...: {}", cadeia_string);
    println!("Cadeia do tipo &str .....: {}", cadeia_str);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
