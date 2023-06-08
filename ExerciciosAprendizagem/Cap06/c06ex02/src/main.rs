use std::io;
use std::io::prelude::*;

fn main() {    

    let valor1 = true as u8;     // mostra 1
    let valor2 = false as u8;    // mostra 0
    let valor3 = 65u8 as char;   // mostra A
    let valor4 = -5i8 as u8;     // mostra 251
    let valor5 = 10.99f32 as i8; // mostra 10
    let valor6 = 513u32 as u8;   // mostra 1
    let valor7 = 987u32 as u64;  // mostra 987
    let valor8 = -9i8 as i16;    // mostra -9

    println!("{}", valor1);
    println!("{}", valor2);
    println!("{}", valor3);
    println!("{}", valor4);
    println!("{}", valor5);
    println!("{}", valor6);
    println!("{}", valor7);
    println!("{}", valor8);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
