use windows::consts::COMPUTER_NAME_DNS_HOSTNAME;
use windows::link::{GetComputerNameExW, GetEnvironmentVariableW};
use windows::utils::{utf8_to_utf16le, utf16le_to_utf8};

fn get_env(name: &str) -> [u16; 256] {
    let mut buf = [0; 256];
    let mut name_buf = [0; 256];
    let nsize = utf8_to_utf16le(name, &mut name_buf);

    unsafe { GetEnvironmentVariableW(name_buf.as_ptr() as *const u16, buf.as_mut_ptr(), nsize as u32); }

    buf
}

pub fn get_user_name(buf: &mut [u8]) -> usize {
    let env_v = get_env("USERNAME");
    utf16le_to_utf8(&env_v, buf)
}

pub fn get_pc_name(buf: &mut [u8]) -> usize {
    let mut pc_buf = [0u16; 256];
    let mut dword = 256u32;
    unsafe { GetComputerNameExW(COMPUTER_NAME_DNS_HOSTNAME, pc_buf.as_mut_ptr(), &mut dword); }
    utf16le_to_utf8(&pc_buf, buf)
}