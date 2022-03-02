.set KERNBASE,    0x80000000
.set KSTACKSIZE,  4096
.set CR4_PSE,     0x00000010

# Multiboot header.  Data to direct multiboot loader.
.p2align 2
.text
.globl multiboot_header
multiboot_header:
    #define magic 0x1badb002
    #define flags 0
    .long magic
    .long flags
    .long (-magic-flags)


.globl _start
_start = entry - KERNBASE

.globl entry
entry:
    # Turn on page size extension for 4Mbyte pages
    movl    %cr4,       %eax
    orl     $(CR4_PSE), %eax
    movl    %eax,       %cr4 

    # Set page directory
    movl    $(entrypgdir - KERNBASE), %eax
    movl    %eax,       %cr3

    # Turn on paging
    movl    %cr0,       %eax
    orl     $0x80010000,%eax
    movl    %eax,       %cr0

    movl    $(stack + KSTACKSIZE),  %esp

    mov     $main,  %eax
    jmp     *%eax

.comm stack, KSTACKSIZE
