#![no_std]
#![no_main]
#![feature(asm)]

mod vga_buffer;
mod x86;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop{};
}

#[no_mangle]
pub fn _start() {
    print!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    loop{}
}
