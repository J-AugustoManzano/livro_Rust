use std::io;
use std::io::prelude::*;

fn trocai32(a: &mut i32, b: &mut i32) {
    let x: i32;
    x = *a;
    *a = *b;
    *b = x;
}
 
fn trocaf32(a: &mut f32, b: &mut f32) {
    let x: f32;
    x = *a;
    *a = *b;
    *b = x;
}

fn trocachar(a: &mut char, b: &mut char) {
    let x: char;
    x = *a;
    *a = *b;
    *b = x;
}

fn main() {
    
    let mut valor_a = 1;
    let mut valor_b = 2;
    
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    trocai32(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    let mut valor_a = 3.5;
    let mut valor_b = 4.5;
    
    println!("\n");
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    trocaf32(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    let mut valor_a = 'a';
    let mut valor_b = 'b';
    
    println!("\n");
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    trocachar(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
