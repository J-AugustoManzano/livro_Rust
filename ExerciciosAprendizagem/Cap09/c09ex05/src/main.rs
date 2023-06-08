use std::io;
use std::io::prelude::*;

struct ClsGenerica<T> {
    dado: T
}

impl <T> ClsGenerica<T> {
    fn pegavalor(&self) -> &T { 
        return &self.dado;
    }
}

fn main() {

    let x1 = ClsGenerica {dado: 5u32};
    let x2 = ClsGenerica {dado: 5.2f64};
    
    println!("{}", x1.pegavalor());
    println!("{}", x2.pegavalor());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
