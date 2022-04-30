pub unsafe fn outb(port: u16, data: u8) {
    asm!(
    "out dx, al",
    in("al") data,
    in("dx") port,
    );
}

pub unsafe fn inb(port: u16) -> u8 {
    let res: u8;

    asm!(
    "in al, dx",
    in("dx") port,
    out("al") res,
    );

    res
}
