#![no_std]	
#![no_main]
use core::panic::PanicInfo;

//called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//on start
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    vga_buffer::print_s();
    loop {}
}

mod vga_buffer;
