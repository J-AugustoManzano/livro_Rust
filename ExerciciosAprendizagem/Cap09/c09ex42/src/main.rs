use std::io;
use std::io::prelude::*;

use std::fs;
use std::fs::OpenOptions;

fn main() {    

    let mut arquivo;
    let acesso: &'static str = "../../arquiv.dad";
    let mut conteudo = String::new();
 
    match fs::metadata(acesso) {
        Ok(_) => {
            arquivo = OpenOptions::new().write(true).append(true)
               .open(acesso).unwrap();
            println!("Informe uma palavra:");
            print!("--> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut conteudo).expect("Entrada incorreta");
            arquivo.write_all(conteudo.as_bytes()).unwrap();
        },
        Err(_) => {
            println!("Arquivo inexistente.");
        }
    };

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
