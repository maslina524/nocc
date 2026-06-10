pub type STD_HANDLE = u32;
pub type HANDLE = *mut core::ffi::c_void;

pub type DWORD = u32;
pub type ULONG = u32;
pub type WCHAR = u16;
pub type BOOL = i32;

pub type PCSTR = *const u8;
pub type PWSTR = *mut u16;
pub type PCWSTR = *const u16;
pub type LPTSTR = *mut u16;
pub type LPDWORD = *mut u32;
pub type LPCWSTR = *const u16;

pub type NAME_TYPE = u32;
pub type HKEY = u32;
pub type PHKEY = *const HKEY;
pub type REGSAM = u32;
pub type NTSTATUS = u32;
pub type LSTATUS = i64;

#[repr(C)]
#[derive(Debug)]
pub struct RTL_OSVERSIONINFOW {
    pub dwOSVersionInfoSize: ULONG,
    pub dwMajorVersion: ULONG,
    pub dwMinorVersion: ULONG,
    pub dwBuildNumber: ULONG,
    pub dwPlatformId: ULONG,
    pub szCSDVersion: [WCHAR; 128],
}