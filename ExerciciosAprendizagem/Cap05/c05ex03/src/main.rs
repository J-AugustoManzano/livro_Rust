use std::io;
use std::io::prelude::*;

fn pausa() {
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn fatorial() {

    let mut n = String::new();
    let mut f: u64 = 1;

    print!("Entre o valor da fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Erro na entrada.");
    let n: u64 = match n.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
    
    for i in 1 .. n + 1 {
		f *= i;
	}
	    
    println!("Resultado = {}", f);  
    pausa();
}

fn main() {
	
    fatorial();
    
}
