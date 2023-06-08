extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::io::prelude::*;

fn main() {
	
    let horautc:   DateTime<Utc>   = Utc::now(); 
    let horalocal: DateTime<Local> = Local::now(); 

    println!("{}", horautc);
    println!("{}", horalocal);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

