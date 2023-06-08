use std::io;
use std::io::prelude::*;

use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
	
   let iniciocrono = SystemTime::now();
   let mut i: u8 = 1;
   
   loop {
       sleep(Duration::new(1, 0));
       match iniciocrono.elapsed() {
           Ok(elapsed) => {
               println!("Mensagem no segundo: {}", elapsed.as_secs());
           }
           Err(erro) => {
               println!("Erro na operação: {:?}", erro);
           }
        }
        i += 1;
        if i > 5 { break; }
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
