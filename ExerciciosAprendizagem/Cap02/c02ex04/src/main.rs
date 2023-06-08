use std::io;
use std::io::prelude::*;

fn main() {    

    let mut hora_trab = String::new();
    let mut vlr_hora = String::new();
    let mut percent_desc = String::new();

    let ht: f64;
    let vh: f64;
    let pd: f64;
    let td: f64;
    let sb: f64;
    let sl: f64;

    print!("Entre a quantidade de horas trabalhadas ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hora_trab).unwrap();
    ht = hora_trab.trim().parse::<f64>().unwrap();

    print!("Entre o valor da hora de trabalho .........: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut vlr_hora).unwrap();
    vh = vlr_hora.trim().parse::<f64>().unwrap();

    print!("Entre o valor do percentual de desconto ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut percent_desc).unwrap();
    pd = percent_desc.trim().parse::<f64>().unwrap();

    sb = ht * vh;
    td = (pd / 100.) * sb;
    sl = sb - td;

    println!();
    println!("Salário Bruto ....: {:8.2}", sb);
    println!("Desconto .........: {:8.2}", td);
    println!("Salário Liquido  .: {:8.2}", sl);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
