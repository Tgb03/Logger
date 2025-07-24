use std::{ffi::{c_char, c_void, CStr}, sync::{atomic::AtomicU32, mpsc::{self, Receiver, Sender}, Arc}};

use serde::Deserialize;

use crate::dll::functions::GTFO_API;

static CHANNEL_ID_COUNT: AtomicU32 = AtomicU32::new(1);


pub struct ContinousParser<T>
where 
    T: for<'a> Deserialize<'a> {

    recv: Receiver<T>,

    channel_id: u32,
    code: u8,

}

impl<T> ContinousParser<T>
where 
    T: for<'a> Deserialize<'a> {

    pub fn new(code: u8) -> Self {
        let (sender, recv) = mpsc::channel();
        let channel_id = CHANNEL_ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let arc_sender: Arc<Sender<T>> = Arc::new(sender.clone());
        let context = Arc::into_raw(arc_sender) as *const c_void;

        unsafe {
            (GTFO_API.add_callback)(code, 1, channel_id, context, callback::<T>)
        }

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
        unsafe { (GTFO_API.remove_callback)(self.code, self.channel_id); }
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
