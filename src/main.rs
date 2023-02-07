#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(error_os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use error_os_rust::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    error_os_rust::init();

    #[cfg(test)]
    test_main();

    println!("no crush");
    error_os_rust::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    error_os_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error_os_rust::test_panic_handler(info)
}
