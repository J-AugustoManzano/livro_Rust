use std::io;
use std::io::prelude::*;

fn main() {    

    let mut base = String::new();
    let mut indice = String::new();

    let bas: f64;
    let ind: f64;

    print!("Entre o valor da base .....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    bas = base.trim().parse::<f64>().unwrap();

    print!("Entre o valor do índice ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut indice).unwrap();
    ind = indice.trim().parse::<f64>().unwrap();

    println!();
    println!("Exponenciação = {:8.2}", bas.powf(ind));
    println!("Raiz quadrada = {:8.2}", bas.sqrt());
    println!("Raiz cúbica   = {:8.2}", bas.cbrt());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
