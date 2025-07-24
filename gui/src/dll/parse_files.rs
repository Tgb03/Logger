use core::run::timed_run::LevelRun;
use std::{ffi::{c_char, c_void, CStr, CString}, path::PathBuf, sync::{mpsc::Sender, Arc}};


use crate::dll::{callback::Code, exported_data::RunGeneratorResult, functions::GTFO_API};


pub fn parse_runs(file_paths: Vec<PathBuf>, sender: &Sender<LevelRun>) {
    let arc_sender: Arc<Sender<LevelRun>> = Arc::new(sender.clone());
    let context = Arc::into_raw(arc_sender) as *const c_void;
        
    let paths: Vec<CString> = file_paths.iter()
        .map(|v| v.to_string_lossy())
        .filter_map(|v| CString::new(v.as_ref()).ok())
        .collect();

    let paths_ptr: Vec<*const i8> = paths.iter()
        .map(|v| v.as_ptr())
        .collect();

    unsafe {
        (GTFO_API.process_paths)(paths_ptr.as_ptr(), paths.len() as u32, Code::RunInfo as u8, 1, context, callback);
    }
}


extern "C" fn callback(context: *const c_void, message: *const c_char) {
    if message.is_null() || context.is_null() {
        eprintln!("Null pointer in callback");
        return;
    }

    unsafe {
        let arc: Arc<Sender<LevelRun>> = Arc::from_raw(context as *const Sender<LevelRun>);
        let sender = arc.clone();
        std::mem::forget(arc);

        let c_str = CStr::from_ptr(message);
        if let Ok(json_str) = c_str.to_str() {
            if let Ok(RunGeneratorResult::LevelRun(run)) = serde_json::from_str::<RunGeneratorResult>(json_str) {
                let converted = run.into();
                let _ = sender.send(converted);
            }
        }
    }
}

