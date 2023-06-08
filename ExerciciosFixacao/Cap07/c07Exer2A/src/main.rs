use std::io;
use std::io::prelude::*;

fn ordendistr(matriz: &mut [i32]) {
    for i in 0 .. matriz.len() - 1 {
        for j in i + 1 .. matriz.len() {
            if matriz[i] < matriz[j] {
                matriz.swap(i, j);
            }
        }
    }
}

fn main() {
	
	let mut a: [i32;  8] = [0;  8];   
    
    let mut valor = String::new();
    
    for i in 0 .. a.len() {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }
    println!();
    
    ordendistr(&mut a);
    
    for i in 0 .. a.len() {
        println!("A[{}] = {:4}", i + 1, a[i]);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
