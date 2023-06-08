use std::io;
use std::io::prelude::*;

fn pausa() {
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

fn fatorial(n: u64) -> u64 {
    if n == 0 { 
        return 1; 
    } else { 
        return n * fatorial(n - 1); 
    }
}

fn escrevafat(vlr: u64, subrotina: fn(u64) -> u64) -> u64 {
    println!("Resultado = {}", subrotina(vlr));
    return vlr;
}

fn main() {    

    let mut valor = String::new();

    print!("Entre o valor da fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).expect("Erro na entrada.");
    let valor: u64 = match valor.trim().parse() {
        Ok(numero) => numero,
        Err(_) => 0
    };
	
    escrevafat(valor, fatorial);   
    pausa();

}
