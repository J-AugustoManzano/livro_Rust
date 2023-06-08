use std::io;
use std::io::prelude::*;

fn main() {    

    let mut dividendo = String::new();
    let mut divisor = String::new();

    let dvd: f32;
    let dvr: f32;
    let quo: f32;

    print!("Entre o valor do dividendo ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dividendo).unwrap();
    dvd = dividendo.trim().parse::<f32>().unwrap();

    print!("Entre o valor do divisor .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut divisor).unwrap();
    dvr = divisor.trim().parse::<f32>().unwrap();

    quo = dvd / dvr;

    println!();
    println!("Quociente = {}", quo);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
