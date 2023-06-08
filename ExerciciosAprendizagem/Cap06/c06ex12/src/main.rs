extern crate ecralib;

use ecralib::*;

use std::io;
use std::io::prelude::*;

fn main() {

    clear();
    println!("Teste de padrão de Cores com Código Numérico\n\n");

    background(1);
    text(14);
    println!("Fundo: Azul // Texto: Amarelo");
    println!();
    normal();
    
    background(1);
    text(7);
    println!("Fundo: Azul // Texto: Cinza claro");
    println!();
    normal();

    background(4);
    text(14);
    println!("Fundo: Vermelho // Texto: Amarelo");
    println!();
    normal();

    background(0);
    text(2);
    println!("Fundo: Preto // Texto: Verde");
    println!();
    normal();

    background(7);
    text(12);
    println!("Fundo: Cinza claro // Texto: Vermelho claro");
    println!();
    normal();

    background(BROWN);
    text(WHITE);
    println!("Fundo: Marrom // Texto: Branco");
    println!();
    normal();

    println!();
    position(24,1);
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
