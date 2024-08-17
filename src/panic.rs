use core::panic;


#[panic_handler]
pub fn panic(_info: &panic::PanicInfo) -> ! {
    loop {}
}
