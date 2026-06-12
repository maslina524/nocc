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

fn read_from_regedit(buf: &mut [u8], key: HKEY, path: &str, name: &str) -> u32 {
    let mut wide_buf = [0u16; 512];
    let mut path_buf = [0u16; 256];
    utf8_to_utf16le(path, &mut path_buf);

    let mut hkey = 0;
    let status = unsafe {
        RegOpenKeyExW(key, path_buf.as_ptr(), 0, KEY_READ, &mut hkey)
    };
    if status != ERROR_SUCCESS {
        return 0;
    }

    let mut val_buf = [0u16; 256];
    utf8_to_utf16le(name, &mut val_buf);
    let mut dw_type = REG_SZ;
    let mut data_size = wide_buf.len() as u32 * 2;

    let result = unsafe {
        RegQueryValueExW(
            hkey,
            val_buf.as_ptr(),
            core::ptr::null_mut(),
            &mut dw_type,
            wide_buf.as_mut_ptr() as *mut u8,
            &mut data_size,
        )
    };
    unsafe { RegCloseKey(hkey); }

    if result == ERROR_SUCCESS && dw_type == REG_SZ {
        let wide_slice = &wide_buf[..data_size as usize / 2];
        utf16le_to_utf8(wide_slice, buf).try_into().unwrap()
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

pub fn get_gpu(buf: &mut [u8]) -> u32 {
    read_from_regedit(
        buf, 
        HKEY_LOCAL_MACHINE, 
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000", 
        "DriverDesc"
    )
}

pub struct Battery {
    pub level: u8,
    pub charging: bool
}

pub fn get_battery() -> Option<Battery> {
    let mut status = SYSTEM_POWER_STATUS {
        ACLineStatus: 0,
        BatteryFlag: 0,
        BatteryLifePercent: 0,
        Reserved1: 0,
        BatteryLifeTime: 0,
        BatteryFullLifeTime: 0,
    };

    let success = unsafe { GetSystemPowerStatus(&mut status) };
    if success == 0 {
        return None;
    }

    let flags = status.BatteryFlag;
    if flags & 128 != 0 || flags == 255 {
        None
    } else {
        Some(
            Battery { level: status.BatteryLifePercent, charging: flags & 8 == 1 }
        )
    }
}

#[derive(Default, Copy, Clone)]
pub struct Drive {
    pub name: [u8; 32],
    pub max: u64,
    pub busy: u64
}

pub fn get_drives(drives_buf: &mut [Drive]) -> usize {
    let mut raw_drives = [0u16; 256];
    unsafe { GetLogicalDriveStringsW(256, &mut raw_drives as *mut u16) };

    let mut drive_names = [[0u16; 32]; 32];
    let mut pos = 0;
    let mut cur_drive = [0u16; 32];
    let mut cur_drive_len = 0;
    let mut drives_len = 0;
    while pos < 256 {
        let ch = raw_drives[pos];
        if ch == 0 {
            if cur_drive_len != 0 {
                drive_names[drives_len] = cur_drive;
                cur_drive = [0u16; 32];
                cur_drive_len = 0;
                drives_len += 1;
            }
        } else {
            cur_drive[cur_drive_len] = ch;
            cur_drive_len += 1;
        }

        pos += 1;
    }

    for i in 0..drives_len {
        let drive_name = drive_names[i];
        let mut free_available = 0u64;
        let mut total_bytes = 0u64;
        let mut total_free_bytes = 0u64;

        unsafe { GetDiskFreeSpaceExW(
            drive_name.as_ptr() as *const u16, 
            &mut free_available as *mut u64, 
            &mut total_bytes as *mut u64, 
            &mut total_free_bytes as *mut u64
        ) };

        let mut drive_name_buf = [0u8; 32];
        utf16le_to_utf8(&drive_name, &mut drive_name_buf);
        let drive = Drive {
            name: drive_name_buf,
            max: total_bytes,
            busy: total_bytes - free_available
        };
        drives_buf[i] = drive;
    }

    drives_len
}