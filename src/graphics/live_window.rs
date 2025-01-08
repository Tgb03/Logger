use std::{fs::{self, File}, io::{BufRead, BufReader, Seek}, path::Path};

use egui::{Color32, Ui, Vec2};

use crate::{logs::{parser::Parser, token_parser::TokenParserT, tokenizer::Tokenizer}, objective_data::ObjectiveData, save_run::SaveManager, time::Time, timed_run::TimedRun};

use super::settings_window::SettingsWindow;

#[derive(Default)]
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


impl LiveWindow {

  fn resize_gui(&mut self, width: f32, ctx: &egui::Context, timed_run: &TimedRun) {
    let times = timed_run.get_times();
    let end_len = times.len() + match timed_run.get_time() != Time::default() {
      true => 1,
      false => 0,
    };

    if end_len != self.run_length {
      self.run_length = end_len;
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 { x: width, y: 102.0 + 22.0 * self.run_length as f32 }));
    }
  }

  fn render_timed_run(&self, ui: &mut Ui, timed_run: &TimedRun, compared_run: Option<&TimedRun>, compared_splits: Option<&Vec<Time>>) {
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
          ui.label(super::create_text(time.to_string()));

          if compared_times.is_some_and(|r| r.len() > id) {
            let compared_times = compared_times.unwrap();

            let (time_diff, color) = match time.is_greater_than(&compared_times[id]) {
              true => (time.sub(&compared_times[id]), Color32::RED),
              false => (compared_times[id].sub(&time), Color32::GREEN),
            };
            
            ui.colored_label(color, super::create_text(time_diff.to_string_no_hours()));
          }

          if compared_splits.is_some() {
            let compared_splits = compared_splits.unwrap();
            let split = match id > 0 {
              true => times[id].sub(&times[id - 1]),
              false => *time,
            };

            let (time_diff, color) = match split.is_greater_than(&compared_splits[id]) {
              true => (split.sub(&compared_splits[id]), Color32::RED),
              false => (compared_splits[id].sub(&split), Color32::GREEN),
            };

            ui.colored_label(color, super::create_text(time_diff.to_string_no_hours()));
          }
        });
      }

      ui.horizontal(|ui| {
        let final_time = timed_run.get_time();

        if final_time != Time::default() && !timed_run.is_win() && self.parser.get_run_parser().is_some_and(|p| !p.is_done()) {
          ui.label(super::create_text(final_time.to_string()));

          if let Some(compared_run) = compared_run {
            let compared_time = match timed_run.is_win() {
              true => compared_run.get_time(),
              false => compared_run.get_times()[times.len()],
            };

            let (time_diff, color) = match final_time.is_greater_than(&compared_time) {
              true => (final_time.sub(&compared_time), Color32::RED),
              false => (compared_time.sub(&final_time), Color32::GREEN),
            };
            
            ui.colored_label(color, super::create_text(time_diff.to_string_no_hours()));
          }

          if let Some(compared_splits) = compared_splits {
            let id = times.len();
            let split = match id {
              0 => final_time,
              _ => final_time.sub(&times[id - 1]),
            };

            let (time_diff, color) = match split.is_greater_than(&compared_splits[id]) {
              true => (split.sub(&compared_splits[id]), Color32::RED),
              false => (compared_splits[id].sub(&split), Color32::GREEN),
            };

            ui.colored_label(color, super::create_text(time_diff.to_string_no_hours()));
          }
        }
      });
    }); 
  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager, settings: &SettingsWindow, ctx: &egui::Context) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      
      self.parser.parse_continously(tokens.into_iter());
      
      let vecs = self.parser.into_result();
      while self.runs_count < vecs.get_runs().len() {
        let mut run = vecs.get_runs().get(self.runs_count).unwrap().clone();
        run.objective_data.early_drop = self.objective.early_drop;
        run.objective_data.glitched = self.objective.glitched;
        run.objective_data.secondary = self.objective.secondary.max(run.objective_data.secondary);
        run.objective_data.overload = self.objective.overload.max(run.objective_data.overload);
        if let Ok(id) = self.player_input_string.parse::<u8>() {
          run.objective_data.player_count = id;
        } 
        save_manager.save(run);
        self.runs_count += 1;
      }
    }

    // if let Some(file_name) = &self.file_name {
    //   ui.label(format!("File:{}", file_name));
    // }

    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective.secondary, super::create_text("Sec"));
      ui.checkbox(&mut self.objective.overload, super::create_text("Ovrld"));
      ui.label(super::create_text("Ps:"));
      ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
        .char_limit(2)
        .background_color(Color32::from_rgba_unmultiplied(32, 32, 32, 32)));
    });
    
    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective.glitched, super::create_text("Glitch"));
      ui.checkbox(&mut self.objective.early_drop, super::create_text("E-Drop"));
    });

    self.objective.level_name = self.objective.level_name.to_uppercase();
    if let Some(count) = self.player_input_string.parse::<u8>().ok() {
      self.objective.player_count = count;
    }

    ui.separator();

    if let Some(parser) = self.parser.get_run_parser() {
      let timed_run = parser.into_result();

      self.objective.level_name = timed_run.objective_data.level_name.clone();
      self.objective.player_count = timed_run.objective_data.player_count;

      ui.label(super::create_text(format!("In run: {}", self.objective.get_id())));
    
      ui.label(self.objective.get_id());
      let compared_run = match settings.get_compare_to_record() { 
        true => save_manager.get_best_run(&self.objective), 
        false => None,
      };
      let best_splits = match settings.get_compare_to_theoretical() {
        true => save_manager.get_best_splits(&self.objective),
        false => None,
      };
      self.render_timed_run(ui, timed_run, compared_run, best_splits);
      // TODO: DELETE .clone()
      self.resize_gui(settings.get_live_rectangle().width(), ctx, &timed_run.clone()); // try to find a way to remove this clone

      return;
    }

    let result = self.parser.into_result();
    if let Some(timed_run) = result.get_runs().last() {
      
      self.objective.level_name = timed_run.objective_data.level_name.clone();
      let compared_run = match settings.get_compare_to_record() { 
        true => save_manager.get_best_run(&self.objective), 
        false => None,
      };
      let best_splits = match settings.get_compare_to_theoretical() {
        true => save_manager.get_best_splits(&self.objective),
        false => None,
      };
      self.render_timed_run(ui, timed_run, compared_run, best_splits);
      // TODO: DELETE .clone()
      self.resize_gui(settings.get_live_rectangle().width(), ctx, &timed_run.clone()); // try to find a way to remove this clone

      ui.label(self.objective.get_id());
      ui.label(super::create_text("Not currently in run"));

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