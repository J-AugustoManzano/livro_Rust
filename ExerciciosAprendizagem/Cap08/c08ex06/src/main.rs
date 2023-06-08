#![allow(unused_variables)]
use std::io;
use std::io::prelude::*;

struct Animal;  
struct Gato;
struct Cao;

impl Animal {  
    fn miar(&self, som: &Gato) -> () {
        println!("Miau, miau, miau!");
    }
}

impl Animal {  
    fn latir(&self, som: &Cao) -> () {
        println!("Au au, au au, au au!");
    }
}

fn main () {  

    let bichinho = Animal{};
    let gato = Gato{};
    let cao = Cao{};

    print!("O gato faz: "); bichinho.miar(&gato);
    print!("O cão faz: "); bichinho.latir(&cao);
 
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
