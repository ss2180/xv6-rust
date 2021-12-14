#![no_std]
#![no_main]
#![feature(asm)]
use core::panic::PanicInfo;

struct Elfhdr {
    magic: u32,
    elf: [u8; 12],
    filetype: u16,
    machine: u16,
    version: u16,
    entry: u32,
    phoff: u32,
    shoff: u32,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16
}

struct Proghdr {
    segtype: u32,
    offset: u32,
    vaddr: u32,
    paddr: u32,
    filesz: u32,
    memsz: u32,
    flags: u32,
    align: u32
}

unsafe fn inb(port: u16) -> u8 {
    let mut data : u8;
    asm!("in al, dx", out("al") data, in("dx") port);
    return data;
}

unsafe fn outb(port: u16, data: u8) {
    asm!("out al,dx", in("al") data, in("dx") port);
}

unsafe fn insl(port: u32, addr: *mut u16, cnt: u32) {
    asm!("cld");
    asm!("rep insl", );
}

unsafe fn waitdisk() {
    while (inb(0x1F7) & 0xC0) != 0x40 {
    }
}

unsafe fn readsect(dst: *mut u8, offset: u32) {
    // Issue command.
    waitdisk();
    outb(0x1F2, 1);   // count = 1
    outb(0x1F3, offset as u8);
    outb(0x1F4, (offset >> 8) as u8);
    outb(0x1F5, (offset >> 16) as u8);
    outb(0x1F6, (offset >> 24) as u8 | 0xE0);
    outb(0x1F7, 0x20);  // cmd 0x20 - read sectors

    // Read data.
    waitdisk();
}

fn readseg(pa: &mut u8, count: u32, mut offset: u32) {

    static SECTSIZE: u32 = 512;
    let epa: *mut u8;
    let mut pa_ptr : *mut u8 = pa;

    unsafe{
        epa = pa_ptr.add(count as usize);

        // Round down to sector boundary
        pa_ptr = pa_ptr.sub((offset % SECTSIZE) as usize);

        offset = (offset / SECTSIZE) + 1;

        while pa_ptr < epa
        {
            // Readsect

            pa_ptr = pa_ptr.add(SECTSIZE as usize);
            offset = offset + 1;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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

#[no_mangle]
pub extern "C" fn bootmain() -> ! {

    unsafe{
        inb(0x64);
    }

    loop {}
}
