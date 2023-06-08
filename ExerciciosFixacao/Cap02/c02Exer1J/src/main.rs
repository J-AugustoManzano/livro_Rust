use std::io;
use std::io::prelude::*;

fn main() {    

    let mut raio = String::new();

    let vol: f64;
    let r: f64;
 
    print!("Entre valor do raio ......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut raio).unwrap();
    r = raio.trim().parse::<f64>().unwrap();

    vol = 3.14159265 * r.powi(2);

    println!("Volume da circunferência .: {}", vol);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
