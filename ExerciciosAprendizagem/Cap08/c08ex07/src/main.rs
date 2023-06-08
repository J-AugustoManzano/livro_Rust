#![allow(unused_variables, dead_code)]
use std::io;
use std::io::prelude::*;

enum Meses {
    Janeiro,
    Fevereiro,
    Marco,
    Abril,
    Maio,
    Junho,
    Julho,
    Agosto,
    Setembro,
    Outubro,
    Novembro,
    Dezembro,
}

fn main() {

    let mes = Meses::Janeiro;
    
    println!("Teste de acesso a lista enumerada");
    println!();
    print!("Mês definido = "); 
    
    match mes {
        Meses::Janeiro   => println!("Janeiro"),
        Meses::Fevereiro => println!("Fevereiro"),
        Meses::Marco     => println!("Março"),
        Meses::Abril     => println!("Abril"),
        Meses::Maio      => println!("Maio"),
        Meses::Junho     => println!("Junho"),
        Meses::Julho     => println!("Julho"),
        Meses::Agosto    => println!("Agosto"),
        Meses::Setembro  => println!("Setembro"),
        Meses::Outubro   => println!("Outubro"),
        Meses::Novembro  => println!("Novembro"),
        Meses::Dezembro  => println!("Dezembro"),
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
