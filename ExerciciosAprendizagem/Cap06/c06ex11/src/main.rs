#![allow(dead_code)]
use std::io;
use std::io::prelude::*;

// Constantes para as cores de texto e fundo

const BLACK:    u8 =  0; // Preto
const BLUE:     u8 =  1; // Azul
const GREEN:    u8 =  2; // Verde
const CYAN:     u8 =  3; // Ciano
const RED:      u8 =  4; // Vermelho
const MAGENTA:  u8 =  5; // Magenta
const BROWN:    u8 =  6; // Marrom
const LGRAY:    u8 =  7; // Cinza claro

// Constantes para as cores de texto

const DGRAY:    u8 =  8; // Cinza escuro
const LBLUE:    u8 =  9; // Azul claro
const LGREEN:   u8 = 10; // Verde claro
const LCYAN:    u8 = 11; // Ciano claro
const LRED:     u8 = 12; // Vermelho claro
const LMAGENTA: u8 = 13; // Magenta claro
const YELLOW:   u8 = 14; // Amarelo
const WHITE:    u8 = 15; // Branco

fn normal() {
    print!("{}[0m", 27 as char);
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn background(cor: u8) {
    match cor {
         BLACK   => print!("{}[40m", 27 as char), // Preto
         BLUE    => print!("{}[44m", 27 as char), // Azul
         GREEN   => print!("{}[42m", 27 as char), // Verde
         CYAN    => print!("{}[46m", 27 as char), // Ciano
         RED     => print!("{}[41m", 27 as char), // Vermelho
         MAGENTA => print!("{}[45m", 27 as char), // Magenta
         BROWN   => print!("{}[43m", 27 as char), // Marron
         LGRAY   => print!("{}[47m", 27 as char), // Cinza claro
         _       => {},
    }
}

fn text(cor: u8) {
    match cor { 
         BLACK    => print!("{}[30m", 27 as char),   // Preto
         BLUE     => print!("{}[34m", 27 as char),   // Azul
         GREEN    => print!("{}[32m", 27 as char),   // Verde
         CYAN     => print!("{}[36m", 27 as char),   // Ciano
         RED      => print!("{}[31m", 27 as char),   // Vermelho
         MAGENTA  => print!("{}[35m", 27 as char),   // Magenta
         BROWN    => print!("{}[33m", 27 as char),   // Marron
         LGRAY    => print!("{}[37m", 27 as char),   // Cinza claro
         DGRAY    => print!("{}[1;30m", 27 as char), // Cinza claro
         LBLUE    => print!("{}[1;34m", 27 as char), // Azul claro
         LGREEN   => print!("{}[1;32m", 27 as char), // Verde claro
         LCYAN    => print!("{}[1;36m", 27 as char), // Ciano claro
         LRED     => print!("{}[1;31m", 27 as char), // Vermelho claro
         LMAGENTA => print!("{}[1;35m", 27 as char), // Magenta claro
         YELLOW   => print!("{}[1;33m", 27 as char), // Amarelo
         WHITE    => print!("{}[1;37m", 27 as char), // Branco
         _        => {},
    }
}

fn position(linha: u8, coluna: u8) {
    if coluna >= 1 && coluna <= 80 && linha >= 1 && linha <= 24 {
        print!("{}[{};{}H", 27 as char, linha, coluna);
    }
}

fn lower() {
    print!("{}[2m", 27 as char);
}

fn high() {
    print!("{}[1m", 27 as char);
}

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
