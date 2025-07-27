use std::ffi::{c_char, c_void};
use libloading::os::windows::{Library, Symbol};

use once_cell::sync::Lazy;

#[cfg(not(debug_assertions))]
#[cfg(target_arch="x86_64")]
static MY_DLL: &[u8] = include_bytes!("../../../resources/gtfo_log_reader_core_64bit.dll");

#[cfg(not(debug_assertions))]
#[cfg(target_arch="x86")]
static MY_DLL: &[u8] = include_bytes!("../../../resources/gtfo_log_reader_core_32bit.dll");

// Define the function pointer types
pub type EventCallback = extern "C" fn(context: *const c_void, message: *const c_char);

pub type StartListenerFn = unsafe extern "C" fn(file_path: *const c_char);
pub type AddCallbackFn = unsafe extern "C" fn(
    code: u8,
    message_type: u8,
    channel_id: u32,
    callback_context: *const c_void,
    event_callback_ptr: EventCallback,
);
pub type RemoveCallbackFn = unsafe extern "C" fn(code: u8, channel_id: u32);
pub type ProcessPathsFn = unsafe extern "C" fn(
    paths: *const *const c_char,
    len: u32,
    code: u8,
    message_type: u8,
    callback_context: *const c_void,
    event_callback_ptr: EventCallback,
);

// Struct to hold all dynamically loaded functions
pub struct GtfoLogReader {
    _lib: Library, // Keep the library alive as long as you use the functions
    pub start_listener: Symbol<StartListenerFn>,
    pub add_callback: Symbol<AddCallbackFn>,
    pub remove_callback: Symbol<RemoveCallbackFn>,
    pub process_paths: Symbol<ProcessPathsFn>,
}

impl GtfoLogReader {
    pub fn new(lib_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(lib_path)?;

            Ok(Self {
                start_listener: lib.get::<StartListenerFn>(b"start_listener")?,
                add_callback: lib.get::<AddCallbackFn>(b"add_callback")?,
                remove_callback: lib.get::<RemoveCallbackFn>(b"remove_callback")?,
                process_paths: lib.get::<ProcessPathsFn>(b"process_paths")?,
                _lib: lib,
            })
        }
    }
}

pub static GTFO_API: Lazy<GtfoLogReader> = Lazy::new(|| {

    #[cfg(debug_assertions)]
    let lib = unsafe { Library::new("resources/gtfo_log_reader.dll") }.expect("Failed to load DLL");

    #[cfg(not(debug_assertions))]
    let lib = unsafe {
        use std::{env, fs::File, io::Write};

        let mut path = env::temp_dir();
        path.push("gtfo_log_reader.dll");

        let mut file = File::create(&path).unwrap();
        let _ = file.write_all(MY_DLL);
        let _ = file.flush();
        drop(file);

        Library::new(path).expect("Failed to load DLL")
    };

    // SAFETY: we store lib in a `Box` to keep it alive
    Box::leak(Box::new(lib));

    GtfoLogReader::new("gtfo_log_reader.dll").unwrap()
});