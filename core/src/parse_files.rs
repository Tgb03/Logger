pub mod file_parse {

    const MAX_THREAD: usize = 8;

    use std::marker::PhantomData;
    use std::path::PathBuf;
    use std::sync::mpsc::{self, Receiver};
    use std::sync::{Arc, Mutex};
    use std::thread::{self, JoinHandle};
    #[cfg(debug_assertions)]
    use std::time::Instant;
    use std::{fs::File, io::Read};

    use crate::logs::parser::{Parser, ParserResult};
    use crate::logs::token_parser::TokenParserT;
    use crate::logs::tokenizer::{GenericTokenizer, RunTokenizer, Tokenizer};

    pub struct AwaitParseFiles<T>
    where 
        T: From<ParserResult> {

        thread_join: JoinHandle<()>,
        receiver: Receiver<(ParserResult, usize)>,
        length: usize,
        left: usize,
        frame_count: usize,

        built_result: ParserResult,

        phantom: PhantomData<T>,

    }

    impl<T> AwaitParseFiles<T>
    where
        T: From<ParserResult> {
        
        pub fn new(paths: Vec<PathBuf>) -> Self {

            let (sender, recv) = mpsc::channel();
            let length = paths.len();

            let thread_join = thread::spawn(move || {
                let paths = Arc::new(Mutex::new(paths.into_iter()));

                let mut threads = Vec::new();

                for _ in 0..MAX_THREAD {
                    let paths_clone = paths.clone();
                    let sender_clone = sender.clone();

                    threads.push(thread::spawn(move || {
                        let tokenizer = GenericTokenizer::default().add_tokenizer(RunTokenizer);
                        
                        loop {
                            let mut result = ParserResult::default();
                            let mut p = 0;

                            for _ in 0..5 {
                                p += 1;
                                let file = match paths_clone.lock() {
                                    Ok(mut p_id) => p_id.next(),
                                    Err(_) => {
                                        let _ = sender_clone.send((result, p));
                                        
                                        return;
                                    }
                                };

                                match file {
                                    Some(path) => result.merge_result(parse_file(&path, &tokenizer)),
                                    None => {
                                        let _ = sender_clone.send((result, p));

                                        return;
                                    },
                                };
                            }

                            match sender_clone.send((result, p)) {
                                Ok(_) => {},
                                Err(_) => return,
                            }
                        }
                    }));
                }
                
                for thread in threads {
                    let _ = thread.join();
                }
            });

            Self {
                length,
                left: length,
                receiver: recv,
                frame_count: 0,
                thread_join,
                built_result: ParserResult::default(),
                phantom: PhantomData
            }
        }

        pub fn update(&mut self) {
            while let Ok((res, len)) = self.receiver.try_recv() {
                self.built_result.merge_result(res);
                self.left = self.left.saturating_sub(len);
            }
            self.frame_count += 1;
        }

        pub fn get_left(&self) -> usize {
            self.left
        }

        pub fn get_len(&self) -> usize {
            self.length
        }
        
        pub fn is_done(&self) -> bool {
            self.thread_join.is_finished()
        }

        pub fn get_frames(&self) -> usize {
            self.frame_count
        }

    }

    impl<T> Into<ParserResult> for AwaitParseFiles<T>
    where
        T: From<ParserResult> {
        
        fn into(self) -> ParserResult {
            let _ = self.thread_join.join();

            self.built_result
        }
    }

    pub fn parse_all_files_async(paths: Vec<PathBuf>) -> ParserResult {
        #[cfg(debug_assertions)]
        let start = Instant::now();

        let thread_count = MAX_THREAD.min(paths.len() / 12 + 1);

        let paths = Arc::new(Mutex::new(paths.into_iter()));
        let mut threads = Vec::new();

        for _ in 0..thread_count {
            let paths_clone = paths.clone();

            threads.push(thread::spawn(move || {
                let mut result = ParserResult::default();
                let tokenizer = GenericTokenizer::default().add_tokenizer(RunTokenizer);
                let mut count = 0u32;

                loop {
                    let file = match paths_clone.lock() {
                        Ok(mut p_id) => p_id.next(),
                        Err(_) => {
                            println!("Thread crashed with {} files parsed.", count);
                            return result;
                        }
                    };

                    match file {
                        Some(file) => result.merge_result(parse_file(&file, &tokenizer)),
                        None => {
                            println!("Thread ended correctly with {} files parsed.", count);
                            return result;
                        }
                    }

                    count += 1;
                }
            }));
        }

        let mut result = ParserResult::default();
        for thread in threads {
            match thread.join() {
                Ok(res) => result.merge_result(res),
                Err(_) => {}
            }
        }

        #[cfg(debug_assertions)]
        {
            let duration = start.elapsed();
            println!("Time elapsed with threads is: {:?}", duration);
        }

        result
    }

    pub fn parse_all_files<'a, I>(paths: I) -> ParserResult
    where
        I: IntoIterator<Item = &'a PathBuf>,
    {
        let mut result: ParserResult = Default::default();

        let tokenizer = GenericTokenizer::default().add_tokenizer(RunTokenizer);

        // let start = Instant::now();
        // let mut counter = 0;
        for path in paths {
            // counter += 1;
            result.merge_result(parse_file(path, &tokenizer));
        }
        // let duration = start.elapsed();
        //println!("Parsed without threads {} files in: {:?}", counter, duration);

        result
    }

    fn parse_file(path: &PathBuf, tokenizer: &impl Tokenizer) -> ParserResult {
        let mut data = String::new();
        let file = File::open(path);
        if let Ok(mut file) = file {
            let res = file.read_to_string(&mut data);
            if res.is_err() {
                return Default::default();
            }
        }

        let tokens = tokenizer.tokenize(&data);
        Parser::parse_all_tokens_default(tokens.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use fs::File;
    use std::{fs, io::Write, path::PathBuf, thread::sleep, time::Duration};
    use tempfile::{TempDir, tempdir};

    use crate::{
        logs::parser::ParserResult, run::{
            objectives::run_objective::RunObjective,
            traits::{Run, Timed},
        }, time::Time
    };

    use super::*;

    const TEXT: &str = "00:00:00.000 - <color=#C84800>SelectActiveExpedition : Selected! Local Local_32 TierC 0 433572712 1571494152  sessionGUID:SNetwork.SNetStructs+pSessionGUID FriendsData expID set to: Local_32,3,0 ActiveExpeditionUniqueKey: Local_32_TierC_0</color>
      00:00:09.000 - GAMESTATEMANAGER CHANGE STATE FROM : StopElevatorRide TO: ReadyToStartLevel
      00:00:10.000 - GAMESTATEMANAGER CHANGE STATE FROM : ReadyToStartLevel TO: InLevel
      00:00:10.000 - Player1 exits PLOC_InElevator 1</color>
      00:00:10:055 - Useless line
      00:00:10.100 - Player2 exits PLOC_InElevator 2</color>
      00:00:10.110 - Player3 exits PLOC_InElevator 3</color>
      00:00:10.250 - Player4 exits PLOC_InElevator 4</color>
      00:01:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:03:12.198 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:04:06.000 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:14:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:16:11.890 - BulkheadDoorController_Core.OnScanDone
      00:17:59.343 - GAMESTATEMANAGER CHANGE STATE FROM : InLevel TO: ExpeditionSuccess";

    fn init_file(dir: &TempDir, name: &str, data: &str) -> (File, PathBuf) {
        let file_path = dir.path().join(name);
        let file = File::create(&file_path);
        if file.is_err() {
            assert!(false)
        }
        let mut file = file.unwrap();

        let _ = write!(file, "{}", data);

        (file, file_path)
    }

    #[test]
    pub fn test_base() {
        let dir = tempdir().unwrap();

        let (file1, path1) = init_file(&dir, "file1.txt", TEXT);
        let (file2, path2) = init_file(&dir, "file2.txt", TEXT);

        let result = file_parse::parse_all_files(&vec![path1, path2]);
        let result = result.get_runs();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].is_win(), true);
        assert_eq!(
            result[0]
                .get_objective::<RunObjective>()
                .unwrap()
                .level_name,
            "R1C1"
        );
        assert_eq!(
            *result[0]
                .get_splits()
                .map(|v| v.get_time())
                .collect::<Vec<Time>>(),
            vec![
                Time::from("00:01:02.135").unwrap(),
                Time::from("00:02:00.063").unwrap(),
                Time::from("00:00:53.802").unwrap(),
                Time::from("00:10:06.135").unwrap(),
                Time::from("00:01:59.755").unwrap(),
                Time::from("00:01:47.453").unwrap(),
            ]
        );

        drop(file1);
        drop(file2);
        let _ = dir.close();
    }

    #[test]
    pub fn test_struct() {
        let dir = tempdir().unwrap();

        let (file1, path1) = init_file(&dir, "file1.txt", TEXT);
        let (file2, path2) = init_file(&dir, "file2.txt", TEXT);

        let result = file_parse::AwaitParseFiles::<ParserResult>::new(vec![path1, path2]);

        while !result.is_done() {
            sleep(Duration::from_millis(10));
        }

        let binding = Into::<ParserResult>::into(result);
        let result = binding.get_runs();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].is_win(), true);
        assert_eq!(
            result[0]
                .get_objective::<RunObjective>()
                .unwrap()
                .level_name,
            "R1C1"
        );
        assert_eq!(
            *result[0]
                .get_splits()
                .map(|v| v.get_time())
                .collect::<Vec<Time>>(),
            vec![
                Time::from("00:01:02.135").unwrap(),
                Time::from("00:02:00.063").unwrap(),
                Time::from("00:00:53.802").unwrap(),
                Time::from("00:10:06.135").unwrap(),
                Time::from("00:01:59.755").unwrap(),
                Time::from("00:01:47.453").unwrap(),
            ]
        );

        drop(file1);
        drop(file2);
        let _ = dir.close();
    }
}
