#[repr(C)]
struct Run<'a> {
    next: Option<&'a Run<'a>>,
}

struct Kmem<'a> {
    use_lock: bool,
    freelist: Option<&'a Run<'a>>,
}

static mut MEMORY: Kmem<'static> = Kmem{use_lock: false, freelist: None,};

pub fn freerange(vstart: usize, vend: usize)
{
    let pgsize = 4096;

    // Round vstart to upper pager boundary.
    let mut p = (vstart + pgsize - 1) & !(pgsize-1);

    while p + pgsize <= vend
    {
        unsafe{kfree(p);}

        p += pgsize;
    }
}

unsafe fn kfree(address:usize)
{
    // TODO: Add extra checks here.
    if address % 4096 != 0
    {
        panic!("kfree");
    }

    let page: *mut u8 = address as *mut u8;
    for i in 0..4096 {
        page.offset(i).write(1u8);
    }

    if MEMORY.use_lock
    {
        //TODO: Acquire memory lock
    }

    let r: &mut Run = &mut *(address as *mut Run);
    r.next = MEMORY.freelist;
    MEMORY.freelist = Some(r);

    if MEMORY.use_lock
    {
        //TODO: Release memory lock
    }
}
