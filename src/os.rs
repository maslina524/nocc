use windows::consts::*;
use windows::link::*;
use windows::types::*;
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

pub fn get_os_ver_win() -> RTL_OSVERSIONINFOW {
    let mut osvi = RTL_OSVERSIONINFOW {
        dwOSVersionInfoSize: core::mem::size_of::<RTL_OSVERSIONINFOW>() as ULONG,
        dwMajorVersion: 0,
        dwMinorVersion: 0,
        dwBuildNumber: 0,
        dwPlatformId: 0,
        szCSDVersion: [0; 128],
    };
    unsafe { RtlGetVersion(&mut osvi); }
    osvi
}

fn read_from_regedit(buf: &mut [u8], hkey_path: HKEY, path: &str, name: &str) -> u32 {
    let mut hkey: HKEY = 0;
    let mut path_buf = [0u16; 256];
    utf8_to_utf16le(path, &mut path_buf);

    let status = unsafe {
        RegOpenKeyExW(
            hkey_path,
            path_buf.as_ptr(),
            0,
            KEY_READ,
            &mut hkey,
        )
    };
    if status != ERROR_SUCCESS {
        return 0;
    }

    let mut value_name_buf = [0u16; 256];
    utf8_to_utf16le(name, &mut value_name_buf);
    let mut dw_type = REG_SZ;
    let mut data_size = buf.len() as u32;

    let result = unsafe {
        RegQueryValueExW(
            hkey,
            value_name_buf.as_ptr(),
            core::ptr::null_mut(),
            &mut dw_type,
            buf.as_mut_ptr(),
            &mut data_size,
        )
    };

    unsafe { RegCloseKey(hkey); }

    if result == ERROR_SUCCESS && dw_type == REG_SZ {
        data_size
    } else {
        0
    }
}

pub fn get_cpu(buf: &mut [u8]) -> u32 {
    read_from_regedit(
        buf, 
        HKEY_LOCAL_MACHINE, 
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0", 
        "ProcessorNameString"
    )
}