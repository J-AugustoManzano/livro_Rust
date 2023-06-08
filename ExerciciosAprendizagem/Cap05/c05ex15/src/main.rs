use std::io;
use std::io::prelude::*;

fn main() {    

    let mut a = String::new();
    let mut b = String::new();

    let compara = |vlr1: i8, vlr2: i8| -> bool {
        if vlr1 == vlr2 {
            return true;
        } else {
            return false;
        };
    };

    print!("Entre o 1o. valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).expect("Erro na entrada.");
    let a: i8 = match a.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
    println!();

    print!("Entre o 2o. valor numérico inteiro: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).expect("Erro na entrada.");
    let b: i8 = match b.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
    println!();

    if compara(a, b) {
        println!("Valores fornecidos são iguais.");
    } else {
        println!("Valores fornecidos são diferentes.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
