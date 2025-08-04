use std::{ffi::{c_char, c_void, CStr}, sync::{atomic::AtomicU32, mpsc::{self, Receiver, Sender}, Arc}};

use glr_lib::dll_exports::{enums::{SubscribeCode, SubscriptionType}, structs::CallbackInfo};
use serde::Deserialize;

static CHANNEL_ID_COUNT: AtomicU32 = AtomicU32::new(1);


pub struct ContinousParser<T>
where 
    T: for<'a> Deserialize<'a> {

    recv: Receiver<T>,

    channel_id: u32,
    code: SubscribeCode,

}

impl<T> ContinousParser<T>
where 
    T: for<'a> Deserialize<'a> {

    pub fn new(code: SubscribeCode) -> Self {
        let (sender, recv) = mpsc::channel();
        let channel_id = CHANNEL_ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let arc_sender: Arc<Sender<T>> = Arc::new(sender.clone());
        let context = Arc::into_raw(arc_sender) as *const c_void;

        glr_lib::dll_exports::functions::add_callback(
            CallbackInfo::new(
                code as SubscribeCode, 
                SubscriptionType::JSON, 
                channel_id, 
                context.into(), 
                Some(callback::<T>)
            )
        );

        Self {
            recv,
            channel_id,
            code,
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        self.recv.try_recv().ok()
    }

}

impl<T> Drop for ContinousParser<T>
where 
    T: for<'a> Deserialize<'a> {
    
    fn drop(&mut self) {
        glr_lib::dll_exports::functions::remove_callback(self.code, self.channel_id);
    }
}

extern "C" fn callback<T>(context: *const c_void, message: *const c_char)
where
    T: for<'a> Deserialize<'a> {
    if message.is_null() || context.is_null() {
        eprintln!("Null pointer in callback");
        return;
    }

    unsafe {
        let arc: Arc<Sender<T>> = Arc::from_raw(context as *const Sender<T>);
        let sender = arc.clone();
        std::mem::forget(arc);

        let c_str = CStr::from_ptr(message);
        if let Ok(json_str) = c_str.to_str() {
            if let Ok(run) = serde_json::from_str::<T>(json_str) {
                let _ = sender.send(run.into());
            }
        }
    }
}
