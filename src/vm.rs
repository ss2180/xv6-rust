use lazy_static::lazy_static;

const KERNBASE: usize = 0x80000000;
const EXTMEM: usize = 0x100000; 
const KERNLINK: usize = KERNBASE + EXTMEM;
const DEVSPACE: usize = 0xFE000000;
const PHYSTOP: usize = 0xE000000;

macro_rules! V2P{
    ($a:expr)=>{
        {
            $a - KERNBASE
        }
    }
}

macro_rules! P2V{
    ($a:expr)=>{
        {
            $a + KERNBASE
        }
    }
}

extern "C" {
    static data: *mut u8;
}

#[repr(C)]
struct Kmap<'a> {
    virt: &'a mut u8,
    phys_start: usize,
    phys_end: usize,
    perm: u16,
}

lazy_static!{
    static ref KMAP: [Kmap<'static>; 4] =  [Kmap {virt: &mut *(KERNBASE as *mut u8), phys_start:0, phys_end:EXTMEM, perm:2},
                                  Kmap {virt: &mut *(KERNLINK as *mut u8), phys_start:V2P!(KERNLINK), phys_end:V2P!(data as usize), perm:0},
                                  Kmap {virt: &mut *data, phys_start:V2P!(data as usize), phys_end:PHYSTOP, perm:2},
                                  Kmap {virt: &mut *(DEVSPACE as *mut u8), phys_start:DEVSPACE, phys_end:0, perm:2}];
}

pub fn kvmalloc()
{
    unsafe {
        
    }
}
