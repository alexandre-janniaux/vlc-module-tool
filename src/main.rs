#![feature(c_variadic)]

use dlopen::symbor::{Library, Symbol, SymBorApi};

use std::env;
use std::path::Path;

mod vlc
{
    use std::ffi::{VaList, CStr};
    use libc::{c_void, c_uint, c_int, c_char};
    use dlopen::symbor::{Symbol, SymBorApi};
    pub type SetFunc   = unsafe extern "C" fn (*mut c_void, *mut c_void, c_int, ...) -> c_int;
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

        pub fn visit(&self)
        {
            unsafe
            {
                (self.sym_entry)(plugin_describe_cb, std::ptr::null_mut());
            }
        }
    }

    const VLC_SUCCESS                       : c_int = 0x0       ;
    const VLC_MODULE_CREATE                 : c_int = 0x0       ;
    const VLC_CONFIG_CREATE                 : c_int = 0x1       ;
    const VLC_MODULE_CPU_REQUIREMENT        : c_int = 0x100     ;
    const VLC_MODULE_SHORTCUT               : c_int = 0x101     ;
    const VLC_MODULE_CAPABILITY             : c_int = 0x102     ;
    const VLC_MODULE_SCORE                  : c_int = 0x103     ;
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

    #[derive(Debug)]
    enum PluginProperty
    {
        ModuleCreate,
        ConfigCreate,
        ModuleCpuRequirement,
        ModuleShortcut(Vec<String>),
        ModuleCapability(String),
        ModuleScore,
        ModuleCallbackOpen,
        ModuleCallbackClose,
        ModuleNoUnload,
        ModuleName(String),
        ModuleShortname,
        ModuleDescription,
        ModuleHelp,
        ModuleTextDomain,
        ConfigName,
        ConfigValue,
        ConfigRange,
        ConfigAdvancedReserved,
        ConfigVolatile,
        ConfigPersistentObsolete,
        ConfigPrivate,
        ConfigRemoved,
        ConfigCapability,
        ConfigShortcut,
        ConfigOldNameObsoleted,
        ConfigSafe,
        ConfigDesc,
        ConfigListObsolete,
        ConfigAddActionObsolete,
        ConfigList,
        ConfigListCbObsolete,
    }

    #[no_mangle]
    unsafe extern "C" fn plugin_describe_cb(
        context: *mut c_void,
        target: *mut c_void,
        prop_id: c_int,
        mut args: ...
    )  -> c_int
    {
        let kind = match prop_id
        {
            VLC_MODULE_CREATE       => Some(PluginProperty::ModuleCreate),
            VLC_CONFIG_CREATE       => Some(PluginProperty::ConfigCreate),
            VLC_MODULE_SHORTCUT     => {
                let shortcut_count = args.arg::<c_uint>();
                let shortcuts = std::slice::from_raw_parts(
                    args.arg::<*const *const c_char>(),
                    shortcut_count as usize)
                    .iter()
                    .map(|&shortcut| CStr::from_ptr(shortcut).to_string_lossy().into())
                    .collect();

                Some(PluginProperty::ModuleShortcut(shortcuts))
            },
            VLC_MODULE_CAPABILITY   => {
                let capability = CStr::from_ptr(args.arg::<*const c_char>())
                    .to_string_lossy()
                    .into();
                Some(PluginProperty::ModuleCapability(capability))
            }
            VLC_MODULE_SCORE        => Some(PluginProperty::ModuleScore),
            VLC_MODULE_CB_OPEN      => Some(PluginProperty::ModuleCallbackOpen),
            VLC_MODULE_CB_CLOSE     => Some(PluginProperty::ModuleCallbackClose),
            VLC_MODULE_NO_UNLOAD    => Some(PluginProperty::ModuleNoUnload),
            VLC_MODULE_NAME         => {
                let name = CStr::from_ptr(args.arg::<*const c_char>())
                    .to_string_lossy();
                Some(PluginProperty::ModuleName(name.into()))
            },
            VLC_MODULE_SHORTNAME    => Some(PluginProperty::ModuleShortname),
            VLC_MODULE_DESCRIPTION  => Some(PluginProperty::ModuleDescription),
            VLC_MODULE_HELP         => Some(PluginProperty::ModuleHelp),
            VLC_MODULE_TEXTDOMAIN   => Some(PluginProperty::ModuleTextDomain),
            VLC_CONFIG_NAME         => Some(PluginProperty::ConfigName),
            VLC_CONFIG_VALUE        => Some(PluginProperty::ConfigValue),
            VLC_CONFIG_RANGE        => Some(PluginProperty::ConfigRange),
            VLC_CONFIG_VOLATILE     => Some(PluginProperty::ConfigVolatile),
            VLC_CONFIG_REMOVED      => Some(PluginProperty::ConfigRemoved),
            VLC_CONFIG_CAPABILITY   => Some(PluginProperty::ConfigCapability),
            VLC_CONFIG_SHORTCUT     => Some(PluginProperty::ConfigShortcut),
            VLC_CONFIG_SAFE         => Some(PluginProperty::ConfigSafe),
            VLC_CONFIG_DESC         => Some(PluginProperty::ConfigDesc),
            VLC_CONFIG_LIST         => Some(PluginProperty::ConfigList),
            _ => None,
        };

        if let Some(property) = kind
        {
            println!(" + property \"{:?}\"", property);
        }
        else
        {
            println!(" x unknown property");
        }

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

    plugin_interface.visit();
}
