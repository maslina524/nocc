use windows::types::HANDLE;
use windows::consts::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE, NULL};
use windows::link::{GetStdHandle, WriteConsoleW};

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

    pub fn print(&self, string: &str) {
        let mut buf = [0; 1024];
        let written_to_utf16 = windows::utils::utf8_to_utf16le(string, &mut buf) as u32;
        let mut written = 0u32;

        unsafe { 
            WriteConsoleW(self.stdout_handle, buf.as_ptr() as *const u16, written_to_utf16, &mut written, NULL);
        }
    }
}