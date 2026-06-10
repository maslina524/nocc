#![no_std]
#![no_main]

mod io;
mod logos;
mod os;

use core::panic::PanicInfo;
use io::Io;
use logos::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let io = Io::new();
    io.print("panicked\n");
    if let Some(message) = info.message().as_str() {
        io.print(message);
    }
    loop {}
}

const ESC_ANSII: &str = "\x1b[0m";
const LINE_LEN_WITH_INDENT: usize = 55;

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

    // USER NAME@PC NAME
    let mut cur_buf = [0; 255];
    let mut pos = "\x1b[1;34m".len();

    pos += paste_to_buf(&mut cur_buf, "\x1b[1;34m".as_bytes(), pos);
    let name_len = os::get_user_name(&mut cur_buf[pos..]);
    pos += name_len;
    pos += paste_to_buf(&mut cur_buf, "\x1b[0m@\x1b[1;34m".as_bytes(), pos);
    let pc_len = os::get_pc_name(&mut cur_buf[pos..]);
    pos += pc_len;

    let info_line = core::str::from_utf8(&cur_buf[..pos]).unwrap_or("Invalid UTF-8");
    info_buf[0] = info_line;

    // PRINT BUFFERS
    for i in 0..LINES {
        let string = logo_buf[i];

        io.print(string);
        for _ in string.len()..LINE_LEN_WITH_INDENT {
            io.print(" ");
        }
        io.print(ESC_ANSII);
        io.print(info_buf[i]);
        io.print("\n");
    }

    io.print(ESC_ANSII);
    0
}

fn paste_to_buf(buf: &mut [u8], bytes: &[u8], index: usize) -> usize {
    for (i, ch) in bytes.iter().enumerate() {
        buf[index + i] = *ch;
    }
    bytes.len()
}