use std::io;
use std::io::prelude::*;

fn divisao(dividendo: f32, divisor: f32) -> Option<f32> {
    if dividendo == 0. || divisor == 0. {
        return None;
    } else {
        return Some(dividendo / divisor);
    }
}

fn main() {    

    let mut dividendo = String::new();
    let mut divisor = String::new();

    let dvd: f32;
    let dvr: f32;

    print!("Entre o valor do dividendo ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dividendo).unwrap();
    dvd = dividendo.trim().parse::<f32>().unwrap();

    print!("Entre o valor do divisor .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut divisor).unwrap();
    dvr = divisor.trim().parse::<f32>().unwrap();

    match divisao(dvd, dvr) {
        Some(quo) => {
            println!("\nQuociente = {}", quo)
        },
        None => println!("\nErro na operação"),
    }
 
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
