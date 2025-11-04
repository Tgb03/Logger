use core::run::timed_run::LevelRun;
use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::AtomicUsize,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
};

use egui::ProgressBar;

use crate::{dll::parse_files::parse_runs, render::Render};

const MAX_THREAD: usize = 8;

pub struct AwaitParseFiles {
    runs_collected: Vec<LevelRun>,
    receiver: Receiver<LevelRun>,

    left: Arc<AtomicUsize>,
    len: usize,
    frames: usize,

    join_handle: Vec<JoinHandle<()>>,
}

impl AwaitParseFiles {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        let (sender, recv) = mpsc::channel();
        let len = paths.len();
        let left = Arc::new(AtomicUsize::new(len));

        let paths = Arc::new(Mutex::new(paths.into_iter()));
        let mut threads = Vec::with_capacity(MAX_THREAD);

        for _ in 0..MAX_THREAD {
            let paths_clone = paths.clone();
            let sender_clone = sender.clone();
            let left_clone = left.clone();

            threads.push(thread::spawn(move || {
                loop {
                    let mut files: Vec<PathBuf> = Vec::with_capacity(5);
                    let mut guard = match paths_clone.lock() {
                        Ok(g) => g,
                        Err(_) => {
                            paths_clone.clear_poison();
                            continue;
                        }
                    };

                    for _ in 0..5 {
                        if let Some(path) = guard.next() {
                            files.push(path);
                        } else {
                            break;
                        }
                    }

                    drop(guard);

                    if files.is_empty() {
                        return;
                    }

                    let len_parsed = files.len();
                    parse_runs(files, &sender_clone);
                    left_clone.fetch_sub(len_parsed, std::sync::atomic::Ordering::Relaxed);
                }
            }));
        }

        Self {
            receiver: recv,
            runs_collected: Vec::new(),
            left,
            len,
            frames: 0,
            join_handle: threads,
        }
    }

    pub fn get_left(&self) -> usize {
        self.left.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_frames(&self) -> usize {
        self.frames
    }

    pub fn is_done(&self) -> bool {
        self.get_left() == 0
    }

    pub fn collect(&mut self) {
        while let Ok(r) = self.receiver.try_recv() {
            self.runs_collected.push(r);
        }
    }
}

impl Render for AwaitParseFiles {
    type Response = bool;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.vertical_centered(|ui| {
            ui.label(format!(
                "Files left to parse: {} out of {}",
                self.get_left(),
                self.get_len()
            ));
            ui.label(format!(
                "Files/frame: {:.2}",
                (self.get_len() - self.get_left()) as f64 / self.get_frames() as f64
            ));
            ui.label(format!(
                "Percentage Done: {:.2}%",
                (self.get_len() - self.get_left()) as f64 * 100.0 / self.get_len() as f64
            ));

            ui.add(ProgressBar::new(
                (self.get_len() - self.get_left()) as f32 / self.get_len() as f32,
            ));
        });

        self.frames += 1;
        self.collect();
        if self.is_done() {
            for jh in self.join_handle.drain(0..self.join_handle.len()) {
                let _ = jh.join();
            }
            self.collect();

            return true;
        }

        false
    }
}

impl Into<Vec<LevelRun>> for AwaitParseFiles {
    fn into(self) -> Vec<LevelRun> {
        self.runs_collected
    }
}
