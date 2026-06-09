use windows::link::GetEnvironmentVariableW;
use windows::utils::{utf8_to_utf16le, utf16le_to_utf8};

fn get_env(name: &str) -> [u16; 255] {
    let mut buf = [0; 255];
    let mut name_buf = [0; 255];
    let nsize = utf8_to_utf16le(name, &mut name_buf);

    unsafe { GetEnvironmentVariableW(name_buf.as_ptr() as *const u16, buf.as_ptr() as *mut u16, nsize as u32); }

    buf
}

pub fn get_user_name() -> [u8; 255] {
    let env_v = get_env("USERNAME");
    let mut buf = [0; 255];
    utf16le_to_utf8(&env_v, &mut buf);
    buf
}