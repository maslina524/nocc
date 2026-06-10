use core::ptr;
use core::ffi::c_void;

use crate::types::*;

pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;

pub const NULL: *const c_void = ptr::null();
pub const ERROR_SUCCESS: i64 = 0;

pub const COMPUTER_NAME_DNS_HOSTNAME: NAME_TYPE = 1;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002;
pub const KEY_READ: u32 = 0x20019;
pub const REG_SZ: DWORD = 1;  