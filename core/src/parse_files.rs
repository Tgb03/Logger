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

    pub fn parse_file(path: &PathBuf, tokenizer: &impl Tokenizer) -> ParserResult {
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
    use std::{
        env, 
        path::PathBuf
    };
    use crate::{
        logs::{
            parser::ParserResult, 
            tokenizer::{
                GenerationTokenizer, 
                GenericTokenizer, 
                RunTokenizer, 
                Tokenizer
            }
        }, 
        run::traits::{
            Run, 
            Timed
        }
    };
    use super::*;

    fn create_tokenizer() -> GenericTokenizer {
        GenericTokenizer::default()
            .add_tokenizer(RunTokenizer)
            .add_tokenizer(GenerationTokenizer)
    }

    fn get_path(name: &str) -> Option<PathBuf> {
        let path_buf = env::current_dir()
            .ok()?
            .parent()?
            .join("examples")
            .join("log_files")
            .join(name)
            .with_extension("txt");

        Some(path_buf)
    }

    fn parse_file_t(name: &str, tokenizer: &impl Tokenizer) -> Option<ParserResult> {
        let path = get_path(name)?;
        
        Some(
            file_parse::parse_file(&path, tokenizer)
        )
    }

    #[test]
    fn test_parser_r1a1() {
        let tokenizer = create_tokenizer();
        let result = vec![
            "R1A1_client.frosty_exp_comp.txt",
            "R1A1_host.maid_exp_comp.txt",
        ];
        
        let iter = result
            .into_iter()
            .map(|v| parse_file_t(v, &tokenizer).unwrap());

        for result in iter {
            assert_eq!(result.get_counter(), 1);
            assert_eq!(result.get_runs()[0].len(), 3);
            assert_eq!(result.get_runs()[0].get_objective_str(), "R1A1_2.save");
            assert_eq!(result.get_runs()[0].get_name(), "R1A1_2.save");
            assert_eq!(result.get_level_name(), "R1A1");
            assert_eq!(result.get_set().len(), 1);
        }
    }
}
