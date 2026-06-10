pub type STD_HANDLE = u32;
pub type HANDLE = *mut core::ffi::c_void;

pub type BOOL = i32;

pub type PCSTR = *const u8;
pub type PWSTR = *mut u16;
pub type PCWSTR = *const u16;
pub type LPTSTR = *mut u16;
pub type LPDWORD = *mut u32;

pub type NAME_TYPE = u32;