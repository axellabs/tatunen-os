#![no_std] // unlink the rust standard library
#![no_main] // disable all rust-level entry points
extern crate rlibc;

use core::panic::PanicInfo;

// This function is called on panic. Must be reimplemented since we are not using std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Instead of a main function (needs runtime), we write our own start function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}