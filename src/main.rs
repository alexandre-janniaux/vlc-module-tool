#![feature(c_variadic)]

use dlopen::symbor::{Library, Symbol, SymBorApi};

use std::env;
use std::path::Path;

mod vlc
{
    use std::ffi::{VaList, CStr};
    use libc::{c_void, c_int, c_char};
    use dlopen::symbor::{Symbol, SymBorApi};
    pub type SetFunc   = unsafe extern "C" fn (*mut c_void, *mut c_void, c_int, VaList) -> c_int;
    pub type EntryFunc = unsafe extern "C" fn (SetFunc, *mut c_void) -> c_int;
    pub type MetaExportFunc = unsafe extern "C" fn () -> *const c_char;

    #[derive(dlopen_derive::SymBorApi)]
    pub struct PluginEntry<'a>
    {
        #[dlopen_name="vlc_entry_api_version"]
        sym_api_version: Symbol<'a, MetaExportFunc>,
        #[dlopen_name="vlc_entry"]
        sym_entry: Symbol<'a, EntryFunc>,
        #[dlopen_name="vlc_entry_copyright"]
        sym_copyright: Symbol<'a, MetaExportFunc>,
        #[dlopen_name="vlc_entry_license"]
        sym_license: Symbol<'a, MetaExportFunc>,
    }

    /**
     * Safe wrappers around dll-defined symbols
     **/
    impl<'a> PluginEntry<'a>
    {
        /**
         * Return the version of libvlccore which the loaded plugin was compiled against.
         **/
        pub fn api_version(&self) -> String
        {
            unsafe
            {
                CStr::from_ptr((self.sym_api_version)())
                    .to_string_lossy()
                    .into()
            }
        }
    }

}


fn main()
{
    let filename = env::args()
        .skip(1).next()
        .expect("Usage: vlc-module-tool <filename>");

    println!("Opening library {}", filename);

    let plugin = Library::open(&filename)
        .expect(&format!("Could not open library {}", filename));

    /* In VLC 3, these symbols were:
     *  - vlc_entry_license__3_0_0f
     *  - vlc_entry_copyright__3_0_0f
     *  - vlc_entry__3_0_0f  */
    let mut plugin_interface = unsafe { vlc::PluginEntry::load(&plugin) }.unwrap();

    let api_version = plugin_interface.api_version();

    println!("LibVLCCore Version: {}", api_version);
}
