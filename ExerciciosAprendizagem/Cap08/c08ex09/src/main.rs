#![allow(unused_variables, unused_assignments)]
use std::io;
use std::io::prelude::*;

#[derive(Debug)] 
enum Meses {
    Janeiro = 1,
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

impl Meses {
    fn pega_mes(valor: u8) -> Meses {
        match valor {
             1 => Meses::Janeiro,
             2 => Meses::Fevereiro,
             3 => Meses::Marco,
             4 => Meses::Abril,
             5 => Meses::Maio,
             6 => Meses::Junho,
             7 => Meses::Julho,
             8 => Meses::Agosto,
             9 => Meses::Setembro,
            10 => Meses::Outubro,
            11 => Meses::Novembro,
            12 => Meses::Dezembro,
             _ => panic!(""),
        }
    }
}

fn main() {

    let mut entra_mes = String::new();
    let mut m: u8;
    let mes: Meses;

    println!("Teste de acesso a lista enumerada");
    println!();
    loop {
        print!("Entre valor numérico entre 1 e 12: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut entra_mes).unwrap();
        m = entra_mes.trim().parse::<u8>().unwrap();
        if m >= 1 && m <= 12 { break; }
        entra_mes.clear();
    }

    mes = Meses::pega_mes(m);
    
    println!();
    print!("Mês {} igual a {:?}\n", m, Meses::pega_mes(m)); 
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
