

use egui::{Ui, Vec2};

use crate::{
  graphics::{
    create_text, live_parser::LiveParser, settings_window::SettingsWindow
  }, 
  logs::{
    location::LocationType, parser::ParserResult, token_parser::TokenParserT, tokenizer::Tokenizer
  }, 
  run::{objectives::run_objective::RunObjective, run_enum::RunEnum, timed_run::{GameRun, LevelRun}, traits::Run}, save_run::SaveManager
};

use super::{run_objective_reader::RunObjectiveReader, game_objective_reader::GameObjectiveReader, key_guesser::KeyGuesser, mapper::Mapper, run_renderer::RunRenderer};

pub struct LiveWindow<'a> {
  
  frame_counter: u8,
  run_counter: usize,
  parser: LiveParser,
  
  key_guesser: KeyGuesser<'a>,
  last_y_size: usize,

  level_run_reader: RunObjectiveReader,
  game_run_reader: GameObjectiveReader,
  game_run: Option<GameRun>,

}

impl<'a> Default for LiveWindow<'a> {
  fn default() -> Self {
    Self {
      frame_counter: Default::default(),
      run_counter: 0,
      parser: LiveParser::default(),
      key_guesser: KeyGuesser::default(),
      game_run_reader: GameObjectiveReader::default(),
      level_run_reader: RunObjectiveReader::default(),
      game_run: None,
      last_y_size: 0,
    }
  }
}


impl<'a> LiveWindow<'a> {

  /// return the last LevelRun stored in the logs.
  fn get_current_run(&self) -> Option<&LevelRun> {

    if let Some(run_parser) = self.parser.get_run_parser() {
      return Some(run_parser.into_result())
    } 

    self.parser.into_result().get_runs().last()

  }

  fn get_result(&self) -> &ParserResult {
    self.parser.into_result()
  }

  /// read the logs and update 
  /// 
  /// also saves the new runs to the save_manager.
  /// 
  /// beware this needs to be called 32 times for it to read logs once.
  fn read_logs(&mut self, save_manager: &mut SaveManager) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.parser.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      
      self.parser.parse_continously(tokens.into_iter());
    
      let runs = self.parser.into_result().get_runs();
      if runs.len() > self.run_counter {
        let to_save = &runs[self.run_counter..runs.len()];
        self.run_counter = runs.len();

        save_manager.save_multiple(to_save.iter().map(|v| RunEnum::Level(v.clone())).collect());
      }
    }

  }

  /// load the latest file in the logs and proceed with this file.
  pub fn load_file(&mut self, settings: &SettingsWindow) {
    self.parser.load_file(settings);
  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager, settings: &SettingsWindow, ctx: &egui::Context) {
    self.read_logs(save_manager);

    let mut y_size = 22;
    
    if settings.get_show_code_guess() { 
      y_size += self.key_guesser.render_key_guesser(ui, settings) + 6; 
      
      ui.separator();
    }
    if settings.get_show_game_splitter() {
      self.game_run = match self.game_run.take() {
        Some(run) => {
          let mut valid = true;

          if ui.button(create_text("Stop Game Run")).clicked() {
            valid = false;
          }

          y_size += 28 + RunRenderer::render_run(ui, &run, None, settings.get_game_splitter_length(), save_manager);

          ui.separator();
        
          match valid {
            true => Some(run),
            false => None,
          }
        },
        None => {
          let mut start = false;
          self.game_run_reader.show(ui);
          y_size += 50;

          if ui.button(create_text("Start Game Run")).clicked() {
            start = true;
          }

          ui.separator();

          match start {
            true => Some(GameRun::new(self.game_run_reader.get_objective().clone())),
            false => None,
          }
        },
      }
    }
    if settings.get_show_warden_mapper() {
      y_size += 6 + Mapper::render_type(
        ui, 
        self.get_result(), 
        match settings.get_show_objective_items() {
          true => None,
          false => Some(LocationType::Key),
        }
      );

      ui.separator();
    }
    if settings.get_show_splitter() {
      self.level_run_reader.show(ui);

      y_size += 50;

      ui.separator();

      if let Some(objective) = self.get_current_run().map(|r| r.get_objective::<RunObjective>()).flatten() {
        self.level_run_reader.set_name(objective.level_name);
      }

      if let Some(current_run) = self.get_current_run() {
        y_size += RunRenderer::render_run(ui, current_run, Some(&self.level_run_reader.get_objective().to_string()), settings.get_splitter_length(), save_manager);
      } 
    }

    if self.last_y_size != y_size {
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 { 
        x: settings.get_live_rectangle().width(), 
        y: y_size as f32 
      }));
      
      self.last_y_size = y_size;
    }
  }
}