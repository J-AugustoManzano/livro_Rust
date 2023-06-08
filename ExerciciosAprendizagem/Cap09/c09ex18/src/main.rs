extern crate chrono;
extern crate chrono_tz;

use std::io;
use std::io::prelude::*;

use chrono::*;
use chrono_tz::UTC;

use chrono_tz::America::Sao_Paulo;
use chrono_tz::Europe::Lisbon;

fn main() {

    let dh = Sao_Paulo.ymd(2017, 4, 26).and_hms(0, 0, 0);
    let h  = dh.with_timezone(&UTC);
   
    println!("São Paulo ...: {}", dh.format("Data: %d/%m/%Y | Hora: %Hh%Mmin"));
    println!("Hora UTC ....: {}", h.format("Data: %d/%m/%Y | Hora: %Hh%Mmin"));
    println!();
    
    let dh = Lisbon.ymd(2017, 4, 26).and_hms(0, 0, 0);
    let h  = dh.with_timezone(&UTC);
   
    println!("Lisboa ......: {}", dh.format("Data: %d/%m/%Y | Hora: %Hh%Mmin"));
    println!("Hora UTC ....: {}", h.format("Data: %d/%m/%Y | Hora: %Hh%Mmin"));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
