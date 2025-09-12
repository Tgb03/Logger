
use core::run::default_dirs::{self, get_config_directory};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use egui::{Color32, Label, RichText};
use serde::{Deserialize, Serialize};

use crate::render::Render;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum FieldValue {
    Boolean(bool),
    Integer(i32, String), // second value: string is input field, not actually given out
    Float(f32, String), // second value: string is input field, not actually given out
    String(String), // string here acts as both input field and value
    Path(PathBuf, String), // it is required to have a string as input as pathbuf cant be edited directly
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Field {

    description: String,
    value: FieldValue,

}

impl Field {
    pub fn new(description: String, value: FieldValue) -> Self {
        Self {
            description,
            value,
        }
    }
}

impl Into<Option<bool>> for &Field {
    fn into(self) -> Option<bool> {
        if let FieldValue::Boolean(b) = self.value {
            return Some(b);
        }

        None
    }
}

impl Into<Option<i32>> for &Field {
    fn into(self) -> Option<i32> {
        if let FieldValue::Integer(i, _) = self.value {
            return Some(i)
        }
        
        None
    }
}

impl Into<Option<f32>> for &Field {
    fn into(self) -> Option<f32> {
        if let FieldValue::Float(f, _) = self.value {
            return Some(f)
        }
        
        None
    }
}

impl<'a> Into<Option<&'a String>> for &'a Field {
    fn into(self) -> Option<&'a String> {
        if let FieldValue::String(s) = &self.value {
            return Some(s)
        }

        None
    }
}

impl<'a> Into<Option<&'a PathBuf>> for &'a Field {
    fn into(self) -> Option<&'a PathBuf> {
        if let FieldValue::Path(p, _) = &self.value {
            return Some(p)
        }

        None
    }
}

impl Render for Field {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        match &mut self.value {
            FieldValue::Boolean(b) => { 
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.checkbox(b, &self.description);
                }); 
            },
            FieldValue::Integer(i, s) => {
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                        ui.monospace(&self.description);
                        if ui
                            .add(
                                egui::TextEdit::singleline(s)
                                    .desired_width(50.0)
                                    .background_color(Color32::from_rgb(32, 32, 32))
                                    .text_color(Color32::WHITE),
                            )
                            .changed()
                        {
                            if let Ok(x) = s.parse::<i32>() {
                                *i = x;
                            }
                        }
                });
            },
            FieldValue::Float(f, s) => {
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.monospace(&self.description);
                    if ui
                        .add(
                            egui::TextEdit::singleline(s)
                                .desired_width(50.0)
                                .background_color(Color32::from_rgb(32, 32, 32))
                                .text_color(Color32::WHITE),
                        )
                        .changed()
                    {
                        if let Ok(x) = s.parse::<f32>() {
                            *f = x;
                        }
                    }
                });
            },
            FieldValue::String(s) => {
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.monospace(&self.description);
                    ui.add(
                        egui::TextEdit::singleline(s)
                            .desired_width(512.0)
                            .background_color(Color32::from_rgb(32, 32, 32))
                            .text_color(Color32::WHITE),
                    );
                });
            },
            FieldValue::Path(path_buf, s) => {
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.monospace(&self.description);
                    if ui.add(
                        egui::TextEdit::singleline(s)
                            .desired_width(512.0)
                            .background_color(Color32::from_rgb(32, 32, 32))
                            .text_color(Color32::WHITE),
                    ).changed() {
                        *path_buf = (&s).into();
                    }
                });
            },
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct SettingsWindow {

    setting_hash: HashMap<String, Field>,
    
    general: Vec<String>,
    splitter: Vec<String>,
    mapper: Vec<String>,
    foresight: Vec<String>,

}

impl Default for SettingsWindow {
    fn default() -> Self {
        if let Some(s) = Self::load() {
            return s;
        }

        let s = Self {
            setting_hash: HashMap::new(),
            general: Vec::new(),
            splitter: Vec::new(),
            mapper: Vec::new(),
            foresight: Vec::new(),
        };

        s.add_all()
    }
}

impl SettingsWindow {

    fn add_all(mut self) -> Self {
        let log_path = Self::logs_path()
            .unwrap_or_default();

        self.add_to_general("automatic_loading".into(), Field::new("Automatic Loading of Runs".into(), FieldValue::Boolean(true)));
        self.add_to_general("automatic_saving".into(), Field::new("Automatic Saving of Runs".into(), FieldValue::Boolean(false)));
        self.add_to_general("show_real_timer".into(), Field::new("Show real timer   Warning: this timer may not be accurate. Use the in game timer for that.".into(), FieldValue::Boolean(false)));
        self.add_to_general("show_game_splitter".into(), Field::new("Show game splitter".into(), FieldValue::Boolean(false)));
        self.add_to_general("show_run_splitter".into(), Field::new("Show run splitter".into(), FieldValue::Boolean(true)));
        self.add_to_general("show_run_counter".into(), Field::new("Show run counter".into(), FieldValue::Boolean(true)));

        self.add_to_splitter("window_transparency".into(), Field::new("LiveWindow Transparency".into(), FieldValue::Float(0.6f32, "0.6f".to_owned())));
        self.add_to_splitter("logs_path".into(), Field::new("Path to logs folder: ".into(), FieldValue::Path(
            log_path.clone(),
            log_path.into_os_string().into_string().unwrap_or_default()
                // unfortunately there is no nicer way of doing this
                // as it is required
        )));
        self.add_to_splitter("compare_record".into(), Field::new("Compare to saved record".into(), FieldValue::Boolean(true)));
        self.add_to_splitter("compare_best_splits".into(), Field::new("Compare to best splits".into(), FieldValue::Boolean(true)));
        self.add_to_splitter("run_splitter_length".into(), Field::new("Run Splitter max length".into(), FieldValue::Integer(100, "100".into())));
        self.add_to_splitter("game_splitter_length".into(), Field::new("Game Splitter max length".into(), FieldValue::Integer(5, "5".into())));
        self.add_to_splitter("x_position".into(), Field::new("X position".into(), FieldValue::Float(0f32, "0".into())));
        self.add_to_splitter("y_position".into(), Field::new("Y position".into(), FieldValue::Float(250f32, "250".into())));
        self.add_to_splitter("x_size".into(), Field::new("X size    ".into(), FieldValue::Float(250f32, "250".into())));

        self.add_to_mapper("show_mapper".into(), Field::new("Show mapper in live splitter".into(), FieldValue::Boolean(true)));
        self.add_to_mapper("show_objectives".into(), Field::new("Show objectives items in live splitter".into(), FieldValue::Boolean(true)));
        self.add_to_mapper("show_code_guess".into(), Field::new("Show code guess".into(), FieldValue::Boolean(false)));
        self.add_to_mapper("code_guess_line_count".into(), Field::new("Code guess number of lines: ".into(), FieldValue::Integer(3, "3".into())));
        self.add_to_mapper("code_guess_word_count".into(), Field::new("Code guess number of words per line: ".into(), FieldValue::Integer(7, "7".into())));

        self.add_to_foresight("show_foresight".into(), Field::new("Show Foresight in live splitter".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_resources".into(), Field::new("Show resources in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_consumables".into(), Field::new("Show consumables in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_artifacts".into(), Field::new("Show artifacts in foresight".into(), FieldValue::Boolean(false)));
        self.add_to_foresight("seed_indexer_show_gather_small_items".into(), Field::new("Show GatherSmallItem in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_fog_turbine".into(), Field::new("Show FogTurbine in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_cell".into(), Field::new("Show Cell in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_colored_key".into(), Field::new("Show ColoredKey in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_bulkhead_key".into(), Field::new("Show BulkKey in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_terminal_uplink".into(), Field::new("Show TerminalUplink in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_retrieve_big_items".into(), Field::new("Show RetrieveBigItems in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_special_terminal_command".into(), Field::new("Show SpecialTerminalCommand in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_hsu".into(), Field::new("Show HSU_FindTakeSample in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_show_power_cell_distribution".into(), Field::new("Show PowerCellDistribution in foresight".into(), FieldValue::Boolean(true)));
        self.add_to_foresight("seed_indexer_length".into(), Field::new("Size of Foresight".into(), FieldValue::Integer(10, "10".into())));

        self
    }

    fn add_to_general(&mut self, id: String, field: Field) {
        if self.setting_hash.contains_key(&id) { return }

        self.general.push(id.clone());
        self.setting_hash.insert(id, field);
    }

    fn add_to_splitter(&mut self, id: String, field: Field) {
        if self.setting_hash.contains_key(&id) { return }

        self.splitter.push(id.clone());
        self.setting_hash.insert(id, field);
    }

    fn add_to_mapper(&mut self, id: String, field: Field) {
        if self.setting_hash.contains_key(&id) { return }
        
        self.mapper.push(id.clone());
        self.setting_hash.insert(id, field);
    }

    fn add_to_foresight(&mut self, id: String, field: Field) {
        if self.setting_hash.contains_key(&id) { return }

        self.foresight.push(id.clone());
        self.setting_hash.insert(id, field);
    }

    fn logs_path() -> Option<PathBuf> {
        if let Some(dirs) = directories::UserDirs::new() {
            return Some(
                dirs.home_dir()
                    .to_path_buf()
                    .join("AppData\\LocalLow\\10 Chambers Collective\\GTFO"),
            );
        }

        None
    }

    pub fn load() -> Option<Self> {
        let path = default_dirs::get_config_directory()?
            .join("app.properties");
        let mut file = File::open(path).ok()?;
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).ok()?;

        let p = serde_yaml::from_str(&file_str).ok()
            .map(|v: SettingsWindow| v.add_all());

        p
    }

    pub fn save_settings(&self) -> Option<()> {
        let path = default_dirs::get_config_directory()?.join("app.properties");
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }

        let text = serde_yaml::to_string(self).ok()?;
        let _ = std::fs::write(path, text).ok()?;

        Some(())
    }

    #[allow(private_bounds)]
    pub fn get<V>(&self, field_name: &str) -> Option<V>
    where 
        for<'a> &'a Field: Into<Option<V>> {
        
        self.setting_hash.get(field_name)
            .map(|v| v.into())
            .flatten()
    }

    #[allow(private_bounds)]
    pub fn get_def<V>(&self, field_name: &str) -> V 
    where
        for<'a> &'a Field: Into<Option<V>>,
        V: Default {
            self.get(field_name)
                .unwrap_or_else(|| {
                    eprintln!("Failed to find: {field_name}");
                    V::default()
                })
    }

    #[allow(private_bounds)]
    pub fn get_path(&self, field_name: &str) -> Option<&PathBuf> {
        self.setting_hash.get(field_name)
            .map(|v| v.into())
            .flatten()
    }

}

impl Render for SettingsWindow {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 60.0)
            .show(ui, |ui| {
            
            ui.add(Label::new(
                Into::<RichText>::into("General settings: ").size(14.0),
            ));

            ui.add_space(10.0);

            for id in &self.general {
                self.setting_hash
                    .get_mut(id)
                    .map(|v| v.render(ui));
            }

            ui.separator();

            ui.add(Label::new(
                Into::<RichText>::into("LiveSplitter settings: ").size(14.0),
            ));

            ui.add_space(10.0);

            for id in &self.splitter {
                self.setting_hash
                    .get_mut(id)
                    .map(|v| v.render(ui));
            }

            ui.separator();

            ui.add(Label::new(
                Into::<RichText>::into("Mapper settings: ").size(14.0),
            ));

            ui.add_space(10.0);

            for id in &self.mapper {
                self.setting_hash
                    .get_mut(id)
                    .map(|v| v.render(ui));
            }

            ui.horizontal(|ui| {
                    if ui.button("Open LevelView folder").clicked() {
                        if let Some(mut path) = get_config_directory() {
                            path = path.join("levels");

                            if !path.exists() {
                                let _ = std::fs::create_dir_all(&path);
                            }

                            let _ = opener::open(path);
                        }
                    }

                    if ui.button("Open examples for LevelView").clicked() {
                        let _ = opener::open_browser(
                            "https://github.com/Tgb03/Logger/tree/master/examples/level_view",
                        );
                    }
            });
            
            ui.separator();

            ui.add(Label::new(
                Into::<RichText>::into("Foresight settings: ").size(14.0),
            ));

            ui.add_space(10.0);

            for id in &self.foresight {
                self.setting_hash
                    .get_mut(id)
                    .map(|v| v.render(ui));
            }
        });

        ui.separator();
        ui.add_space(5.0);

        ui.label(format!("App version: {}", env!("CARGO_PKG_VERSION")));
        ui.label(format!("Made by Tgb03"));
    }
}

