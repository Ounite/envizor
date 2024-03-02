use core::{arch::asm, ascii};

// use super::mode;

// pub struct Position {
//     pub row: u8,
//     pub column: u8,
// }

// impl From<(u8, u8)> for Position {
//     fn from(pos: (u8, u8)) -> Self {
//         Self { row: pos.0, column: pos.1 }
//     }
// }

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

pub fn print_string(s: &str) -> Result<(), ()> {
    s.as_ascii().ok_or(())?.iter().for_each(|c| print_char(*c));
    Ok(())
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

// pub enum OverflowPrintStrategy {
//     Loop, Wrap, Cutoff
// }

// pub fn print_string_strategied(s: &str, attr: u8, overflow_strat: OverflowPrintStrategy) {
//     match overflow_strat {
//         OverflowPrintStrategy::Wrap => s.as_bytes().iter().for_each(|c| print_char(*c, attr)),
//         strat => {
//             let (reset_row, space_left) = {
//                 let total = match mode::VideoMode::from(unsafe { mode::CURRENT }.unwrap()) {
//                     mode::VideoMode::Defined { info, .. } => info.get_columns_count(),
//                     _ => unimplemented!("unrecognised video mode must not be used when using non-wrapping overflow strategy"),
//                 };
        
//                 let Position { column, row } = get_cursor_pos();
                
//                 (row, total - column)
//             };

//             s.as_bytes()[..(space_left as usize).min(s.as_bytes().len())].iter().for_each(|c| print_char(*c, attr));
            
//             let s_len = s.as_bytes().len();
//             if s_len > space_left as usize && let OverflowPrintStrategy::Loop = strat {
//                 set_cursor_pos((0, reset_row));
//                 s.as_bytes()[space_left as usize..].iter().for_each(|c| print_char(*c, attr));
//             };
//         }
//     };
// }

// pub fn print_string(s: &str, attr: u8) {
//     print_string_strategied(s, attr, OverflowPrintStrategy::Wrap)
// }

// pub fn print_string_offsetted(offset: u8, s: &str, attr: u8, overflow_strat: OverflowPrintStrategy) {
//     for _ in 0..offset {
//         print_char(' ' as u8, 0x00);
//     };

//     print_string_strategied(s, attr, overflow_strat)
// }

// pub fn print_u16(mut n: u16, attr: u8) {
//     if n == 0 {
//         print_char('0' as u8, attr);
//     };

//     let mut buffer = <[u8; 5]>::default();

//     let mut n_l = 0;
//     for i in 0..=5 {
//         if n == 0 {
//             n_l = i;
//             break;
//         };
     
//         buffer[i] = (n % 10 + 0x30) as u8;
//         n /= 10;
//     };

//     for i in (0..5).rev().skip(5-n_l).take(n_l) {
//         print_char(buffer[i], attr);
//     };
// }

// pub fn set_cursor_pos(pos: impl Into<Position>) {
//     let pos = pos.into();

//     unsafe {
//         asm!(
//             "int 0x10",
//             in("ah") 0x02_u8,
//             in("bh") 0x00_u8,
//             in("dl") pos.column,
//             in("dh") pos.row,
//         );
//     };
// }

// pub fn get_cursor_pos() -> Position {
//     let row;
//     let column;

//     unsafe {
//         asm!(
//             "int 0x10",
//             in("ah") 0x03_u8,
//             in("bh") 0x00_u8,
//             lateout("ch") _,
//             lateout("cl") _,
//             lateout("dh") row,
//             lateout("dl") column,
//         );
//     };

//     Position { row, column }
// }
