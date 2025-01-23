use egui::{Color32, Ui};
use strum::IntoEnumIterator;

use crate::{game_runs::{levels::GameRunRundown, objectives::GameRunObjective, run_manager::RunManager}, logs::{token_parser::TokenParserT, tokenizer::Tokenizer}, objective_data::ObjectiveData, save_run::SaveManager, time::Time, timed_run::{GetByObjective, TimedRun}};

use super::{live_parser::LiveParser, settings_window::SettingsWindow};


#[derive(Default)]
pub struct FullGameWindow {

  run_manager: Option<RunManager>,
  frame_counter: u8,
  runs_count: usize,

  objective: ObjectiveData,

  parser: LiveParser,

  best_run: Option<(Vec<TimedRun>, Time, bool)>,
  player_count: String,

}

impl FullGameWindow {

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager) {

    self.frame_counter += 1;
    if self.frame_counter == 32 {
      self.frame_counter = 0;
      let new_lines = self.parser.load_text();

      let tokens = Tokenizer::tokenize(&new_lines);
      self.parser.parse_continously(tokens.into_iter());
      let vecs = self.parser.into_result();
      
      while self.runs_count < vecs.get_runs().len() {
        let mut run = vecs.get_runs().get(self.runs_count).unwrap().clone();
        run.objective_data.early_drop = self.objective.early_drop;
        run.objective_data.glitched = self.objective.glitched;
        run.objective_data.secondary = self.objective.secondary.max(run.objective_data.secondary);
        run.objective_data.overload = self.objective.overload.max(run.objective_data.overload);
        run.objective_data.player_count = self.objective.player_count;
        save_manager.save(run.clone());
        self.run_manager.as_mut().map(|rm| rm.finished_level(run));
        self.runs_count += 1;
      }
    }

    if let Some(run_manager) = &mut self.run_manager {
      let compared_run = save_manager.get_best_game_run(run_manager.get_objective());

      ui.horizontal(|ui| {
        egui::ComboBox::from_label("")
          .selected_text(super::create_text(format!("{}", run_manager.get_game_rundown())))
          .height(256.0)
          .show_ui(ui, |ui| {
            
            for key in GameRunRundown::iter() {
              if ui.selectable_value(run_manager.get_mut_game_rundown(), key.clone(), super::create_text(key.to_string())).clicked() {
                let game_rundown = run_manager.get_game_rundown().clone();
                let game_objective = run_manager.get_game_objective().clone();
                let player_count = run_manager.get_player_count();
                self.player_count = player_count.to_string();

                *run_manager = RunManager::new(game_objective, game_rundown, player_count);
                for run in self.parser.into_result().get_runs() {
                  run_manager.finished_level(run.clone());
                }
              }
            }

          });
          
        egui::ComboBox::from_label(" ")
        .selected_text(super::create_text(format!("{}", run_manager.get_game_objective())))
        .height(256.0)
        .show_ui(ui, |ui| {
          
          for key in GameRunObjective::iter() {
            if ui.selectable_value(run_manager.get_mut_game_objective(), key.clone(), super::create_text(key.to_string())).clicked() {
              let game_rundown = run_manager.get_game_rundown().clone();
              let game_objective = run_manager.get_game_objective().clone();
              let player_count = run_manager.get_player_count();
              self.player_count = player_count.to_string();

              *run_manager = RunManager::new(game_objective, game_rundown, player_count);
              for run in self.parser.into_result().get_runs() {
                run_manager.finished_level(run.clone());
              }
            }
          }

        });

        if ui.text_edit_singleline(&mut self.player_count)
          .changed() {
          let game_rundown = run_manager.get_game_rundown().clone();
          let game_objective = run_manager.get_game_objective().clone();
          let player_count = self.player_count.parse::<u8>().ok();

          if let Some(player_count) = player_count {
            *run_manager = RunManager::new(game_objective, game_rundown, player_count);
            for run in self.parser.into_result().get_runs() {
              run_manager.finished_level(run.clone());
            }
          }
        }

      });
      ui.separator();

      ui.label(super::create_text(format!("Level count left: {}", run_manager.len())));

      if let Some(next) = run_manager.get_next_level() {
        ui.label(super::create_text(format!("Next level: {}", next)));
      }

      ui.separator();

      ui.horizontal(|ui| {
        ui.label(super::create_text("LEVEL "));
        ui.label(super::create_text("YOUR TIME   "));
        ui.label(super::create_text("DIFFERENCE  "));
      });

      for run in run_manager.get_last_n_runs(6) {
        ui.horizontal(|ui| {
          
          ui.label(super::create_text(format!("{} ", &run.objective_data.level_name)));
          ui.label(super::create_text(run.get_time().to_string()));
          if let Some(compared_run) = compared_run.map(|cr| cr.0.get_by_objective(&run.objective_data)).flatten() {
            let (color, time) = match run.get_time().is_smaller_than(&compared_run.get_time()) {
              true => (Color32::GREEN, compared_run.get_time().sub(&run.get_time())),
              false => (Color32::RED, run.get_time().sub(&compared_run.get_time())),
            };
          
            ui.label(super::create_text(time.to_string()).color(color));
          };
        });
      }
    }
  }

  pub fn start_run(&mut self, missions: GameRunRundown, objectives: GameRunObjective, player_count: u8) {
    
    if self.run_manager.is_none() {
      self.run_manager = Some(RunManager::new(objectives, missions, player_count));
      self.player_count = "1".to_owned();
    }

  }

  pub fn end_run(&mut self) {

    self.run_manager = None;

  }

  pub fn load_file(&mut self, ctx: &egui::Context, settings: &SettingsWindow, save_manager: &mut SaveManager) {
    self.parser.load_file(settings);
    let empty_str = "".to_owned();
    self.best_run = save_manager.get_best_game_run(
      self.run_manager.as_ref().map(|rm| rm.get_objective()).unwrap_or(&empty_str)
    ).cloned();

    let mut rect = settings.get_live_rectangle();
    rect.set_height(260.0);
    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(rect.size()));
  }

}
