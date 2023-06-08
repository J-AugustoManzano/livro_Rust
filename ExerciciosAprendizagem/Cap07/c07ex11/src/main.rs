use std::io;
use std::io::prelude::*;

fn ordenselec(matriz: &mut [u32]) {

    let tamanho = matriz.len();
    let mut aux;

    for i in 0 .. tamanho {
        aux = i;
        for j in i + 1 .. tamanho {
            if matriz [aux] > matriz [j] { 
                aux = j;
            }
        }
        matriz.swap(i, aux);
    }

}

fn main() {
	
    let mut valores: [u32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    
    ordenselec(&mut valores);
    
    println!("{:?}", valores);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
