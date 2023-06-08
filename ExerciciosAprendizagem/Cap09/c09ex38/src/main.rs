#![allow(unused_assignments)]
extern crate cipher_crypt;

use cipher_crypt::{Cipher, Vigenere};

use std::io;
use std::io::prelude::*;

fn main() {

    let mut msgorig = String::new();
    let mut msgcifr = String::new();
    let mut msgdeci = String::new();
    let mut chave   = String::new();

    let rotacao;

    println!("CIFRA DE VIGENÈRE");
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

    print!("Informe chave de cifragem .......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut chave).expect("Entrada incorreta");

    if let Some('\n') = chave.chars().next_back() {
        chave.pop();
    }
    if let Some('\r') = chave.chars().next_back() {
        chave.pop();
    }
    println!();

    rotacao = Vigenere::new(String::from(chave)).unwrap();

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
