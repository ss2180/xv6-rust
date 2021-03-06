use crate::kalloc;
use crate::x86;
use lazy_static::lazy_static;

const KERNBASE: usize = 0x80000000;
const EXTMEM: usize = 0x100000;
const KERNLINK: usize = KERNBASE + EXTMEM;
const DEVSPACE: usize = 0xFE000000;
const PHYSTOP: usize = 0xE000000;

macro_rules! V2P {
    ($a:expr) => {{
        $a - 0x80000000usize // subtract KERNBASE
    }};
}

macro_rules! P2V {
    ($a:expr) => {{
        $a + 0x80000000usize // add KERNBASE
    }};
}

extern "C" {
    static data: u8;
}

struct Kmap {
    virt: usize,
    phys_start: usize,
    phys_end: usize,
    perm: usize,
}

lazy_static! {
    static ref KMAP: [Kmap; 4] = unsafe {
        [
            Kmap {
                virt: KERNBASE,
                phys_start: 0,
                phys_end: EXTMEM,
                perm: 2,
            },
            Kmap {
                virt: KERNLINK,
                phys_start: V2P!(KERNLINK),
                phys_end: V2P!((&data as *const u8) as usize),
                perm: 0,
            },
            Kmap {
                virt: (&data as *const u8) as usize,
                phys_start: V2P!((&data as *const u8) as usize),
                phys_end: PHYSTOP,
                perm: 2,
            },
            Kmap {
                virt: DEVSPACE,
                phys_start: DEVSPACE,
                phys_end: 0xFFFFFFFFusize,
                perm: 2,
            },
        ]
    };
}

lazy_static! {
    static ref KPGDIR: usize = unsafe { setupkvm() };
}

pub unsafe fn kvmalloc() {
    println!(
        "Kmap1: Virt: {:x}, PS: {:x}, PE {:x}, PERM: {:x}",
        KMAP[0].virt, KMAP[0].phys_start, KMAP[0].phys_end, KMAP[0].perm
    );
    println!(
        "Kmap2: Virt: {:x}, PS: {:x}, PE {:x}, PERM: {:x}",
        KMAP[1].virt, KMAP[1].phys_start, KMAP[1].phys_end, KMAP[1].perm
    );
    println!(
        "Kmap3: Virt: {:x}, PS: {:x}, PE {:x}, PERM: {:x}",
        KMAP[2].virt, KMAP[2].phys_start, KMAP[2].phys_end, KMAP[2].perm
    );
    println!(
        "Kmap4: Virt: {:x}, PS: {:x}, PE {:x}, PERM: {:x}",
        KMAP[3].virt, KMAP[3].phys_start, KMAP[3].phys_end, KMAP[3].perm
    );

    let test = *KPGDIR;
    x86::lcr3(V2P!(test));
}

unsafe fn setupkvm() -> usize {
    let pgdir;
    match kalloc::kalloc() {
        Some(page) => pgdir = page as *mut u8,
        None => return 0,
    }

    for i in 0..4096 {
        pgdir.offset(i).write(0u8);
    }

    for i in 0..3 {
        mappages(
            pgdir as usize,
            KMAP[i].virt,
            KMAP[i].phys_end - KMAP[i].phys_start,
            KMAP[i].phys_start,
            KMAP[i].perm,
        );
    }

    pgdir as usize
}

unsafe fn mappages(pgdir: usize, va: usize, size: usize, mut pa: usize, perm: usize) -> usize {
    let pgsize = 4096;

    let mut address = va & !(pgsize - 1);
    let last_address = (va + size - 1) & !(pgsize - 1);

    while address != last_address {
        let pte = walkpgdir(pgdir, address, true);
        *(pte as *mut usize) = pa | perm | 0x1;

        address = address + pgsize;
        pa = pa + pgsize;
    }

    0usize
}

unsafe fn walkpgdir(pgdir: usize, va: usize, alloc: bool) -> usize {
    let pde_offset = ((va >> 22) & 0x3FF) as isize;
    let pte_offset = ((va >> 12) & 0x3FF) as isize;

    let pde = (pgdir as *mut usize).offset(pde_offset);
    let pgtab;

    if (*pde & 1) == 1 {
        pgtab = (P2V!(*pde & !0xFFF)) as *mut usize;
    } else {
        //println!("PAGE TABLE NOT PRESENT");

        match kalloc::kalloc() {
            Some(page) => pgtab = page as *mut usize,
            None => return 0,
        }

        if !alloc {
            return 0;
        }
        for i in 0..1024 {
            pgtab.offset(i).write(0usize);
        }

        *pde = V2P!(pgtab as usize) | 0x1 | 0x2 | 0x4;
    }

    pgtab.offset(pte_offset) as usize
}
