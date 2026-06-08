use core::ptr;
use core::ffi::c_void;

use crate::types::*;

pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;

pub const NULL: *const c_void = ptr::null();