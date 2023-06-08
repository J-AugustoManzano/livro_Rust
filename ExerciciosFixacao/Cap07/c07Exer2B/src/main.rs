use std::io;
use std::io::prelude::*;

fn main() {
	
	let mut a: [f64;  5] = [0.;  5];   
    let mut aux;
    
    let mut valor = String::new();
    
    for i in 0 .. a.len() {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f64>().unwrap();
        valor.clear();
    }
    println!();
    
    for i in 0 .. a.len() {
        aux = i;
        for j in i + 1 .. a.len() {
            if a[aux] > a[j] { 
                aux = j;
            }
        }
        a.swap(i, aux);
    }
    
    for i in 0 .. a.len() {
        println!("A[{}] = {:4}", i + 1, a[i]);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
