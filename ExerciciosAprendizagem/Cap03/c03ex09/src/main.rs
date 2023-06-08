use std::io;
use std::io::prelude::*;

fn main() {    

    let mut mes = String::new();

    let m: u8;

    print!("Entre um numero equivalente a um MES: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mes).unwrap();
    m = mes.trim().parse::<u8>().unwrap();

    println!();

    if m == 1 {
        println!("Janeiro.");
    }

    if m == 2 {
        println!("Fevereiro.");
    }

    if m == 3 {
        println!("Março.");
    }

    if m == 4 {
        println!("Abril.");
    }

    if m == 5 {
        println!("Maio.");
    }

    if m == 6 {
        println!("Junho.");
    }

    if m == 7 {
        println!("Julho.");
    }

    if m == 8 {
        println!("Agosto.");
    }

    if m == 9 {
        println!("Setembro.");
    }

    if m == 10 {
        println!("Outubro.");
    }

    if m == 11 {
        println!("Novembro.");
    }

    if m == 12 {
        println!("Dezembro.");
    }

    if m < 1 || m > 12 {
        println!("Mês inválido.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
