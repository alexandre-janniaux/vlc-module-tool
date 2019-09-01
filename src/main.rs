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
    const VLC_SUCCESS                       : c_int = 0x0       ;
    const VLC_MODULE_CREATE                 : c_int = 0x0       ;
    const VLC_CONFIG_CREATE                 : c_int = 0x1       ;
    const VLC_MODULE_CPU_REQUIREMENT        : c_int = 0x100     ;
    const VLC_MODULE_SHORTCUT               : c_int = 0x101     ;
    const VLC_MODULE_CAPABILITY             : c_int = 0x102     ;
    const VLC_MODULE_SCODE                  : c_int = 0x103     ;
    const VLC_MODULE_CB_OPEN                : c_int = 0x104     ;
    const VLC_MODULE_CB_CLOSE               : c_int = 0x105     ;
    const VLC_MODULE_NO_UNLOAD              : c_int = 0x106     ;
    const VLC_MODULE_NAME                   : c_int = 0x107     ;
    const VLC_MODULE_SHORTNAME              : c_int = 0x108     ;
    const VLC_MODULE_DESCRIPTION            : c_int = 0x109     ;
    const VLC_MODULE_HELP                   : c_int = 0x10A     ;
    const VLC_MODULE_TEXTDOMAIN             : c_int = 0x10B     ;
    const VLC_CONFIG_NAME                   : c_int = 0x1000    ;
    const VLC_CONFIG_VALUE                  : c_int = 0x1001    ;
    const VLC_CONFIG_RANGE                  : c_int = 0x1002    ;
    const VLC_CONFIG_ADVANCED_RESERVED      : c_int = 0x1003    ;
    const VLC_CONFIG_VOLATIVE               : c_int = 0x1004    ;
    const VLC_CONFIG_PERSISTENT_OBSOLETE    : c_int = 0x1005    ;
    const VLC_CONFIG_PRIVATE                : c_int = 0x1006    ;
    const VLC_CONFIG_REMOVED                : c_int = 0x1007    ;
    const VLC_CONFIG_CAPABILITY             : c_int = 0x1008    ;
    const VLC_CONFIG_SHORTCUT               : c_int = 0x1009    ;
    const VLC_CONFIG_OLDNAME_OBSOLETE       : c_int = 0x100A    ;
    const VLC_CONFIG_SAFE                   : c_int = 0x100B    ;
    const VLC_CONFIG_DESC                   : c_int = 0x100C    ;
    const VLC_CONFIG_LIST_OBSOLETE          : c_int = 0x100D    ;
    const VLC_CONFIG_ADD_ACTION_OBSOLETE    : c_int = 0x100E    ;
    const VLC_CONFIG_LIST                   : c_int = 0x100F    ;
    const VLC_CONFIG_LIST_CB_OBSOLETE       : c_int = 0x1010    ;

    #[no_mangle]
    extern "C" fn plugin_describe_cb(
        context: *mut c_void,
        target: *mut c_void,
        prop_id: c_int,
        args: VaList
    )  -> c_int
    {
        let kind = match prop_id
        {
            VLC_MODULE_CREATE       => "VLC_MODULE_CREATE",
            VLC_CONFIG_CREATE       => "VLC_CONFIG_CREATE",
            VLC_MODULE_SHORTCUT     => "VLC_MODULE_SHORTCUT",
            VLC_MODULE_CAPABILITY   => "VLC_MODULE_CAPABILITY",
            VLC_MODULE_SCORE        => "VLC_MODULE_SCORE",
            VLC_MODULE_CB_OPEN      => "VLC_MODULE_CB_OPEN",
            VLC_MODULE_CB_CLOSE     => "VLC_MODULE_CB_CLOSE",
            VLC_MODULE_NO_UNLOAD    => "VLC_MODULE_NO_UNLOAD",
            VLC_MODULE_NAME         => "VLC_MODULE_NAME",
            VLC_MODULE_SHORTNAME    => "VLC_MODULE_SHORTNAME",
            VLC_MODULE_DESCRIPTION  => "VLC_MODULE_DESCRIPTION",
            VLC_MODULE_HELP         => "VLC_MODULE_HELP",
            VLC_MODULE_TEXTDOMAIN   => "VLC_MODULE_TEXTDOMAIN",
            VLC_CONFIG_NAME         => "VLC_CONFIG_NAME",
            VLC_CONFIG_VALUE        => "VLC_CONFIG_VALUE",
            VLC_CONFIG_RANGE        => "VLC_CONFIG_RANGE",
            VLC_CONFIG_VOLATILE     => "VLC_CONFIG_VOLATILE",
            VLC_CONFIG_REMOVED      => "VLC_CONFIG_REMOVED",
            VLC_CONFIG_CAPABILITY   => "VLC_CONFIG_CAPABILITY",
            VLC_CONFIG_SHORTCUT     => "VLC_CONFIG_SHORTCUT",
            VLC_CONFIG_SAFE         => "VLC_CONFIG_SAFE",
            VLC_CONFIG_DESC         => "VLC_CONFIG_DESC",
            VLC_CONFIG_LIST         => "VLC_CONFIG_LIST",
            _ => "Unknown module property",
        };

        println!("Got property \"{}\"", kind);

        VLC_SUCCESS
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
