use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::{
    run::{
        merge_splits::{
            LevelsMergeSplits, 
            MergeSplits
        }, 
        run_enum::RunEnum, 
        traits::{
            Run, 
            Timed
        }
    },
    sort::Sortable,
    time::Time,
};

/// Save manager struct
///
/// This handles loading runs into memory and then ultimately saving them
pub struct SaveManager {
    loaded_runs: HashMap<String, Vec<RunEnum>>,

    best_splits: HashMap<String, HashMap<String, Time>>,
    split_names: HashMap<String, Vec<String>>,

    split_merges: LevelsMergeSplits,
    reversed_merges: HashMap<String, HashMap<String, Vec<String>>>,

    automatic_saving: bool,
}

impl Default for SaveManager {
    fn default() -> Self {
        let split_merges: LevelsMergeSplits = Self::get_directory()
            .map(|v| v.join("merge_data.bin"))
            .map(|path| {
                    std::fs::read(&path).map(|data| {
                        bincode::deserialize(&data)
                            .ok()
                })
                .ok()
                .flatten()
            })
            .flatten()
            .unwrap_or_default();

        Self { 
            loaded_runs: Default::default(), 
            best_splits: Default::default(), 
            split_names: Default::default(), 
            reversed_merges: split_merges.reversed(),
            automatic_saving: false,
            split_merges, 
        }
    }
}

impl SaveManager {
    fn remove_duplicates(&mut self, objective: &String) {
        if let Some(vec) = self.loaded_runs.remove(objective) {
            let set: HashSet<RunEnum> = HashSet::from_iter(vec);

            self.loaded_runs
                .insert(objective.clone(), set.into_iter().collect());
        }
    }

    fn save_no_remove_duplicates(&mut self, timed_run: RunEnum) -> Option<String> {
        if timed_run.len() == 1 && !timed_run.is_win() {
            return None;
        }

        let objective = timed_run.get_objective_str().clone();

        match self.loaded_runs.get_mut(&objective) {
            Some(vec) => {
                vec.push(timed_run);
            }
            None => {
                self.loaded_runs
                    .insert(objective.to_string(), vec![timed_run]);
            }
        };

        self.calculate_best_splits(&objective);

        return Some(objective);
    }

    pub fn set_automatic_saving(&mut self, automatic_saving: bool) {
        self.automatic_saving = automatic_saving;
    }

    pub fn get_split_merge(&self, objective: &String, split_name: &String) -> Option<&String> {
        self.split_merges.get_level(objective)
            .map(|v| v.get_split(split_name))
            .flatten()
    }

    pub fn get_split_names(&self, objective: &String) -> Option<&Vec<String>> {
        self.split_names.get(objective)
    }

    pub fn get_directory() -> Option<PathBuf> {
        #[cfg(debug_assertions)]
        if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger Debug") {
            return Some(proj_dirs.data_dir().to_path_buf());
        }

        #[cfg(not(debug_assertions))]
        if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger") {
            return Some(proj_dirs.data_dir().to_path_buf());
        }

        None
    }

    pub fn get_config_directory() -> Option<PathBuf> {
        #[cfg(debug_assertions)]
        if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger Debug") {
            return Some(proj_dirs.config_dir().to_path_buf());
        }

        #[cfg(not(debug_assertions))]
        if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger") {
            return Some(proj_dirs.config_dir().to_path_buf());
        }

        None
    }

    /// save a single timed run into RAM
    ///
    /// duplicates are automatically removed.
    pub fn save(&mut self, timed_run: RunEnum) {
        if let Some(name) = self.save_no_remove_duplicates(timed_run) {
            self.remove_duplicates(&name);
        }
    }

    pub fn calculate_best_splits(&mut self, objective_id: &String) {
        let empty = Vec::new();
        let runs = self.loaded_runs.get(objective_id).unwrap_or(&empty);
        let merge_splits = self.split_merges.get_level(objective_id);

        let mut build_vec: Vec<String> = Vec::new();
        let mut build_hash: HashMap<String, (Time, u32)> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();

        for run in runs {
            let mut times = HashMap::new();

            for split in run.get_splits() {
                let name = split.get_name();

                let name = merge_splits
                    .map(|ms| ms.get_split(name))
                    .flatten()
                    .unwrap_or(name);

                if name != "LOSS" && !set.contains(name) {
                    //println!("SET: {:?}", set);
                    build_vec.push(name.clone());
                    set.insert(name.clone());
                }

                match times.get_mut(name) {
                    Some((time, count)) => {
                        *time += split.get_time();
                        *count += 1;
                    },
                    None => { times.insert(name.clone(), (split.get_time(), 1)); },
                };
            }

            for (name, (time, count)) in times {
                match build_hash.get(&name) {
                    Some((b_time, b_count)) => {
                        if count < *b_count { continue; }

                        if time < *b_time || count > *b_count { 
                            build_hash.insert(name, (time, count)); 
                        }
                    },
                    None => { build_hash.insert(name, (time, count)); },
                }
            }
        }

        let mut bh = HashMap::new();
        for (key, (value, _)) in build_hash {
            bh.insert(key, value);
        }

        // println!("Inserted: {:?}", build_vec);
        // println!("Hashed: {:?}", build_hash);
        self.split_names.insert(objective_id.clone(), build_vec);
        self.best_splits.insert(objective_id.clone(), bh);
    }

    /// save multiple runs into the RAM memory.
    ///
    /// duplicates are automatically removed.
    pub fn save_multiple(&mut self, timed_runs: Vec<RunEnum>) {
        let mut set = HashSet::new();

        for run in timed_runs {
            if let Some(name) = self.save_no_remove_duplicates(run) {
                set.insert(name);
            }
        }

        for name in &set {
            self.remove_duplicates(name);
        }

        for objective in set {
            self.calculate_best_splits(&objective);
        }
    }

    // returns the world record run for a level
    pub fn get_best_run(&self, objective_data: &String) -> Option<&RunEnum> {
        match self.loaded_runs.get(objective_data) {
            Some(runs) => {
                let mut best_run = None;
                let mut best_time = Time::max();

                for timed_run in runs {
                    if timed_run.get_time() < best_time && timed_run.is_win() {
                        best_run = Some(timed_run);
                        best_time = timed_run.get_time();
                    }
                }

                best_run
            }
            None => None,
        }
    }

    pub fn get_runs(&self, objective_data: &String) -> Option<&Vec<RunEnum>> {
        self.loaded_runs.get(objective_data)
    }

    /// returns all runs for the objective.
    pub fn get_runs_mut(&mut self, objective_data: &String) -> Option<&mut Vec<RunEnum>> {
        self.loaded_runs.get_mut(objective_data)
    }

    pub fn get_best_split(&self, objective: &String, name: &String) -> Option<&Time> {
        self.best_splits
            .get(objective)
            .map(|h| h.get(name))
            .flatten()
    }

    /// returns all best splits for the objective.
    pub fn get_best_splits(&self, objective_data: &String) -> Option<&HashMap<String, Time>> {
        self.best_splits.get(objective_data)
    }

    /// load all runs that were saved to folder
    pub fn load_all_runs(&mut self) {
        let file_path = Self::get_directory();
        let paths = match file_path {
            Some(file_path) => {
                if !file_path.exists() {
                    let _ = fs::create_dir_all(&file_path);
                }

                Some(fs::read_dir(file_path).unwrap())
            }
            None => None,
        };

        if let Some(paths) = paths {
            for path in paths {
                if let Ok(entry) = path {
                    if entry.file_name().into_string().unwrap().contains(".save") {
                        self.load(&entry.file_name().into_string().unwrap());
                    }

                    if entry.file_name().into_string().unwrap().contains(".rsave") {
                        self.load(&entry.file_name().into_string().unwrap());
                    }
                }
            }
        }
    }

    /// optimize these runs by removing all that do not hold
    /// important information
    ///
    /// if the run is not world record or has a best split it is removed
    pub fn optimize_obj(&mut self, objective_data: &String) {
        let best_time = match self.get_best_run(objective_data) {
            Some(run) => Some(run.get_time()),
            None => None,
        };
        let mut for_deletions = Vec::new();

        if let Some(runs) = self.loaded_runs.get(objective_data) {
            for (r_id, run) in runs.into_iter().enumerate() {
                let mut is_valid = false;
                if best_time.is_some_and(|t| run.get_time() == t) {
                    continue;
                }

                for split in run.get_splits() {
                    if self
                        .get_best_split(objective_data, split.get_name())
                        .is_some_and(|t| split.get_time() == *t)
                    {
                        is_valid = true;
                        break;
                    }
                }

                if !is_valid {
                    for_deletions.push(r_id);
                }
            }
        }

        let runs = self.get_runs_mut(objective_data).unwrap();
        for elem in for_deletions.iter().rev() {
            runs.remove(*elem);
        }
    }

    /// load from file the objective data.
    pub fn load(&mut self, objective_data: &String) {
        let file_path = Self::get_directory().map(|path| path.join(objective_data.clone()));

        if let Some(file_path) = file_path {
            match std::fs::read(file_path) {
                Ok(binary_data) => {
                    let mut vec: Vec<RunEnum> = match bincode::deserialize(&binary_data) {
                        Ok(vec) => vec,
                        Err(_) => Vec::new(),
                    };

                    for it in &mut vec {
                        it.set_objective_str(objective_data.clone());
                    }

                    //println!("Added vec with size for obj: {}, {}, {}", vec.len(), binary_data.len(), objective_data);
                    self.save_multiple(vec);
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }
    }

    pub fn save_to_file(&self, objective_data: &String) {
        let file_path = Self::get_directory();

        if let Some(file_path) = file_path.as_ref() {
            if !file_path.exists() {
                let _ = std::fs::create_dir(&file_path);
            }
        }

        let file_path =
            file_path.map(|path| path.join(Into::<String>::into(objective_data.to_string())));

        let empty = Vec::new();
        if let Ok(bin_data) =
            bincode::serialize(self.loaded_runs.get(objective_data).unwrap_or(&empty))
        {
            if let Some(file_path) = file_path {
                let _ = std::fs::write(file_path, &bin_data);
            }
        }
    }

    /// save all loaded runs to files
    pub fn save_to_files(&self) {
        for (key, vec) in &self.loaded_runs {
            let file_path =
                Self::get_directory().map(|path| path.join(Into::<String>::into(key.to_string())));

            if let Ok(bin_data) = bincode::serialize(&vec) {
                //println!("Saved vec with size: {}: {}", vec.len(), bin_data.len());
                if let Some(file_path) = file_path {
                    let _ = std::fs::write(file_path, &bin_data);
                }
            }
        }
    }

    pub fn get_all_objectives(&self) -> Vec<String> {
        let mut v = self.loaded_runs.keys().cloned().collect::<Vec<String>>();
        v.sort_by_key(|a| Into::<String>::into(a.to_string()));
        v
    }

    pub fn set_merge_splits(&mut self, objective: &String, data: &str) {
        let merged: MergeSplits = data.into();

        self.reversed_merges.insert(objective.clone(), merged.reverse());
        self.split_merges.add_level(objective, merged);

        self.calculate_best_splits(objective);
    }

    pub fn get_splits_req(&self, objective: &String, split_name: &String) -> Option<&Vec<String>> {
        self.reversed_merges
            .get(objective)?
            .get(split_name)
    }

    pub fn get_level_merge_split_str(&self, objective: &String) -> Option<String> {
        self.split_merges.get_level(objective)
            .map(|v| v.into())
    }
}

impl Sortable<RunEnum> for SaveManager {
    fn get_vec(&mut self, objective: &String) -> Option<&mut Vec<RunEnum>> {
        self.loaded_runs.get_mut(objective)
    }
}

impl Drop for SaveManager {
    // save the merge splits automatically
    fn drop(&mut self) {
        if let Some(file_path) = Self::get_directory().map(|v| v.join("merge_data.bin")) {
            if let Ok(bin) = bincode::serialize(&self.split_merges) {
                let _ = std::fs::write(file_path, bin);
            }
        }
        if self.automatic_saving {
            self.save_to_files();
        }
    }
}
