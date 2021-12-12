#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn bootmain() -> ! {
    let buf = 0xb8000 as *mut u8;
    let mut i : u8 = 0;
    loop{
    i = i + 1;
    unsafe{
        *buf = i;
        *buf.offset(1)=i;
    }
}
}