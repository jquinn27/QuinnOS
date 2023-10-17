#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(quinn_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use quinn_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    quinn_os::init();

    #[cfg(test)]
    test_main();

    println!("No crash");

    quinn_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    quinn_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    quinn_os::test_panic_handler(info)
}
