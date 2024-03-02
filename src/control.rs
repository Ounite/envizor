use core::arch::asm;

pub fn halt() -> ! {
    unsafe {
        asm!(
            "cli",
            "hlt",
            options(noreturn)
        )
    }
}
