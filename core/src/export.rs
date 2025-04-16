use std::io::{BufWriter, Write};

use crate::run::traits::Run;

pub struct Export;

impl Export {
    pub fn export_times<'a, I, T>(runs: I, writer: impl Write)
    where
        I: Iterator<Item = &'a T>,
        T: Run + 'a,
    {
        let mut buffered = BufWriter::new(writer);

        let _ = buffered.write(b"Name,Time,StampCount,IsWin\n");
        for run in runs {
            let _ = buffered.write(run.get_name().as_bytes());
            let _ = buffered.write(b",");
            let _ = buffered.write(run.get_time().to_string().as_bytes());
            let _ = buffered.write(b",");
            let _ = buffered.write(run.get_splits().count().to_string().as_bytes());
            let _ = buffered.write(b",");
            let _ = buffered.write(run.is_win().to_string().as_bytes());
            let _ = buffered.write(b",");

            for split in run.get_splits() {
                let _ = buffered.write(split.get_time().to_string().as_bytes());
                let _ = buffered.write(b",");
            }

            let _ = buffered.write(b"\n");
        }
    }
}
