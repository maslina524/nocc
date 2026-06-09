#![no_std]
#![no_main]

mod io;
mod logos;

use core::panic::PanicInfo;
use io::Io;
use logos::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let io = Io::new();

    // CREATE LOGO BUFFER
    let mut logo_buf = [""; LINES];
    let logo = WIN11;
    for (i, line) in logo.iter().enumerate() {
        logo_buf[i] = line;
    }

    // CREATE INFO BUFFER
    let mut info_buf = [""; LINES];
    info_buf[0] = "Hello";
    info_buf[1] = "Windows";

    // PRINT BUFFERS
    for i in 0..LINES {
        let string = logo_buf[i];

        io.print(string);
        for _ in string.len()..LINE_LEN {
            io.print(" ");
        }
        io.print(info_buf[i]);
        io.print("\n");
    }

    0
}