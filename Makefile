xv6_rust.iso: xv6-rust.bin
	grub-mkrescue -o ./target/xv6_rust.iso isodir

xv6-rust.bin: xv6-rust boot.o
	i686-elf-ld -T linker.ld -o ./isodir/boot/xv6-rust.bin ./target/i686-xv6_rust/debug/boot.o ./target/i686-xv6_rust/debug/libxv6_rust.a

xv6-rust: src/main.rs
	cargo build

boot.o: src/main.s
	i686-elf-as -o ./target/i686-xv6_rust/debug/boot.o ./src/main.s

qemu:
	make xv6_rust.iso
	qemu-system-i386 -cdrom ./target/xv6_rust.iso