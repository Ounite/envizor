use core::arch::asm;
use crate::display::text;

pub fn halt() -> ! {
    unsafe {
        asm!(
            "cli",
            "hlt",
            options(noreturn)
        )
    }
}


pub fn abort(reason: &str) -> ! {
    text::print_string("\r\n!!! ABORTING !!!\r\nreason: ");
    text::print_string(reason);
    halt()
}
