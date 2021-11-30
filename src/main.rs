#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let buf = 0xb8000 as *mut u8;
    unsafe{
        *buf = 66;
        *buf.offset(1)=0xb;
    }

    loop {}
}
