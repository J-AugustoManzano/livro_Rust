
use std::io;
use std::io::prelude::*;

fn main() {    

    let mut base = String::new();
    let mut indice = String::new();

    let bas: f64;
    let ind: i32;

    print!("Entre o valor da base .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    bas = base.trim().parse::<f64>().unwrap();

    print!("Entre o valor do índice ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).unwrap();
    ind = indice.trim().parse::<i32>().unwrap();

    println!();
    println!("Exponenciação = {:8.2}", bas.powi(ind));
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
