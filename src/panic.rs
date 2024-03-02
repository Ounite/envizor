use core::panic;


#[panic_handler]
pub fn panic(info: &panic::PanicInfo) -> ! {
    loop {}
}
