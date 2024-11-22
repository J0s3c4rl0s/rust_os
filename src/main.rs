#![no_std] // tell it not to link stdlib
#![no_main] // no main function because main needs stdlib
use core::panic::PanicInfo;

#[no_mangle] // dont mangle function so that linker finds the start function
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler] // required to give a "custom" panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}