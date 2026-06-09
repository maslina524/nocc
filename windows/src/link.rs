use crate::types::*;

// macros have been extracted from the std library
macro_rules! link_raw_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[cfg_attr(not(target_arch = "x86"), link(name = $library, kind = "raw-dylib", modifiers = "+verbatim"))]
        #[cfg_attr(target_arch = "x86", link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated"))]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

macro_rules! link_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        // Note: the windows-targets crate uses a pre-built Windows.lib import library which we don't
        // have in this repo. So instead we always link kernel32.lib and add the rest of the import
        // libraries below by using an empty extern block. This works because extern blocks are not
        // connected to the library given in the #[link] attribute.
        #[link(name = "kernel32")]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[cfg(feature = "windows_raw_dylib")]
macro_rules! link {
    ($($tt:tt)*) => (
        link_raw_dylib!($($tt)*);
    )
}

#[cfg(not(feature = "windows_raw_dylib"))]
macro_rules! link {
    ($($tt:tt)*) => (
        link_dylib!($($tt)*);
    )
}

link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> HANDLE);
link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : HANDLE, lpbuffer : PCWSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> BOOL);
link!("kernel32.dll" "system" fn GetEnvironmentVariableW(lpname : PCWSTR, lpbuffer : PWSTR, nsize : u32) -> u32);