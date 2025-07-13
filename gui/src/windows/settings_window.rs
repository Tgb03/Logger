use core::save_manager::SaveManager;
use std::{collections::HashMap, fs::File, io::Read};



enum Field {
    Boolean(bool),
    Integer(u32),
    Float(f32),
    String(String),
}


pub struct SettingsWindow {

    settings: HashMap<String, Field>,

}

impl Default for SettingsWindow {
    fn default() -> Self {
        let settings = HashMap::new();

        let path = SaveManager::get_config_directory();
        let file_str: Option<String> =
            path.map(|path| match File::open(path.join("app.properties")) {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    let _ = file.read_to_string(&mut buffer);
                    buffer
                }
                Err(_) => String::new(),
            });

        let props: HashMap<String, String> = file_str.map_or(HashMap::default(), |file_str| {
            match serde_yaml::from_str(&file_str) {
                Ok(map) => map,
                Err(_) => HashMap::new(),
            }
        });

        Self { 
            settings
        }
    }
}

impl SettingsWindow {

    

}

