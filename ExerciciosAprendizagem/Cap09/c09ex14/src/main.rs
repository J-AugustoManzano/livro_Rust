extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::io::prelude::*;

fn main() {
	
    let datahora: DateTime<Local> = Local::now();  

    println!("Dia ..........................: {}", datahora.day());
    println!("Mês ..........................: {}", datahora.month());
    println!("Ano ..........................: {}", datahora.year());
    println!();
    println!("Hora .........................: {}", datahora.hour());
    println!("Minuto .......................: {}", datahora.minute());
    println!("Segundo ......................: {}", datahora.second());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
