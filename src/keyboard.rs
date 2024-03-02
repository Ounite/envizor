use core::{arch::asm, ascii};

use crate::display::text;


#[derive(PartialEq)]
pub struct Scancode {
    pub char: ascii::Char,
    pub modif: u8,
}


impl From<(u8, u8)> for Scancode {
    fn from(value: (u8, u8)) -> Self {
        Self { char: ascii::Char::from_u8(value.0).unwrap(), modif: value.0 }
    }
}


pub const ENTER_KEY_SCANCODE: Scancode = Scancode { char: ascii::Char::CarriageReturn, modif: 0x1c };


pub fn input_char<const VERBOSE: bool>() -> Scancode {
    let char_raw;
    let modif;
    unsafe {
        asm!(
            "int 0x16",
            in("ah") 0x00_u8,
            lateout("al") char_raw,
            lateout("ah") modif,
        );
    };

    let char = ascii::Char::from_u8(char_raw).unwrap();

    if VERBOSE {
        crate::display::text::print_char(char);
    };

    Scancode { char, modif }
}


pub fn await_key(scancode: Scancode) {
    while input_char::<false>() != scancode {};
}


pub fn expect_key(scancode: Scancode) -> bool {
    input_char::<false>() == scancode
}

pub fn input_line<const BUFFER_SIZE: usize>(verbose: bool) -> ([ascii::Char; BUFFER_SIZE], usize) {
    let mut buffer = [ascii::Char::Null; BUFFER_SIZE];

    let mut used = BUFFER_SIZE;
    for i in 0..BUFFER_SIZE {
        let scancode = input_char::<false>();

        match scancode {
            ENTER_KEY_SCANCODE => { used = i; break },
            Scancode { char: ascii::Char::Backspace, .. } => text::print_backspace(),
            scancode => {
                if verbose {
                    text::print_char(scancode.char);
                };

                buffer[i] = scancode.char;
            },
        };
    };

    (buffer, used)
}
