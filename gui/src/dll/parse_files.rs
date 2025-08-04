use core::run::timed_run::LevelRun;
use std::{ffi::{c_char, c_void, CStr}, path::PathBuf, sync::{mpsc::Sender, Arc}};

use glr_core::run_gen_result::RunGeneratorResult;
use glr_lib::dll_exports::{
    enums::{SubscribeCode, SubscriptionType}, structs::CallbackInfo
};

pub fn parse_runs(file_paths: Vec<PathBuf>, sender: &Sender<LevelRun>) {
    let arc_sender: Arc<Sender<LevelRun>> = Arc::new(sender.clone());
    let context = Arc::into_raw(arc_sender) as *const c_void;

    let callback_info = CallbackInfo::new(
        SubscribeCode::RunInfo, 
        SubscriptionType::JSON, 
        0, 
        context.into(), 
        Some(callback)
    );
    
    glr_lib::dll_exports::functions::process_paths(file_paths, callback_info); 
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

