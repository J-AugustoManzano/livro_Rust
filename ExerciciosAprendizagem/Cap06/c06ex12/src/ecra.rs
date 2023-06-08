pub const BLACK:    u8 =  0; // Preto
pub const BLUE:     u8 =  1; // Azul
pub const GREEN:    u8 =  2; // Verde
pub const CYAN:     u8 =  3; // Ciano
pub const RED:      u8 =  4; // Vermelho
pub const MAGENTA:  u8 =  5; // Magenta
pub const BROWN:    u8 =  6; // Marrom
pub const LGRAY:    u8 =  7; // Cinza claro
pub const DGRAY:    u8 =  8; // Cinza escuro
pub const LBLUE:    u8 =  9; // Azul claro
pub const LGREEN:   u8 = 10; // Verde claro
pub const LCYAN:    u8 = 11; // Ciano claro
pub const LRED:     u8 = 12; // Vermelho claro
pub const LMAGENTA: u8 = 13; // Magenta claro
pub const YELLOW:   u8 = 14; // Amarelo
pub const WHITE:    u8 = 15; // Branco

pub fn normal() {
    print!("{}[0m", 27 as char);
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn background(cor: u8) {
    match cor {
         BLACK   => print!("{}[40m", 27 as char), // Preto
         BLUE    => print!("{}[44m", 27 as char), // Azul
         GREEN   => print!("{}[42m", 27 as char), // Verde
         CYAN    => print!("{}[46m", 27 as char), // Ciano
         RED     => print!("{}[41m", 27 as char), // Vermelho
         MAGENTA => print!("{}[45m", 27 as char), // Magenta
         BROWN   => print!("{}[43m", 27 as char), // Marron
         LGRAY   => print!("{}[47m", 27 as char), // Cinza claro
         _       => {},
    }
}

pub fn text(cor: u8) {
    match cor { 
         BLACK    => print!("{}[30m", 27 as char),   // Preto
         BLUE     => print!("{}[34m", 27 as char),   // Azul
         GREEN    => print!("{}[32m", 27 as char),   // Verde
         CYAN     => print!("{}[36m", 27 as char),   // Ciano
         RED      => print!("{}[31m", 27 as char),   // Vermelho
         MAGENTA  => print!("{}[35m", 27 as char),   // Magenta
         BROWN    => print!("{}[33m", 27 as char),   // Marron
         LGRAY    => print!("{}[37m", 27 as char),   // Cinza claro
         DGRAY    => print!("{}[1;30m", 27 as char), // Cinza claro
         LBLUE    => print!("{}[1;34m", 27 as char), // Azul claro
         LGREEN   => print!("{}[1;32m", 27 as char), // Verde claro
         LCYAN    => print!("{}[1;36m", 27 as char), // Ciano claro
         LRED     => print!("{}[1;31m", 27 as char), // Vermelho claro
         LMAGENTA => print!("{}[1;35m", 27 as char), // Magenta claro
         YELLOW   => print!("{}[1;33m", 27 as char), // Amarelo
         WHITE    => print!("{}[1;37m", 27 as char), // Branco
         _        => {},
    }
}

pub fn position(linha: u8, coluna: u8) {
    if coluna >= 1 && coluna <= 80 && linha >= 1 && linha <= 24 {
        print!("{}[{};{}H", 27 as char, linha, coluna);
    }
}
