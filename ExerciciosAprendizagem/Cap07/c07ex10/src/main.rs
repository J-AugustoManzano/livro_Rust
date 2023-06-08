use std::io;
use std::io::prelude::*;

fn ordendistr(matriz: &mut [u32]) {

    let tamanho = matriz.len();

    for i in 0 .. tamanho - 1 {
        for j in i + 1 .. tamanho {
            if matriz[i] > matriz[j] {
                matriz.swap(i, j);
            }
        }
    }

}

fn main() {
	
    let mut valores: [u32; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    
    ordendistr(&mut valores);
    
    println!("{:?}", valores);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
