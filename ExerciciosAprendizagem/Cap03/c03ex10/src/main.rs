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

    match m {
         1 => println!("Janeiro."),
         2 => println!("Fevereiro."),
         3 => println!("Março."),
         4 => println!("Abril."),
         5 => println!("Maio."),
         6 => println!("Junho."),
         7 => println!("Julho."),
         8 => println!("Agosto."),
         9 => println!("Setembro."),
        10 => println!("Outubro."),
        11 => println!("Novembro."),
        12 => println!("Dezembro."),
         _ => println!("Mês inválido."),
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
