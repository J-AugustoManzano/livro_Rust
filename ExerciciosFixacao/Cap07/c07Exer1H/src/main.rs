use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [u32; 12] = [0; 12];              
    
    let mut valor = String::new();   
    
    let mut soma_par: u32 = 0;
    let mut soma_imp: u32 = 0;
    
    for i in 0 .. a.len() {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<u32>().unwrap();
        valor.clear();
    }
    println!();

    for i in 0 .. a.len() {
        if a[i] % 2 == 0 {
            soma_par += 1;
        } else {
            soma_imp += 1;
        }
    }

    println!("Elementos pares ....: {}", soma_par);
    println!("Elementos impares ..: {}", soma_imp);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
