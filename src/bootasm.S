.code16                       # Assemble for 16-bit mode
.globl start
start:
    cli                         # BIOS enabled interrupts; disable

    # Zero data segment registers DS, ES, and SS.
    xorw    %ax,%ax             # Set %ax to zero
    movw    %ax,%ds             # -> Data Segment
    movw    %ax,%es             # -> Extra Segment
    movw    %ax,%ss             # -> Stack Segment

    # Physical address line A20 is tied to zero so that the first PCs 
    # with 2 MB would run software that assumed 1 MB.  Undo that.
seta20.1:
    inb     $0x64,%al               # Wait for not busy
    testb   $0x2,%al
    jnz     seta20.1

    movb    $0xd1,%al               # 0xd1 -> port 0x64
    outb    %al,$0x64

seta20.2:
    inb     $0x64,%al               # Wait for not busy
    testb   $0x2,%al
    jnz     seta20.2

    movb    $0xdf,%al               # 0xdf -> port 0x60
    outb    %al,$0x60

    # Switch from real to protected mode.  Use a bootstrap GDT that makes
    # virtual addresses map directly to physical addresses so that the
    # effective memory map doesn't change during the transition.
    lgdt    gdtdesc
    movl    %cr0, %eax
    orl     $0x1, %eax  # 0x1 = Protected Mode Enable bit
    movl    %eax, %cr0

    # Complete the transition to 32-bit protected mode by using a long jmp
    # to reload %cs and %eip.  The segment descriptors are set up with no
    # translation, so that the mapping is still the identity mapping.

    ljmp    $0x8, $start32 # Pass value of 8 offset 8 bytes to code segment in GDT

.code32  # Tell assembler to generate 32-bit code now.
start32:
    # Set up the protected-mode data segment registers
    movw    $0x10, %ax               # Our data segment selector
    movw    %ax, %ds                # -> DS: Data Segment
    movw    %ax, %es                # -> ES: Extra Segment
    movw    %ax, %ss                # -> SS: Stack Segment
    movw    $0, %ax                 # Zero segments not ready for use
    movw    %ax, %fs                # -> FS
    movw    %ax, %gs                # -> GS

    # Set up the stack pointer and call into C.
    movl    $start, %esp # NOTE: Why set the stack pointer to value of start?
    call    bootmain

    # If bootmain returns (it shouldn't),  loop.
spin:
    jmp     spin



# Bootstrap GDT
.p2align 2                                # force 4 byte alignment
gdt:
    # null seg
    .word 0, 0
    .byte 0, 0, 0, 0
 
    # code seg
    .word 0xffff, 0x0000
    .byte 0x00, 0x9a, 0xcf, 0x00
   
    # data seg
    .word 0xffff, 0x0000
    .byte 0x00, 0x92, 0xcf, 0x00

gdtdesc:
    .word   (gdtdesc - gdt - 1)             # sizeof(gdt) - 1
    .long   gdt                             # address gdt

