extern crate chrono;

use std::io;
use std::io::prelude::*;
use chrono::prelude::*;

fn main() {

    let datahora = Utc.ymd(1965, 4, 26).and_hms(17, 35, 0); 

    println!("Dia ..........................: {}", datahora.day());
    println!("Mês ..........................: {}", datahora.month());
    println!("Ano ..........................: {}", datahora.year());
    println!();
    println!("Hora .........................: {}", datahora.hour());
    println!("Minuto .......................: {}", datahora.minute());
    println!("Segundo ......................: {}", datahora.second());
    println!();
    println!("{}", datahora.format("Data: %d/%m/%Y | Hora: %Hh%Mmin"));
    println!();
    println!("Dia da semana (inglês) .......: {}", datahora.format("%A"));
    print!("Dia da semana (português) ....: ");

    match datahora.weekday().number_from_monday() {
        1 => println!("Segunda-feira"),
        2 => println!("Terça-feira"),       
        3 => println!("Quarta-feira"),       
        4 => println!("Quinta-feira"),       
        5 => println!("Sexta-feira"),       
        6 => println!("Sábado"),       
        7 => println!("Domingo"),
        _ => {} 
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
