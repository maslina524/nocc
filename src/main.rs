#![no_std]
#![no_main]

mod io;
mod logos;
mod os;

use core::panic::PanicInfo;
use io::Io;
use logos::*;
use os::get_user_name;

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
    paste_to_buf(&mut cur_buf, "\x1b[1;34m".as_bytes(), 0);
    let mut pos = "\x1b[1;34m".len();

    let name = get_user_name();
    let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    paste_to_buf(&mut cur_buf, &name[..name_len], pos);
    pos += name_len;

    let string = "\x1b[0m@\x1b[1;34m";
    paste_to_buf(&mut cur_buf, string.as_bytes(), pos);
    pos += string.len();

    paste_to_buf(&mut cur_buf, "PC_NAME".as_bytes(), pos);
    info_buf[0] = unsafe { str::from_utf8_unchecked(&cur_buf) };

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

fn paste_to_buf(buf: &mut [u8], bytes: &[u8], index: usize) {
    for (i, ch) in bytes.iter().enumerate() {
        buf[index + i] = ch.clone();
    }
}