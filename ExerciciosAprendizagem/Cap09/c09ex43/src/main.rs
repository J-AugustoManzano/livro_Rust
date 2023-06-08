#![allow(unused_must_use)]

use std::io;
use std::io::prelude::*;

use std::fs::File;
use std::fs;

fn main() {

    let mut arquivo;
    let acesso: &'static str = "../../arquiv.dad";
    let mut conteudo = String::new();

    match fs::metadata(acesso) {
        Ok(_) => {
            arquivo = File::open(acesso).unwrap();
            arquivo.read_to_string(&mut conteudo);
            if let Some('\n') = conteudo.chars().next_back() {
                conteudo.pop();
            }
            if let Some('\r') = conteudo.chars().next_back() {
                conteudo.pop();
            }
            println!("{}", conteudo);
        },
        Err(_) => {
            println!("Arquivo inexistente.");
        }
    };
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
