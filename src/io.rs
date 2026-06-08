use windows::types::HANDLE;
use windows::consts::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
use windows::link::GetStdHandle;

pub struct Io {
    stdout_handle: HANDLE,
    stderr_handle: HANDLE,
    input_handle: HANDLE
}

impl Io {
    pub fn new() -> Io {
        let stdout_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        let stderr_handle = unsafe { GetStdHandle(STD_ERROR_HANDLE) };
        let input_handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

        Io { stdout_handle, stderr_handle, input_handle }
    }
}