use core::{arch::asm, ascii};


pub fn print_char(c: ascii::Char) {
    unsafe {
        asm!(
            "int 0x10",
            in("ah") 0x0e_u8,
            in("al") c as u8,
            in("bh") 0x00_u8,
            in("bl") 0x07_u8,
        );
    };
}


pub fn print_string(s: &str) {
    s.as_ascii().unwrap_or_else(|| panic!()).iter().for_each(|c| print_char(*c));
}


pub fn print_backspace() {
    // TODO handle if in the row beginning

    print_char(ascii::Char::Backspace);
    print_char(ascii::Char::Space);
    print_char(ascii::Char::Backspace);
}


pub fn print_newline() {
    print_char(ascii::Char::CarriageReturn);
    print_char(ascii::Char::LineFeed);
}


pub fn print_u16(n: u16) {
    if n == 0 {
        print_char(ascii::Char::Digit0);
    };
    
    let mut passed_zeroes = false;
    let mut coef = 10000;
    while coef != 0 {
        let digit = (n / coef % 10) as u8;
        coef /= 10;
        
        if digit == 0 && !passed_zeroes {
            continue;
        };
        
        passed_zeroes = true;
        
        print_char(ascii::Char::digit(digit).unwrap());
    };
}
