#![no_std]
#![no_main]

mod io;
mod logos;
mod os;

#[cfg(not(test))]
use core::panic::PanicInfo;
use io::Io;
use logos::*;
use windows::types::RTL_OSVERSIONINFOW;

use crate::os::{Battery, get_battery};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let io = Io::new();
    io.print("panicked\n");
    loop {}
}

const RED_ANSI: &str = "\x1b[1;31m";
const YELLOW_ANSI: &str = "\x1b[1;32m";
const GREEN_ANSI: &str = "\x1b[1;33m";
const BLUE_ANSI: &str = "\x1b[1;34m";

const ESC_ANSI: &str = "\x1b[0m";
const LINE_LEN_WITH_INDENT: usize = 55;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let io = Io::new();
    let osvi = os::get_os_ver_win();

    // CREATE LOGO BUFFER
    let mut logo_buf = [""; LINES];
    let mut logo = match (osvi.dwMajorVersion, osvi.dwMinorVersion, osvi.dwBuildNumber) {
        (6, 1, _) => WIN7,
        (6, 2 | 3, _) => WIN10_8,
        (10, 0, build) if build >= 22000 => WIN7,
        (10, 0, build) if build >= 10240 => WIN10_8,
        _ => WIN10_8
    };
    let mut temp_buf = [0u8; 1024];
    to_ansi(&mut temp_buf, logo);
    logo = unsafe { core::str::from_utf8_unchecked(&temp_buf) };
    for (i, line) in logo.lines().enumerate() {
        logo_buf[i] = line;
    }

    // CREATE INFO BUFFER
    let mut info_buf = [""; LINES];
    let mut buf_pos = 0;

    // USER NAME@PC NAME
    let mut temp_buf = [0; 255];
    let owner = owner_as_str(&mut temp_buf);
    info_buf[buf_pos] = owner;
    buf_pos += 1;

    // SPLITTER
    info_buf[buf_pos] = "---------------";
    buf_pos += 1;

    // OS NAME
    let mut temp_buf = [0u8; 256];
    let os_str = os_ver_as_str(&mut temp_buf, osvi);
    info_buf[buf_pos] = os_str;
    buf_pos += 1;

    // CPU
    let mut temp_buf = [0u8; 256];
    let cpu_str = cpu_as_str(&mut temp_buf);
    info_buf[buf_pos] = cpu_str;
    buf_pos += 1;

    // GPU
    let mut temp_buf = [0u8; 256];
    let gpu_str = gpu_as_str(&mut temp_buf);
    info_buf[buf_pos] = gpu_str;
    buf_pos += 1;

    // BATTERY
    let mut temp_buf = [0u8; 256];
    if let Some(b) = get_battery() {
        let gpu_str = battery_as_str(&mut temp_buf, b);
        info_buf[buf_pos] = gpu_str;
        buf_pos += 1;
    }

    // PRINT BUFFERS
    for i in 0..LINES {
        let string = logo_buf[i];

        io.print(string);
        for _ in visible_width(string)..LINE_LEN_WITH_INDENT {
            io.print(" ");
        }
        io.print(ESC_ANSI);
        io.print(info_buf[i]);
        io.print("\n");
    }

    io.print(ESC_ANSI);
    0
}

fn visible_width(s: &str) -> usize {
    let mut in_escape = false;
    let mut width = 0;
    for b in s.as_bytes() {
        if *b == b'\x1b' {
            in_escape = true;
        } else if in_escape && *b == b'm' {
            in_escape = false;
        } else if !in_escape {
            width += 1;
        }
    }
    width
}

pub fn to_ansi<'a>(buf: &mut [u8], string: &'a str) {
    let mut pos = 0;
    let mut spec = false;
    for ch in string.chars() {
        if spec {
            let r = match ch {
                '1' => Some(BLUE_ANSI),
                '2' => Some(RED_ANSI),
                '3' => Some(GREEN_ANSI),
                '4' => Some(YELLOW_ANSI),
                _ => None
            };

            if let Some(c) = r {
                pos += paste_to_buf(buf, c.as_bytes(), pos)
            } else {
                buf[pos - 1] = '$' as u8;
                buf[pos] = ch as u8;
            };

            spec = false;
            pos += 1;
            continue;
        }

        if ch == '$' {
            spec = true;
        } else {
            buf[pos] = ch as u8;
        }
        pos += 1;
    }
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

fn cpu_as_str<'a>(temp_buf: &'a mut [u8]) -> &'a str {
    let mut pos = 0;
    pos += paste_to_buf(temp_buf, "\x1b[38;2;255;165;0mCPU: \x1b[0m".as_bytes(), pos);

    let mut buf = [0u8; 256];
    let len: u32 = os::get_cpu(&mut buf);
    pos += paste_to_buf(temp_buf, &buf[..len as usize], pos);

    unsafe { core::str::from_utf8_unchecked(&temp_buf[..pos]) }
} 

fn gpu_as_str<'a>(temp_buf: &'a mut [u8]) -> &'a str {
    let mut pos = 0;
    pos += paste_to_buf(temp_buf, "\x1b[38;2;255;165;0mGPU: \x1b[0m".as_bytes(), pos);

    let mut buf = [0u8; 256];
    let len: u32 = os::get_gpu(&mut buf);
    pos += paste_to_buf(temp_buf, &buf[..len as usize], pos);

    unsafe { core::str::from_utf8_unchecked(&temp_buf[..pos]) }
} 

fn battery_as_str<'a>(temp_buf: &'a mut [u8], battery: Battery) -> &'a str {
    let mut pos = 0;
    pos += paste_to_buf(temp_buf, "\x1b[38;2;255;165;0mBATTERY: \x1b[0m".as_bytes(), pos);

    let mut buf = [0u8; 10];
    pos += paste_to_buf(temp_buf, u32_to_str(battery.level as u32, &mut buf).as_bytes(), pos);

    if battery.charging {
        pos += paste_to_buf(temp_buf, "% [AC Connected]".as_bytes(), pos);
    } else {
        pos += paste_to_buf(temp_buf, "%".as_bytes(), pos);
    }

    unsafe { core::str::from_utf8_unchecked(&temp_buf[..pos]) }
} 