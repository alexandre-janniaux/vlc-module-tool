#![feature(c_variadic)]
use dylib::DynamicLibrary;

use std::env;
use std::path::Path;


mod vlc
{
    use std::ffi::{VaList};
    use libc::{c_void, c_int, c_char};
    pub type SetFunc   = unsafe extern fn (*mut c_void, *mut c_void, c_int, VaList) -> c_int;
    pub type EntryFunc = unsafe extern fn (SetFunc, *mut c_void) -> c_int;
    pub type MetaExportFunc = unsafe extern fn () -> *const c_char;
}

fn main()
{
    let filename = env::args()
        .skip(1).next()
        .expect("Usage: vlc-module-tool <filename>");

    let filepath = Path::new(&filename);

    println!("Opening library {}", filename);

    let plugin = DynamicLibrary::open(Some(filepath))
        .expect(&format!("Could not open library {}", filename));
}
