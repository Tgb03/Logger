

use egui::{Color32, FontId, Ui, Vec2};


use crate::{key_guess::KeyGuess, logs::{token_parser::TokenParserT, tokenizer::Tokenizer}, run::{objectives::{game_objective::GameObjective, objective_enum::ObjectiveEnum, run_objective::RunObjective}, run_enum::RunEnum, time::Time, timed_run::LevelRun, traits::Run}, save_run::SaveManager};

use super::{live_parser::LiveParser, settings_window::SettingsWindow};

pub struct LiveWindow<'a> {
  
  frame_counter: u8,
  objective_level: RunObjective,
  objective_game: GameObjective,
  player_input_string: String,
  key_guess_input_string: String, 

  runs_count: usize,
  ui_y_size: usize,
  
  key_guesser: KeyGuess<'a>,
  parser: LiveParser,

}

impl<'a> Default for LiveWindow<'a> {
  fn default() -> Self {
    Self { 
      frame_counter: Default::default(),
      objective_level: Default::default(), 
      objective_game: Default::default(), 
      player_input_string: Default::default(), 
      key_guess_input_string: "----".to_owned(), 
      runs_count: Default::default(), 
      ui_y_size: Default::default(),
      key_guesser: Default::default(),
      parser: LiveParser::default(),
    }
  }
}


impl<'a> LiveWindow<'a> {

  fn get_current_run(&self) -> Option<&LevelRun> {

    if let Some(run_parser) = self.parser.get_run_parser() {
      return Some(run_parser.into_result())
    }

    if let Some(timed_run) =  self.parser.into_result().get_runs().last() {
      return Some(timed_run)
    }

    None

  }

  fn get_comparison_run<'b, T: Run>(&self, current_run: &T, save_manager: &'b SaveManager) -> Option<&'b RunEnum> {

    save_manager.get_best_run(
      &current_run.get_objective::<ObjectiveEnum>()?
    )

  }

  fn get_comparison_splits<'b, T: Run>(&self, current_run: &T, save_manager: &'b SaveManager) -> Option<&'b Vec<Time>> {

    save_manager.get_best_splits(
      &current_run.get_objective::<ObjectiveEnum>()?
    )

  }

  fn resize_gui(&mut self, width: f32, ctx: &egui::Context, timed_run_size: usize, mapper_size: usize, key_guesser_lines: usize) {

    let end_len = timed_run_size + mapper_size + key_guesser_lines;

    if end_len != self.ui_y_size {
      self.ui_y_size = end_len;
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 { x: width, y: 122.0 + 22.0 * end_len as f32 }));
    }

  }

  fn render_key_guesser(&mut self, ui: &mut Ui, line_count: usize, line_width: usize) -> usize {

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
      if len <= line * line_width {
        return line + 1
      }

      ui.horizontal(|ui| {

        for i in 0..line_width {
          if len == i + line * line_width {
            break
          }

          ui.label(super::create_text(format!("{}", std::str::from_utf8(&list[i + line * line_width][0..4]).unwrap().to_ascii_uppercase())));
        }

      });
    }

    1 + line_count
  }

  fn render_mapper(&self, ui: &mut Ui, settings: &SettingsWindow) -> usize {
    let locations = match self.parser.get_generation_parser() {
      Some(parser) => parser.into_result(),
      None => self.parser.into_result().get_locations(),
    };

    let mut len = 0;
    ui.vertical(|ui| {
      for location in locations {
        if settings.get_show_objective_items() || !location.has_type(crate::logs::location::LocationType::Objective) {
          ui.label(super::create_text::<String>(location.into()));
          len += 1;
        }
      }    
    });

    len
  }

  fn render_timed_run<T, Y>(&self, ui: &mut Ui, timed_run: &T, compared_run: Option<&Y>, compared_splits: Option<&Vec<Time>>) -> usize
  where 
    T: Run,
    Y: Run {

    let times = timed_run.get_times();
    let mut result = timed_run.len();
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

            if let Some(compared_time) = compared_splits.get(id) {
              let (time_diff, color) = match split.is_greater_than(compared_time) {
                true => (split.sub(&compared_splits[id]), Color32::RED),
                false => (compared_splits[id].sub(&split), Color32::GREEN),
              };

              ui.colored_label(color, super::create_text(time_diff.to_string_no_hours()));
            }
          }
        });
      }

      ui.horizontal(|ui| {
        let final_time = timed_run.get_time();

        if final_time != Time::default() && !timed_run.is_win() && self.parser.get_run_parser().is_some_and(|p| !p.is_done()) {
          ui.label(super::create_text(final_time.to_string()));
          result += 1;

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

    result
  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager, settings: &SettingsWindow, ctx: &egui::Context) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.parser.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      
      self.parser.parse_continously(tokens.into_iter());
      
      let vecs = self.parser.into_result();
      while self.runs_count < vecs.get_runs().len() {
        let mut run = vecs.get_runs().get(self.runs_count).unwrap().clone();
        
        let mut objective = run.get_objective::<RunObjective>()
          .unwrap()
          .with_secondary(self.objective_level.secondary)
          .with_overload(self.objective_level.overload)
          .with_glitched(self.objective_level.glitched)
          .with_early_drop(self.objective_level.early_drop);
        
        if let Ok(id) = self.player_input_string.parse::<u8>() {
          objective = objective.with_player_count(id);
        }

        run.set_objective(&objective);
        save_manager.save(RunEnum::Level(run));
        self.runs_count += 1;
      }
    }

    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective_level.secondary, super::create_text("Sec"));
      ui.checkbox(&mut self.objective_level.overload, super::create_text("Ovrld"));
      ui.label(super::create_text("Ps:"));
      ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
        .char_limit(2)
        .background_color(Color32::from_rgba_unmultiplied(32, 32, 32, 32)));
    });
    
    ui.horizontal(|ui| {
      ui.checkbox(&mut self.objective_level.glitched, super::create_text("Glitch"));
      ui.checkbox(&mut self.objective_level.early_drop, super::create_text("E-Drop"));
    });

    self.objective_level.level_name = self.objective_level.level_name.to_uppercase();
    if let Some(count) = self.player_input_string.parse::<u8>().ok() {
      self.objective_level.player_count = count;
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
      key_guesser_lines = self.render_key_guesser(ui, settings.get_code_guess_line_count(), settings.get_code_guess_line_width());
    }

    let mut mapper_size = 0;
    if settings.get_show_warden_mapper() {
      ui.separator();
      mapper_size = self.render_mapper(ui, settings);
    }

    ui.separator();

    let mut splitter_size = 0;
    if settings.get_show_splitter() {
      if let Some(current_run) = self.get_current_run() {
        
        let comparison_run = self.get_comparison_run(&RunEnum::Level(current_run.clone()), save_manager);
        let comparison_splits = self.get_comparison_splits(&RunEnum::Level(current_run.clone()), save_manager);

        splitter_size = self.render_timed_run(ui, current_run, comparison_run, comparison_splits);

      }
    }

    self.resize_gui(settings.get_live_rectangle().width(), ctx, splitter_size, mapper_size, key_guesser_lines);
  }

  pub fn load_file(&mut self, settings: &SettingsWindow) {
    self.parser.load_file(settings);
  }
}