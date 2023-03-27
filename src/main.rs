#![no_std]
#![no_main]

use core::panic::PanicInfo;

// this function never returns
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Ensure that the compile outputs a function with that exact name
// extern "C" tells the compiler to use the C calling convention
//
// The function should never return because it is not invoked by another function,
// but the operating system or the bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}