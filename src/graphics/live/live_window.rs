

use egui::{Ui, Vec2};

use crate::{
  graphics::{
    create_text, live_parser::LiveParser, 
    settings_window::SettingsWindow
  }, 
  logs::{
    location::Location, 
    token_parser::TokenParserT, 
    tokenizer::{
      GenerationTokenizer, 
      GenericTokenizer, 
      RunTokenizer, 
      Tokenizer
    }
  }, 
  run::{
    objectives::run_objective::RunObjective, 
    run_enum::RunEnum, 
    timed_run::{
      GameRun, 
      LevelRun
    }, 
    traits::Run
  }, 
  save_run::SaveManager
};

use super::{run_objective_reader::RunObjectiveReader, game_objective_reader::GameObjectiveReader, key_guesser::KeyGuesser, mapper::Mapper, run_renderer::RunRenderer};

pub struct LiveWindow<'a> {
  
  frame_counter: u8,
  run_counter: usize,
  parser: LiveParser,
  mapper: Mapper,
  
  key_guesser: KeyGuesser<'a>,
  last_y_size: usize,

  level_run_reader: RunObjectiveReader,
  game_run_reader: GameObjectiveReader,
  game_run: Option<GameRun>,

  game_run_renderer: Option<RunRenderer>,
  level_run_renderer: Option<RunRenderer>,
  
  tokenizer: GenericTokenizer,

}

impl<'a> Default for LiveWindow<'a> {
  fn default() -> Self {
    Self {
      frame_counter: Default::default(),
      mapper: Default::default(),
      run_counter: 0,
      parser: LiveParser::default(),
      key_guesser: KeyGuesser::default(),
      game_run_reader: GameObjectiveReader::default(),
      level_run_reader: RunObjectiveReader::default(),
      level_run_renderer: None,
      game_run_renderer: None,
      game_run: None,
      last_y_size: 0,
      tokenizer: GenericTokenizer::default()
        .add_tokenizer(RunTokenizer)
        .add_tokenizer(GenerationTokenizer)
    }
  }
}


impl<'a> LiveWindow<'a> {

  /// return the last map locations in the logs or the current ones.
  /// 
  /// this function exists otherwise the logs are only 
  /// shown when the game starts.
  fn get_current_map(&self) -> Option<&Vec<Location>> {

    if let Some(gen_parser) = self.parser.get_generation_parser() {
      return Some(&gen_parser.into_result());
    }

    Some(self.parser.into_result().get_locations())
  }

  /// return the last LevelRun stored in the logs.
  fn get_current_run(&self) -> Option<&LevelRun> {

    if let Some(run_parser) = self.parser.get_run_parser() {
      return Some(run_parser.into_result())
    } 

    self.parser.into_result().get_runs().last()

  }

  pub fn start_watcher(&mut self, settings: &SettingsWindow) {
    self.parser.start_watcher(settings.get_logs_folder().clone());
    self.last_y_size = 0;
  }

  pub fn stop_watcher(&mut self) {
    self.parser.stop_watcher();
  }

  pub fn save_unsaved_forced(&mut self, save_manager: &mut SaveManager) {
    let runs = self.parser.into_result().get_runs();
    let to_save = &runs[self.run_counter..runs.len()];
    self.run_counter = runs.len();

    save_manager.save_multiple(to_save.iter().map(|v| RunEnum::Level(v.clone())).collect());
  }

  fn save_unsaved_runs(&mut self, save_manager: &mut SaveManager) {
    let runs = self.parser.into_result().get_runs();
    if self.parser.get_run_parser().is_some() && runs.len() > self.run_counter {
      self.save_unsaved_forced(save_manager);
    }
  }

  /// read the logs and update 
  /// 
  /// also saves the new runs to the save_manager.
  /// 
  /// beware this needs to be called 32 times for it to read logs once.
  fn read_logs(&mut self, save_manager: &mut SaveManager, settings: &SettingsWindow) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      if self.parser.load_file() {
        self.run_counter = 0;
      }
      let new_lines = self.parser.load_text();

      let tokens = self.tokenizer.tokenize(&new_lines);
      self.parser.parse_continously(tokens.into_iter());
    
      self.save_unsaved_runs(save_manager);
    }

    self.level_run_renderer = match (self.level_run_renderer.take(), self.get_current_run(), self.parser.get_run_parser().is_some()) {
        (None, Some(run), _) => Some(RunRenderer::new(
          run,
          settings.get_compare_to_record(), 
          settings.get_compare_to_theoretical(), 
          settings.get_splitter_length(), 
          save_manager)),
        (Some(_), Some(run), true) => Some(RunRenderer::new(
          run,
          settings.get_compare_to_record(), 
          settings.get_compare_to_theoretical(), 
          settings.get_splitter_length(), 
          save_manager)),
        (Some(renderer), Some(_), false) => Some(renderer),
        _ => None,
    }

  }

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager, settings: &SettingsWindow, ctx: &egui::Context) {
    self.read_logs(save_manager, settings);
    
    if let Some(objective) = self.get_current_run().map(|r| r.get_objective::<RunObjective>()).flatten() {
      self.level_run_reader.set_name(objective.level_name);
      self.level_run_reader.set_player_count(objective.player_count);
    }

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
            self.game_run_renderer = None;
          }

          self.game_run_renderer.as_mut().map(|r| r.update(&run, save_manager));
          y_size += 28 + self.game_run_renderer.as_ref().map(|r| r.render(ui)).unwrap_or_default();

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
            true => {
              let run = GameRun::new(self.game_run_reader.get_objective().clone());

              self.game_run_renderer = Some(RunRenderer::new(
                &run, 
                settings.get_compare_to_record(), 
                settings.get_compare_to_theoretical(), 
                settings.get_game_splitter_length(), 
                save_manager
              ));

              Some(run)
            },
            false => None,
          }
        },
      }
    }
    if settings.get_show_run_counter() {
      y_size += 27;

      ui.horizontal(|ui| {
        let result = self.parser.into_result();

        ui.label(create_text(format!("Run Counter: {}", result.get_counter())));
        ui.label(create_text(format!("Unique: {}", result.get_set().len())));
      });

      ui.separator();
    }
    if settings.get_show_warden_mapper() {
      let level_name = &self.level_run_reader.get_objective().to_string();
      self.mapper.load_level_info(level_name);

      y_size += 6 + Mapper::render_type(
        ui, 
        self.get_current_map().unwrap_or(&Vec::new()), 
        settings.get_show_objective_items(),
        self.mapper.get_color_info(level_name)
      );

      self.mapper.render_error(ui, level_name);

      ui.separator();
    }
    if settings.get_show_splitter() {
      self.level_run_reader.show(ui);

      ui.separator();

      y_size += 32 + self.level_run_renderer.as_ref().map(|r| r.render(ui)).unwrap_or_default();
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