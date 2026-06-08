#![no_std]
#![no_main]

mod io;
use core::panic::PanicInfo;
use io::Io;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let io = Io::new();
    io.print("Hello World!\n");
    0
}