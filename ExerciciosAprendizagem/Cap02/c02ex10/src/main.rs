use std::io;
use std::io::prelude::*;

fn main() {    

    const PI: f64 = 3.14159;

    let mut altura = String::new();
    let mut raio = String::new();

    let h: f64;
    let r: f64;
    let a: f64;

    print!("Entre a altura ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut altura).unwrap();
    h = altura.trim().parse::<f64>().unwrap();

    print!("Entre o raio .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut raio).unwrap();
    r = raio.trim().parse::<f64>().unwrap();

    a = 2.0 * PI * r * (r + h);
    
    println!();
    println!("Área do Cilindro .: {:8.2}", a);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
