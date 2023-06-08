extern crate chrono;

use std::io;
use std::io::prelude::*;
use chrono::Local;

fn main() {

    let data = Local::now();

    println!("{}", data.format("Data: %d/%m/%Y | Hora: %Hh%Mmin  - %U"));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

