use std::io;
use std::io::prelude::*;

fn main() {    

    let mut celsius = String::new();

    let c: f64;
    let f: f64;
 
    print!("Entre temperatura em gruas Celsius ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut celsius).unwrap();
    c = celsius.trim().parse::<f64>().unwrap();

    f = (9.0 * c + 160.0) / 5.0;

    println!("Temperatura em graus Fahrenheit .....: {}", f);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
