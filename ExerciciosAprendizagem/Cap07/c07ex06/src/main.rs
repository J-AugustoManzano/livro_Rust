use std::io;
use std::io::prelude::*;
use std::mem;

fn main() {

    let valores: [[i32; 5]; 3] = [
                                     [ 0,  1,  2,  3,  4],
                                     [ 5,  6,  7,  8,  9],
                                     [10, 11, 12, 13, 14]
                                 ];
    for i in 0..3 {
        for j in 0..5 {
            println!("VALORES[{},{}] = {}", i + 1, j + 1, valores[i][j]);	
        }
    }

    println!();
    println!("A matriz ocupa {} bytes", mem::size_of_val(&valores));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
