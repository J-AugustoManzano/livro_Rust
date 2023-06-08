use std::io;
use std::io::prelude::*;

fn main() {    

    let mut sal_atu = String::new();
    let mut per_rea = String::new();

    let sa: f32;
    let pr: f32;
    let ns: f32;
 
    print!("Entre valor do salário atual ....: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sal_atu).unwrap();
    sa = sal_atu.trim().parse::<f32>().unwrap();

    print!("Entre o percentual de reajuste ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut per_rea).unwrap();
    pr = per_rea.trim().parse::<f32>().unwrap();

    ns = sa + sa * pr / 100.0;

    println!("Novo salário ....................: {:8.2}", ns);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
