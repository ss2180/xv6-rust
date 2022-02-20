#![no_std]
#![no_main]
#![feature(asm)]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{};
}

#[no_mangle]
pub fn _start() {
    vga_buffer::print_something();
    loop{}
}
