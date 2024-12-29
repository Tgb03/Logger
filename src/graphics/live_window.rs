use std::{fs::{self, File}, io::{BufRead, BufReader, Seek}, path::Path};

use egui::Ui;

use crate::{logs::tokenizer::Tokenizer, objective_data::ObjectiveData, save_run::SaveManager};

pub struct LiveWindow {
  
  file: Option<File>,
  frame_counter: u8,
  last_position: u64,
  file_name: Option<String>,
  objective: ObjectiveData,
  player_input_string: String,

}

impl Default for LiveWindow {
  fn default() -> Self {

    Self { 
      file: None,
      frame_counter: 0,
      last_position: 0,
      file_name: None,
      objective: ObjectiveData::default(),
      player_input_string: String::default(),
    }
  }
}

impl LiveWindow {

  pub fn show(&mut self, ui: &mut Ui, _save_manager: &mut SaveManager) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      //let result = self.run_parser.parse_continously(tokens);
    }

    if let Some(file_name) = &self.file_name {
      //ui.label(format!("File:{}", file_name));
    }

    ui.horizontal(|ui| {
      ui.label("Level name: ");
      ui.add(egui::TextEdit::singleline(&mut self.objective.level_name)
        .desired_width(30.0));
    });
        
    ui.horizontal(|ui| {
      ui.label("PlayerCount: ");
      ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
          .desired_width(10.0));
    });

    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective.secondary, "Sec");
      ui.checkbox(&mut self.objective.overload, "Ovrld");
    });
      
    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective.glitched, "Glitch");
      ui.checkbox(&mut self.objective.early_drop, "E-Drop");
    });

    self.objective.level_name = self.objective.level_name.to_uppercase();
    if let Some(count) = self.player_input_string.parse::<u8>().ok() {
      self.objective.player_count = count;
    }

  }

  pub fn load_file(&mut self) {
    let path = Path::new(env!("HOME")).join("Appdata\\Locallow\\10 Chambers Collective\\GTFO\\");

    let path = fs::read_dir(path)
      .expect("Couldn't access local directory")
      .flatten()
      .filter(|f| {
        let metadata = f.metadata().unwrap();

        metadata.is_file() && f.file_name().to_str().unwrap_or_default().contains("NICKNAME_NETSTATUS")
      })
      .max_by_key(|x| x.metadata().unwrap().modified().unwrap());

    if let Some(path) = path {
      let path = path.path();
      let name = path.file_name().unwrap_or_default();
      let str_name = name.to_str().unwrap_or_default();
    
      self.file_name = Some(str_name.to_string());
      self.file = match File::open(path) {
        Ok(file) => Some(file),
        Err(_) => None,
      }  
    }

  }

  fn load_text(&mut self) -> String {
    if let Some(file) = &mut self.file {
      let _ = file.seek(std::io::SeekFrom::Start(self.last_position));
      let mut reader = BufReader::new(file);
      let mut buffer = String::new();
      let mut new_lines = Vec::new();

      while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        //println!("EH: {}", buffer);
        new_lines.push(buffer.to_string());
        buffer.clear();
      }

      self.last_position = reader.seek(std::io::SeekFrom::Current(0)).expect("Seek 0 failed in live window.");
    
      return new_lines.iter()
        .fold(String::new(), |s1, s2| s1 + s2)
        .to_string();
    }

    "".to_owned()
  }

}