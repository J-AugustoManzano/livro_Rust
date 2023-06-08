use std::io;
use std::io::prelude::*;
 
fn main() {
	
    let mut valores = 1 .. 101;
    let lista;
    let mut pesq = String::new(); 
    
    print!("\nEntre o valor a ser pesquisado: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut pesq).expect("Erro na entrada.");
        let pesq: u32 = match pesq.trim().parse() {
            Ok(numero) => numero,
            Err(_) => 0
        };
            
     lista = valores.find(|vlr| *vlr == pesq);

    match lista {
        Some(valor) => println!("Encontrei {}.", valor),
        None => println!("Não encontrei o valor!"),
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
