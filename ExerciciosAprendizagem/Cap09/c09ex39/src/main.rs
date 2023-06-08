extern crate crypto;
 
use std::io;
use std::io::prelude::*;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::whirlpool::Whirlpool;
 
fn main() {

    let mut palavra = Md5::new();
    palavra.input_str("Linguagem Rust");
    println!("MD5 .........: {}", palavra.result_str());

    let mut palavra = Sha1::new();
    palavra.input_str("Linguagem Rust");
    println!("SHA1 ........: {}", palavra.result_str());

    let mut palavra = Whirlpool::new();
    palavra.input_str("Linguagem Rust");
    println!("WHIRLPOOL ...: {}", palavra.result_str());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
