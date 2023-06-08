#![allow(unused_assignments)]
extern crate cipher_crypt;

use cipher_crypt::{Cipher, Caesar}; 

use std::io;
use std::io::prelude::*;

fn main() {

    let mut msgorig = String::new();
    let mut msgcifr = String::new();
    let mut msgdeci = String::new();

    let rotacao = Caesar::new(3).unwrap();

    println!("CIFRA DE CAESAR");
    println!();

    print!("Informe mensagem a ser cifrada ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut msgorig).expect("Entrada incorreta");

    if let Some('\n') = msgorig.chars().next_back() {
        msgorig.pop();
    }
    if let Some('\r') = msgorig.chars().next_back() {
        msgorig.pop();
    }
    println!();
    
    msgcifr = rotacao.encrypt(&msgorig).unwrap();
    msgdeci = rotacao.decrypt(&msgcifr).unwrap();

    println!("Mensagem original ......: {}", msgorig);
    println!("Mensagem com cifragem ..: {}", msgcifr.to_uppercase());
    println!("Mensagem sem cifragem ..: {}", msgdeci.to_uppercase());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
