#![allow(unused_variables, dead_code)]
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Formacao {
    Basico,
    Fundamental,
    Medio,
    Superior,
}

#[derive(Debug)]
struct Pessoa {
    nome: String,
    nascimento: u16,
    escolaridade: Formacao,
}

fn main() {

    let nome = "Augusto Manzano".to_string();
    let nascimento = 1965;
    let escolaridade = Formacao::Superior;
    
    let pessoa = Pessoa {nome, nascimento, escolaridade};
    
    println!("Nome ..........: {:?}", pessoa.nome);
    println!("Nascimento ....: {:?}", pessoa.nascimento);
    println!("Escolaridade ..: {:?}", pessoa.escolaridade);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
