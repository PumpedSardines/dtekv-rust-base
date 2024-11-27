#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod boot;
mod io;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn handle_interrupt() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn handle_exception() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    io::uart::print("[PANIC]: ");
    let file = info.location().unwrap().file();
    let line = info.location().unwrap().line();

    match info.message().as_str() {
        Some(message) => io::uart::print(message),
        None => io::uart::print("no message"),
    }

    io::uart::print(" at ");
    io::uart::print(file);
    io::uart::printc(b'\n');

    loop {}
}
