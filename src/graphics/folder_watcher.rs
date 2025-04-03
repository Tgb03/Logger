use std::{fs, path::PathBuf, sync::mpsc::{channel, Receiver, Sender}, thread, time::Duration};

use might_sleep::prelude::CpuLimiter;


pub struct FolderWatcher;

impl FolderWatcher {

    pub fn new_watcher(folder_path: PathBuf) -> Receiver<PathBuf> {
        let (sender, recv) = channel::<PathBuf>();

        thread::spawn(|| Self::watch(folder_path, sender));

        recv
    }

    fn watch(folder_path: PathBuf, sender: Sender<PathBuf>) {
        let mut limiter = CpuLimiter::new(Duration::from_secs(5));
        let mut last_path = None;
        
        loop {
            // not using notify cause of issues with large folders just in case
            let path = fs::read_dir(&folder_path)
                .expect("Couldn't access local directory")
                .flatten()
                .filter(|f| {
                    let metadata = match f.metadata() {
                    Ok(metadata) => metadata,
                    Err(_) => { return false; },
                    };

                    metadata.is_file() && f.file_name().to_str().unwrap_or_default().contains("NICKNAME_NETSTATUS")
                })
                .max_by_key(|x| {
                    match x.metadata() {
                    Ok(metadata) => metadata.modified().ok(),
                    Err(_) => Default::default(),
                    }
                })
                .map(|v| v.path());

            if path != last_path {
                if let Some(path) = path {
                    match sender.send(path.clone()) {
                        Ok(_) => {},
                        Err(_) => break,
                    }
                    println!("File sent");
                    last_path = Some(path);
                }
            }
        
            limiter.might_sleep();
        }
    }

}
