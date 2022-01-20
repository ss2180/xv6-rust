#![no_std]
#![no_main]
#![feature(asm)]
use core::panic::PanicInfo;

#[repr(C)]
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

#[repr(C)]
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
    asm!("in al,dx", out("al") data, in("dx") port);
    return data;
}

unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("al") data, in("dx") port);
}

unsafe fn insl(port: u32, addr: *const u8, cnt: u16) {
    asm!("cld");
    asm!("rep insl", in("edi") addr, in("ecx") cnt, in("dx") port, options(att_syntax));
}

unsafe fn stosb(addr: *const u8, data: u8, cnt: u16) {
    asm!("cld");
    asm!("rep stosb", in("edi") addr, in("ecx") cnt, in("al") data, options(att_syntax));
}

unsafe fn waitdisk() {
    while (inb(0x1F7) & 0xC0) != 0x40 {
    }
}

unsafe fn readsect(dst: *const u8, offset: u32) {
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
    insl(0x1F0, dst, 128);
}

fn readseg(mut pa: *mut u8, count: u32, mut offset: u32) {

    static SECTSIZE: u32 = 512;
    let epa: *mut u8;
    //let mut pa_ptr : *mut u8 = pa;

    unsafe{
        epa = pa.add(count as usize);

        // Round down to sector boundary
        pa = pa.sub((offset % SECTSIZE) as usize);

        offset = (offset / SECTSIZE) + 1;

        while pa < epa
        {
            readsect(pa, offset);

            pa = pa.add(SECTSIZE as usize);
            offset = offset + 1;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{};
}

fn fail() -> !{
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

    let elf: *mut u8 = 0x10000 as *mut u8;

    readseg(elf, 4096, 0);

    unsafe { 
        
        let elf_ref: &Elfhdr = core::mem::transmute::<*mut u8, &Elfhdr>(elf);

        if elf_ref.magic != 0x464C457F
        {
            fail();
        }

        let mut ph: *mut u8 = elf.add(elf_ref.phoff as usize);
        let mut ph_ref: & Proghdr = core::mem::transmute::<*mut u8, &Proghdr>(ph);
        let eph: *mut u8 = ph.add((elf_ref.phnum * elf_ref.phentsize) as usize);
        let mut i = 0;
        while ph < eph
        {

            let pa: *mut u8 = ph_ref.paddr as *mut u8;
            readseg(pa, ph_ref.filesz, ph_ref.offset);

            if ph_ref.memsz > ph_ref.filesz {
                stosb(pa.add(ph_ref.filesz as usize), 0, (ph_ref.memsz - ph_ref.filesz) as u16);
            }

            ph = ph.add(elf_ref.phentsize as usize);

            ph_ref = core::mem::transmute::<*mut u8, &Proghdr>(ph);

            i = i + 1;
        }
        let entry = elf_ref.entry as *const ();
        let code: fn() = core::mem::transmute::<*const (), fn()>(entry);
        (code)();
    }
    
    

    loop {}
}
