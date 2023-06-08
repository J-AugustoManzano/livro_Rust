use std::io;
use std::io::prelude::*;

fn main() {

    let vlr = 10;
    let imutavel = &vlr as *const i32;

    unsafe {
        let valor = *imutavel;
        println!("Valor do ponteiro bruto imutável: {}", valor);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
