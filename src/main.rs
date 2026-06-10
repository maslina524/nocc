#![no_std]
#![no_main]

mod io;
mod logos;
mod os;

use core::panic::PanicInfo;
use io::Io;
use logos::*;
use windows::types::RTL_OSVERSIONINFOW;

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
    let mut temp_buf = [0; 255];
    let owner = owner_as_str(&mut temp_buf);
    info_buf[0] = owner;

    // SPLITTER
    info_buf[1] = "---------------";

    // OS NAME
    let osvi = os::get_os_ver_win();
    let mut temp_buf = [0u8; 256];
    let os_str = os_ver_as_str(&mut temp_buf, osvi);
    info_buf[2] = os_str;

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

fn u32_to_str<'a>(mut n: u32, buffer: &'a mut [u8; 10]) -> &'a str {
    let mut idx = buffer.len();
    
    if n == 0 {
        buffer[idx - 1] = b'0';
        idx -= 1;
    } else {
        while n > 0 {
            idx -= 1;
            buffer[idx] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    }

    let bytes = &buffer[idx..];
    unsafe { core::str::from_utf8_unchecked(bytes) }
}

fn owner_as_str<'a>(temp_buf: &'a mut [u8]) -> &'a str {
    let mut pos = "\x1b[1;34m".len();

    pos += paste_to_buf(temp_buf, "\x1b[1;34m".as_bytes(), pos);
    let name_len = os::get_user_name(&mut temp_buf[pos..]);
    pos += name_len;

    pos += paste_to_buf(temp_buf, "\x1b[0m@\x1b[1;34m".as_bytes(), pos);
    let pc_len = os::get_pc_name(&mut temp_buf[pos..]);
    pos += pc_len;

    unsafe { core::str::from_utf8_unchecked(&temp_buf[..pos]) }
}

fn os_ver_as_str<'a>(temp_buf: &'a mut [u8], osvi: RTL_OSVERSIONINFOW) -> &'a str {
    let mut pos = 0;
    pos += paste_to_buf(temp_buf, "\x1b[38;2;255;165;0mOS: \x1b[0m".as_bytes(), pos);

    let name = if cfg!(target_os = "windows") {
        "Windows "
    } else if cfg!(target_os = "linux") {
        "Linux "
    } else if cfg!(target_os = "macos") {
        "MacOS "
    } else {
        "Unknown "
    };
    pos += paste_to_buf(temp_buf, name.as_bytes(), pos);

    let build = if cfg!(target_os = "windows") {
        match (osvi.dwMajorVersion, osvi.dwMinorVersion, osvi.dwBuildNumber) {
            (6, 1, 7600) => "7 RTM",
            (6, 1, 7601) => "7 SP1",
            (6, 2, 9200) => "8",
            (6, 3, 9600) => "8.1",
            (10, 0, build) if build >= 22000 => "11",
            (10, 0, build) if build >= 10240 => "10",
            _ => "Unknown",
        }
    } else { "" };
    pos += paste_to_buf(temp_buf, build.as_bytes(), pos);
    pos += paste_to_buf(temp_buf, " (".as_bytes(), pos);

    let mut num_buf = [0u8; 10];
    pos += paste_to_buf(temp_buf, u32_to_str(osvi.dwMajorVersion, &mut num_buf).as_bytes(), pos);
    pos += paste_to_buf(temp_buf, ".".as_bytes(), pos);
    pos += paste_to_buf(temp_buf, u32_to_str(osvi.dwMinorVersion, &mut num_buf).as_bytes(), pos);
    pos += paste_to_buf(temp_buf, ".".as_bytes(), pos);
    pos += paste_to_buf(temp_buf, u32_to_str(osvi.dwBuildNumber, &mut num_buf).as_bytes(), pos);
    pos += paste_to_buf(temp_buf, ")".as_bytes(), pos);

    unsafe { core::str::from_utf8_unchecked(&temp_buf[..pos]) }
}