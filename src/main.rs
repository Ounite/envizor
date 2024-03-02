#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![no_std]
#![no_main]


mod display;
mod keyboard;
mod panic;
mod control;

use display::text;
use keyboard as kb;


// #[no_mangle]
// extern fn main() -> ! {
    // display::mode::set(display::mode::TEXT_COLOR4_80X25);

    // for offset in (0..80).cycle() {
    //     // text::set_cursor_pos((12, 0));
    //     text::print_string_offsetted(offset, "Hello, this is a rolling text test.", 0x07, text::OverflowPrintStrategy::Loop);
    //     for _ in 0..2 { unsafe { core::arch::asm!("hlt") }; };
    //     display::mode::set(display::mode::TEXT_COLOR4_80X25);
    // };

    // control::halt();
// }


#[no_mangle]
extern fn main() -> ! {
    text::print_string("Raahhh!!").unwrap();

    control::halt()
}
