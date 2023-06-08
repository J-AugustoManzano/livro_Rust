use std::io;
use std::io::prelude::*;

#[derive(Debug)] 
enum Lista1 {
    Um,
    Dois,
    Tres,
}

#[derive(Debug)] 
enum Lista2 {
    Um    = 1,
    Dois,
    Tres,
}

#[derive(Debug)] 
enum Lista3 {
    Um    = 1,
    Tres  = 3,
    Cinco = 5,
}

fn main() {

    println!("{:?} = {}", Lista1::Um,    Lista1::Um    as u8);
    println!("{:?} = {}", Lista1::Dois,  Lista1::Dois  as u8);
    println!("{:?} = {}", Lista1::Tres,  Lista1::Tres  as u8);
    println!();

    println!("{:?} = {}", Lista2::Um,    Lista2::Um    as u8);
    println!("{:?} = {}", Lista2::Dois,  Lista2::Dois  as u8);
    println!("{:?} = {}", Lista2::Tres,  Lista2::Tres  as u8);
    println!();

    println!("{:?} = {}", Lista3::Um,    Lista3::Um    as u8);
    println!("{:?} = {}", Lista3::Tres,  Lista3::Tres  as u8);
    println!("{:?} = {}", Lista3::Cinco, Lista3::Cinco as u8);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
