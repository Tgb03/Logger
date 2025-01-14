use std::{fs::{self, File}, io::{BufRead, BufReader, Seek}, path::Path};

use egui::{Color32, FontId, Ui, Vec2};

use crate::{key_guess::KeyGuess, logs::{parser::Parser, token_parser::TokenParserT, tokenizer::Tokenizer}, objective_data::ObjectiveData, save_run::SaveManager, time::Time, timed_run::TimedRun};

use super::settings_window::SettingsWindow;

pub struct LiveWindow<'a> {
  
  file: Option<File>,
  frame_counter: u8,
  last_position: u64,
  file_name: Option<String>,
  objective: ObjectiveData,
  player_input_string: String,
  key_guess_input_string: String, 

  runs_count: usize,
  ui_y_size: usize,
  
  parser: Parser,
  key_guesser: KeyGuess<'a>,

}

impl<'a> Default for LiveWindow<'a> {
  fn default() -> Self {
    Self { 
      file: Default::default(), 
      frame_counter: Default::default(), 
      last_position: Default::default(), 
      file_name: Default::default(), 
      objective: Default::default(), 
      player_input_string: Default::default(), 
      key_guess_input_string: "----".to_owned(), 
      runs_count: Default::default(), 
      ui_y_size: Default::default(), 
      parser: Default::default(), 
      key_guesser: Default::default() }
  }
}


impl<'a> LiveWindow<'a> {

  fn resize_gui(&mut self, width: f32, ctx: &egui::Context, timed_run: &TimedRun, key_guesser_lines: usize) {
    let times = timed_run.get_times();
    let mapper_size = match self.parser.get_generation_parser() {
      Some(parser) => parser.into_result().len(),
      None => self.parser.into_result().get_locations().len(),
    };
    let end_len = 
      times.len() 
      + match timed_run.get_time() != Time::default() {
        true => 1,
        false => 0,
      }
      + mapper_size
      + key_guesser_lines;

    if end_len != self.ui_y_size {
      self.ui_y_size = end_len;
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 { x: width, y: 122.0 + 22.0 * end_len as f32 }));
    }
  }

  fn render_key_guesser(&mut self, ui: &mut Ui, line_count: usize) -> usize {

    ui.horizontal(|ui| {
      
      ui.label(super::create_text("Code: "));
      if ui.add(egui::TextEdit::singleline(&mut self.key_guess_input_string)
        .desired_width(32.0)
        .font(FontId::new(12.0, egui::FontFamily::Name("jetbrains_mono".into())))
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          self.key_guesser = KeyGuess::default();

          for (id, key) in self.key_guess_input_string.bytes().enumerate() {
            if 97 <= key && key <= 122 {
              self.key_guesser.add_key(id as u8, key);
            } 

            if 65 <= key && key <= 90 {
              self.key_guesser.add_key(id as u8, key + 32);
            }
          }
        };
      ui.label(super::create_text(format!("Count: {}", self.key_guesser.len())));

    });

    let list = self.key_guesser.get_list();
    let len = self.key_guesser.len();

    for line in 0..line_count {
      if len <= line * 6 {
        return line + 1
      }

      ui.horizontal(|ui| {

        for i in 0..6 {
          if len == i + line * 6 {
            break
          }

          ui.label(super::create_text(format!("{}", std::str::from_utf8(&list[i + line * 6][0..4]).unwrap().to_ascii_uppercase())));
        }

      });
    }

    1 + line_count
  }

  fn render_mapper(&self, ui: &mut Ui) {
    let locations = match self.parser.get_generation_parser() {
      Some(parser) => parser.into_result(),
      None => self.parser.into_result().get_locations(),
    };

    ui.vertical(|ui| {
      for location in locations {
        ui.label(super::create_text::<String>(location.into()));
      }    
    });
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

    let mut key_guesser_lines = 0;
    if settings.get_show_code_guess() {
      /*
      // basically this doesn't work cause egui and keyboard input is mentally challenged.
      if ctx.input(|i| {
        i.modifiers.shift
      }) {
        if let (Some(id), Some(value)) = ctx.input(|i| {
          let mut id: Option<u8> = None;

          if i.key_down(egui::Key::Backtick) { self.key_guesser = KeyGuess::default(); }
          if i.key_down(egui::Key::Num1) { id = Some(0) }
          if i.key_down(egui::Key::Num2) { id = Some(1) }
          if i.key_down(egui::Key::Num3) { id = Some(2) }
          if i.key_down(egui::Key::Num4) { id = Some(3) }

          let mut value: Option<u8> = None;

          for key_id in 97u8..122u8 {
            if i.key_pressed(egui::Key::from_name(std::str::from_utf8(&[key_id]).unwrap()).unwrap()) {
              value = Some(key_id);
            }
          }

          return (id, value)
        }) {

          self.key_guesser.add_key(id, value);
        
        }
      }
      */

      ui.separator();
      key_guesser_lines = self.render_key_guesser(ui, settings.get_code_guess_line_count());
    }

    if settings.get_show_warden_mapper() {
      ui.separator();
      self.render_mapper(ui);
      
      if self.parser.get_run_parser().is_none() {
        self.resize_gui(
          settings.get_live_rectangle().width(), 
          ctx, 
          &self.parser.into_result().get_runs().last().unwrap_or(&TimedRun::new("".to_owned())).clone(),
          key_guesser_lines
        ); // try to find a way to remove this clone
      }
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
      self.resize_gui(settings.get_live_rectangle().width(), ctx, &timed_run.clone(), key_guesser_lines); // try to find a way to remove this clone

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
      self.resize_gui(settings.get_live_rectangle().width(), ctx, &timed_run.clone(), key_guesser_lines); // try to find a way to remove this clone

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
      self.ui_y_size = 0;
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