#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic. Must be reimplemented since we are not using std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Instead of a main function (needs runtime), we write our own start function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
