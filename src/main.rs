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

    /* In VLC 3, these symbols were:
     *  - vlc_entry_license__3_0_0f
     *  - vlc_entry_copyright__3_0_0f
     *  - vlc_entry__3_0_0f  */
    unsafe {
        let api_version = plugin.symbol::<vlc::MetaExportFunc>("vlc_entry_api_version")
            .unwrap();
        let entry       = plugin.symbol::<vlc::EntryFunc>("vlc_entry")
            .unwrap();
        let copyright   = plugin.symbol::<vlc::MetaExportFunc>("vlc_entry_copyright")
            .unwrap();
        let license     = plugin.symbol::<vlc::MetaExportFunc>("vlc_entry_license")
            .unwrap();
    }
}
