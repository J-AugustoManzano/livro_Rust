extern crate rand;

use std::io;
use std::io::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

fn main() {    
 
    println!("\nAdivinhe um número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 11);

    loop {

        print!("\nEntre um número: ");
        io::stdout().flush().unwrap();

        let mut numero_informado = String::new();

        io::stdin().read_line(&mut numero_informado)
            .expect("Erro na entrada de dados.");

        let numero_informado: u32 = match numero_informado.trim().parse() {
            Ok(numero) => numero,
            Err(_) => continue
        };

        println!("Você informou o valor: {}", numero_informado);

        match numero_informado.cmp(&numero_secreto) {
            Ordering::Less    => println!("Valor é baixo!"),
            Ordering::Greater => println!("Valor é alto!"),
            Ordering::Equal   => {
                println!("Você acertou!");
                break;
            }
        }
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
