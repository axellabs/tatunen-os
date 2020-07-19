#![no_std] // unlink the rust standard library
#![no_main] // disable all rust-level entry points

extern crate rlibc;

use core::panic::PanicInfo;

mod vga_buffer;

// Instead of a main function (needs runtime), we write our own start function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("This is dragon's operating system{}", "!");

    loop {}
}

// This function is called on panic. Must be reimplemented since we are not using std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}