use std::{fs::{self, File}, io::{BufRead, BufReader, Seek}, path::Path};

use egui::{Color32, Ui, Vec2};

use crate::{logs::{parser::Parser, token_parser::TokenParserT, tokenizer::Tokenizer}, objective_data::ObjectiveData, save_run::SaveManager, time::Time, timed_run::TimedRun};

pub struct LiveWindow {
  
  file: Option<File>,
  frame_counter: u8,
  last_position: u64,
  file_name: Option<String>,
  objective: ObjectiveData,
  player_input_string: String,

  runs_count: usize,
  run_length: usize,
  
  parser: Parser,

}

impl Default for LiveWindow {
  fn default() -> Self {
    Self { 
      file: Default::default(), 
      frame_counter: Default::default(), 
      last_position: Default::default(), 
      file_name: Default::default(), 
      objective: Default::default(), 
      player_input_string: Default::default(), 
      runs_count: Default::default(), 
      run_length: Default::default(), 
      parser: Default::default(),
    }
  }
}

impl LiveWindow {

  fn resize_gui(&mut self, ctx: &egui::Context, timed_run: &TimedRun) {
    let times = timed_run.get_times();
    let end_len = times.len() + match timed_run.get_time() != Time::default() {
      true => 1,
      false => 0,
    };

    if end_len != self.run_length {
      self.run_length = end_len;
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 { x: 200.0, y: 82.0 + 22.0 * self.run_length as f32 }));
    }
  }

  fn render_timed_run(&self, ui: &mut Ui, timed_run: &TimedRun, compared_run: Option<&TimedRun>) {
    let times = timed_run.get_times();
    let compared_times = match compared_run {
      Some(run) => Some(run.get_times()),
      None => None,
    };

    // if times.len() == 0 {
    //   return;
    // }

    ui.vertical(|ui| {
      for (id, time) in times.iter().enumerate() {
        ui.horizontal(|ui| {
          ui.label(time.to_string());

          if compared_times.is_some_and(|r| r.len() > id) {
            let compared_times = compared_times.unwrap();

            let (time_diff, color) = match time.is_greater_than(&compared_times[id]) {
              true => (time.sub(&compared_times[id]), Color32::RED),
              false => (compared_times[id].sub(&time), Color32::GREEN),
            };
            
            ui.colored_label(color, time_diff.to_string_no_hours());
          }
        });
      }

      ui.horizontal(|ui| {
        let final_time = timed_run.get_time();
        if final_time != Time::default() && !timed_run.is_win() {
          ui.label(final_time.to_string());

          if compared_run.is_some() {
            let compared_run = compared_run.unwrap();
            let compared_time = match timed_run.is_win() {
              true => compared_run.get_time(),
              false => compared_run.get_times()[times.len()],
            };

            let (time_diff, color) = match final_time.is_greater_than(&compared_time) {
              true => (final_time.sub(&compared_time), Color32::RED),
              false => (compared_time.sub(&final_time), Color32::GREEN),
            };
            
            ui.colored_label(color, time_diff.to_string_no_hours());
          }
        }
      });
    }); 
  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager, ctx: &egui::Context) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      
      self.parser.parse_continously(tokens.into_iter());
      
      let vecs = self.parser.into_result();
      while self.runs_count < vecs.get_runs().len() {
        save_manager.save(vecs.get_runs().get(self.runs_count).unwrap().clone());
        self.runs_count += 1;
      }
    }

    // if let Some(file_name) = &self.file_name {
    //   ui.label(format!("File:{}", file_name));
    // }

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

    if let Some(parser) = self.parser.get_run_parser() {
      let timed_run = parser.into_result();

      self.objective.level_name = timed_run.objective_data.level_name.clone();
      self.objective.player_count = timed_run.objective_data.player_count;

      ui.label(format!("In run: {}", timed_run.objective_data.get_id()));
    
      // ui.label(self.objective.get_id());
      self.render_timed_run(ui, timed_run, save_manager.get_best_run(&self.objective));
      // TODO: DELETE .clone()
      self.resize_gui(ctx, &timed_run.clone()); // try to find a way to remove this clone

      return;
    }

    let result = self.parser.into_result();
    if let Some(timed_run) = result.get_runs().last() {
      
      self.objective.level_name = timed_run.objective_data.level_name.clone();
      self.render_timed_run(ui, timed_run, save_manager.get_best_run(&timed_run.objective_data));
      // TODO: DELETE .clone()
      self.resize_gui(ctx, &timed_run.clone()); // try to find a way to remove this clone

      ui.label("Not currently in run");

      return;
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
      };

      self.parser = Parser::default();
      self.last_position = 0;
      self.run_length = 0;
    }

  }

  fn load_text(&mut self) -> String {
    if let Some(file) = &mut self.file {
      let _ = file.seek(std::io::SeekFrom::Start(self.last_position));
      let mut reader = BufReader::new(file);
      let mut buffer = String::new();
      let mut new_lines = Vec::new();

      while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
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