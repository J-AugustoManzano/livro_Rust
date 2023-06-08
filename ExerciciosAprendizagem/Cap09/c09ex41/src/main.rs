#![allow(unused_variables, unused_assignments)]

use std::io;
use std::io::prelude::*;

use std::fs::File;
use std::fs;

fn main() {    

    let arquivo;
    let acesso: &'static str = "../../arquiv.dad";

    match fs::metadata(acesso) {
        Ok(_) => {
            println!("Arquivo existente.");
        },
        Err(_) => {
            arquivo = File::create(acesso).unwrap();
            println!("Arquivo criado com sucesso.");
        }
    };
 
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
