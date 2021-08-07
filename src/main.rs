#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/** custom entry point
	linker looks for _start by default 
    extern "C" -> use C calling convention
    return type ! -> diverging function, never returns
**/
#[no_mangle] // disable name mangling
pub extern "C" fn _start() -> ! {
    loop {}
}