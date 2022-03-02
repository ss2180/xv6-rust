#![no_std]
#![no_main]
#![feature(asm)]

mod vga_buffer;
mod x86;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //println!("{}", _info);

    loop{};
}

#[no_mangle]
#[used]
pub static mut entrypgdir: u32 = 50;

#[no_mangle]
pub fn main() {

    print!("Hello, World!");

    loop{}
}
