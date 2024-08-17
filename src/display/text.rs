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


pub fn print_u16(mut n: u16) {
    // TODO size-optimize following same logic as in boot
    if n == 0 {
        print_char(ascii::Char::Digit0);
    };

    let mut buffer = <[ascii::Char; 5]>::default();

    let mut n_l = 0;
    for i in 0..=5 {
        if n == 0 {
            n_l = i;
            break;
        };
     
        buffer[i] = ascii::Char::digit((n % 10) as u8).unwrap();
        n /= 10;
    };

    for i in (0..5).rev().skip(5-n_l).take(n_l) {
        print_char(buffer[i]);
    };
}
